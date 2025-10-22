use super::cooldown_service::CooldownService;
use crate::commands::template_service::TemplateService;
use crate::AppState;
use regex::Regex;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;

// ChatMessage

#[allow(dead_code)]
#[derive(Clone, Copy,  Ord, PartialOrd, PartialEq, Eq)]
pub enum TwitchUserPermission {
    Everyone,
    PredictionsBlue,
    PredictionsPink,
    Subscriber,
    Artist,
    Founder,
    Vip,
    Moderator,
    Broadcaster,
    Owner,
    System,
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
pub type TriggerCallback = fn(Arc<AppState>, Box<TriggerId>, ChatMessage) -> Pin<Box<dyn Future<Output=()>>>;

pub enum ChatCooldown {
    SECONDS(u32),
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
    match TemplateService::get_template_by_trigger_id(&app_state.prod_db, trigger_id.as_ref()).await {
        Err(_e) => {
            // log
            // logger.debug("Executing text command {}", commandId);
        }
        Ok(None) => {
            // log
            //     logger.error("Could not find template id for command id {}", commandId);
        }
        Ok(Some(t)) => app_state.twitch_service.send_raw_template(t.template.as_ref(), HashMap::new())
    }
});

// Service

pub struct CommandExecutorService {
    triggers: Vec<CommandTrigger>,
    cooldown_service: CooldownService,
}


impl CommandExecutorService {
    #[allow(dead_code)]
    pub async fn receive_commands(&self, app_state: Arc<AppState>, mut receiver: Receiver<ChatMessage>){
        loop {
            let message = match receiver.recv().await {
                Ok(m) => m,
                Err(RecvError::Closed) => return,
                Err(RecvError::Lagged(_s)) => {
                    // log skip
                    continue;
                }
            };
            for trigger in self.triggers.iter() {
                self.execute_trigger_if_matching(app_state.clone(), trigger, message.clone()).await
            }
        }
    }

    async fn execute_trigger_if_matching(&self, app_state: Arc<AppState>, trigger: &CommandTrigger, chat_message: ChatMessage) {
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

