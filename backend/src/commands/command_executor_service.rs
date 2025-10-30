use super::cooldown_service::CooldownService;
use crate::commands::command_controller::{Command, CooldownType, MessagePattern};
use crate::commands::template_service::TemplateService;
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
// ChatMessage

#[allow(dead_code)]
#[derive(Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Deserialize, Serialize, FromPrimitive, Type)]
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
    pub is_highlighted_message: bool,
    pub is_skip_subs_mode_message: bool,
    pub is_designated_first_message: bool,
    pub is_user_introduction: bool,
    pub get_custom_reward_id: Option<Box<str>>,
    pub reply_to_message_id: Option<Box<str>>,
    pub channel_id: Box<str>,
    pub send_at: Instant,
}

// Triggers

pub type TriggerId = str;
// pub type TriggerCallback = fn(&AppState, &TriggerId, &ChatMessage) ;
pub type TriggerCallback = fn(Arc<FullState>, Box<TriggerId>, ChatMessage) -> Pin<Box<dyn Future<Output=()>>>;

#[derive(Deserialize, Serialize)]
pub enum ChatCooldown {
    SECONDS(u16),
    MESSAGES(u16)
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
        Err(_e) => {
            // log
            // logger.debug("Executing text command {}", commandId);
        }
        Ok(None) => {
            // log
            //     logger.error("Could not find template id for command id {}", commandId);
        }
        Ok(Some(t)) => app_state.l2.twitch_service.send_raw_template(t.template.as_ref(), HashMap::new())
    }
});

// Service

//TODO we have a race condition here, if we first request the values from the database, and then return the service, the direct call for us to refresh might get lost.
// because we might not exist yet, but the modification in the db still goes through.
// Possible solutions:
//  - indicate to users of our service that we are being created, and make them wait for us to finish.
//  - move creation of out service into l1 state. The creation of this just requires the DB, the execution is difficult
//    we also can't just create an emtpy version of ourselves, and fill the rest in later, because then our users would try to modify non existing commands
#[derive(Default)]
pub struct CommandExecutorService {
    triggers: Vec<CommandTrigger>,
    cooldown_service: CooldownService,
}

impl CommandExecutorService {
    pub(crate) fn remove_command(&self, command_id: &TriggerId) {
        //TODO
        // self.triggers.retain(|c| c.id.as_ref() != command_id);
    }

    pub(crate) fn upsert_command(&self, command: &Command) -> Result<(), regex::Error> {
        self.remove_command(command.id.as_ref());
        let global_cooldown = match command.global_cooldown_type {
            CooldownType::SECONDS => ChatCooldown::SECONDS(command.global_cooldown_amount),
            CooldownType::MESSAGES => ChatCooldown::MESSAGES(command.global_cooldown_amount)
        };
        let user_cooldown = match command.user_cooldown_type {
            CooldownType::SECONDS => ChatCooldown::SECONDS(command.user_cooldown_amount),
            CooldownType::MESSAGES => ChatCooldown::MESSAGES(command.user_cooldown_amount)
        };
        //TODO 
        // self.triggers.push(CommandTrigger {
        //     id: command.id.clone().into_boxed_str(),
        //     global_cooldown,
        //     user_cooldown,
        //     permission: command.permission,
        //     patterns: Self::convert_patterns(command.patterns.as_ref())?,
        //     callback: TEXT_COMMAND_CALLBACK
        // });
        Ok(())
    }

    pub(crate) fn refresh_patterns(&self, prod_db: &ProdDB, trigger_id: &TriggerId) {
        todo!()
    }

    fn convert_patterns(patterns: &[MessagePattern]) -> Result<Vec<Regex>,regex::Error> {
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

    pub async fn process_chat_message(&self, app_state: Arc<FullState>, message: ChatMessage) {
        for trigger in self.triggers.iter() {
            self.execute_trigger_if_matching(app_state.clone(), trigger, message.clone()).await
        }
    }

    async fn execute_trigger_if_matching(&self, app_state: Arc<FullState>, trigger: &CommandTrigger, chat_message: ChatMessage) {
        if chat_message.user.permission < trigger.permission {
            // logger.debug("User {} with {}, missing {} permission for command {}", message.user().name(), message.user().permission(), trigger.permission(), trigger.id());
            return;
        }

        if !trigger.patterns.iter().any(|r| r.is_match(&chat_message.message)) {
            return;
        }

        let cooldown_res = self.cooldown_service.check_update_cooldown(&chat_message, trigger.id.as_ref(), &trigger.user_cooldown, &trigger.global_cooldown);
        if cooldown_res.is_some() {
            // logger.debug("Call to command {} from {} rejected because of global cooldowns", trigger.id(), message.user().name());
            // logger.debug("Call to command {} from {} rejected because of user cooldowns", trigger.id(), message.user().name());
            return;
        }

        (trigger.callback)(app_state, trigger.id.clone(), chat_message).await;
    }
}

