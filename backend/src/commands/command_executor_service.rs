use super::cooldown_service::CooldownService;
use crate::commands::command_controller::{Command, CooldownType, MessagePattern};
use crate::commands::template_service::{StringTemplate, TemplateService};
use crate::db::ProdDB;
use crate::state::FullState;
use num_derive::FromPrimitive;
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;
use log::{debug, error, trace, warn};
use tokio::sync::RwLock;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Ord, PartialOrd, PartialEq, Eq, Deserialize, Serialize, FromPrimitive, Type)]
#[repr(u8)]
pub enum TwitchUserPermission {
    Everyone = 0,
    PredictionsBlue = 1,
    PredictionsPink = 2,
    Subscriber = 3,
    Artist = 4,
    Founder = 5,
    Vip = 6,
    Moderator = 7,
    Broadcaster = 8,
    Owner = 9,
    System = 10,
}

pub type TwitchUserId = str;

#[allow(dead_code)]
#[derive(Clone)]
pub struct TwitchUser {
    pub id: Box<TwitchUserId>,
    pub name: Box<str>,
    pub permission: TwitchUserPermission,
    pub subscriber_months: u16,
    pub subscription_tier: u16,
}

// official id, uuid
pub type TwitchMessageId = Box<str>;

#[derive(Clone)]
pub struct ChatMessage {
    pub message_id: TwitchMessageId,
    pub message: Box<str>,
    pub user: TwitchUser,
    pub get_custom_reward_id: Option<Box<str>>,
    pub reply_to_message_id: Option<Box<str>>,
    pub channel_id: Box<str>,
    pub received_at: Instant,
}

// Triggers
pub type TriggerId = str;
pub type TriggerCallback = fn(Arc<FullState>, Box<TriggerId>, ChatMessage) -> Pin<Box<dyn Future<Output=()> + Send>>;

#[derive(Deserialize, Serialize)]
pub enum ChatCooldown {
    SECONDS(u16),
    MESSAGES(u16)
}

pub struct CommandWithRegex<'a> {
    pub id: &'a str,
    pub description: &'a str,
    pub patterns: Vec<Regex>,
    pub permission: TwitchUserPermission,
    pub is_auto_generated: bool,
    pub global_cooldown_amount: u16,
    pub global_cooldown_type: CooldownType,
    pub user_cooldown_amount: u16,
    pub user_cooldown_type: CooldownType,
    pub template: Option<&'a StringTemplate>,
}

impl<'a> TryFrom<&'a Command> for CommandWithRegex<'a> {
    type Error = regex::Error;

    fn try_from(value: &'a Command) -> Result<Self, Self::Error> {
        Ok(CommandWithRegex {
            patterns: CommandExecutorService::convert_patterns(value.patterns.as_ref())?,
            permission: value.permission,
            global_cooldown_amount: value.global_cooldown_amount,
            global_cooldown_type: value.global_cooldown_type,
            id: value.id.as_str(),
            template: value.template.as_ref(),
            is_auto_generated: value.is_auto_generated,
            description: value.description.as_str(),
            user_cooldown_amount: value.user_cooldown_amount,
            user_cooldown_type: value.user_cooldown_type,
        })
    }
}

pub struct CommandTrigger {
    pub id: Box<TriggerId>,
    pub patterns: Vec<Regex>,
    pub permission: TwitchUserPermission,
    pub user_cooldown: ChatCooldown,
    pub global_cooldown: ChatCooldown,
    pub callback: TriggerCallback,
}

static TEXT_COMMAND_CALLBACK: TriggerCallback = |app_state, trigger_id, _chat_message| Box::pin(async move {
    match TemplateService::get_template_by_trigger_id(&app_state.l1.prod_db, trigger_id.as_ref()).await {
        Err(e) => error!("Could not fetch template for command {} from db: {:?}", trigger_id, e),
        Ok(None) => warn!("Could not find template id for command id {}", trigger_id),
        Ok(Some(t)) => {
            debug!("Executing text command {}", trigger_id);
            app_state.l2.twitch_service.send_raw_template(t.template.as_ref(), HashMap::new())
        }
    }
});

pub struct CommandExecutorService {
    triggers: RwLock<Vec<CommandTrigger>>,
    cooldown_service: CooldownService,
}

impl CommandExecutorService {
    pub(crate) async fn new(_db: &ProdDB) -> CommandExecutorService {
        todo!("get commands from db, and also return Result here")
    }

    pub(crate) async fn remove_command(&self, command_id: &TriggerId) {
        self.triggers.write().await.retain(|c| c.id.as_ref() != command_id);
    }

    pub(crate) async fn upsert_command(&self, command: CommandWithRegex<'_>) {
        self.remove_command(command.id.as_ref()).await;
        let global_cooldown = match command.global_cooldown_type {
            CooldownType::SECONDS => ChatCooldown::SECONDS(command.global_cooldown_amount),
            CooldownType::MESSAGES => ChatCooldown::MESSAGES(command.global_cooldown_amount)
        };
        let user_cooldown = match command.user_cooldown_type {
            CooldownType::SECONDS => ChatCooldown::SECONDS(command.user_cooldown_amount),
            CooldownType::MESSAGES => ChatCooldown::MESSAGES(command.user_cooldown_amount)
        };
        self.triggers.write().await.push(CommandTrigger {
            id: Box::from(command.id),
            global_cooldown,
            user_cooldown,
            permission: command.permission,
            patterns: command.patterns,
            callback: TEXT_COMMAND_CALLBACK
        });
    }

    pub(crate) async fn refresh_patterns(&self, _prod_db: &ProdDB, _trigger_id: &TriggerId) {
        todo!()
    }

    fn convert_patterns(patterns: &[MessagePattern]) -> Result<Vec<Regex>, regex::Error> {
        patterns.iter()
            .filter(|x| x.is_enabled)
            .map(|x1| {
                match x1.is_regex {
                    true => Regex::new(x1.pattern.as_str()),
                    false => RegexBuilder::new(format!("^{}(?: |$).*", &x1.pattern).as_str())
                        .case_insensitive(true)
                        .build(),
                }
            })
            .collect()
    }
}

impl  CommandExecutorService {
    pub async fn process_chat_message(app_state: Arc<FullState>, message: ChatMessage) {
        for trigger in app_state.l1.command_executor_service.triggers.read().await.iter() {
            app_state.l1.command_executor_service.execute_trigger_if_matching(app_state.clone(), trigger, message.clone()).await
        }
    }

    async fn execute_trigger_if_matching(&self, app_state: Arc<FullState>, trigger: &CommandTrigger, chat_message: ChatMessage) {
        if chat_message.user.permission < trigger.permission {
            trace!("User {} with perm: {:?}, missing {:?} permission for command {}", chat_message.user.name, chat_message.user.permission, trigger.permission, trigger.id);
            return;
        }

        if !trigger.patterns.iter().any(|r| r.is_match(&chat_message.message)) {
            return;
        }

        let cooldown_res = self.cooldown_service.check_update_cooldown(&chat_message, trigger.id.as_ref(), &trigger.user_cooldown, &trigger.global_cooldown);
        if cooldown_res.is_some() {
            trace!("Call to command {} from {} rejected because of global cooldowns", trigger.id, chat_message.user.name);
            trace!("Call to command {} from {} rejected because of user cooldowns", trigger.id, chat_message.user.name);
            return;
        }

        (trigger.callback)(app_state, trigger.id.clone(), chat_message).await;
    }
}

