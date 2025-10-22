use super::cooldown_service::CooldownService;
use crate::AppState;
use regex::Regex;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;
// ChatMessage

#[allow(dead_code)]
#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
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
    pub name: String,
    pub permission: TwitchUserPermission,
    pub subscriber_months: u16,
    pub subscription_tier: u16,
}

// official id, uuid
pub type TwitchMessageId = String;

#[derive(Clone)]
pub struct ChatMessage {
    pub message_id: TwitchMessageId,
    pub message: String,
    pub user: TwitchUser,
    pub is_highlighted_message: bool,
    pub is_skip_subs_mode_message: bool,
    pub is_designated_first_message: bool,
    pub is_user_introduction: bool,
    pub get_custom_reward_id: Option<String>,
    pub reply_to_message_id: Option<String>,
    pub channel_id: String,
    pub send_at: Instant,
}

// Triggers

pub type TriggerId = str;
pub type TriggerCallback = fn(&AppState, &TriggerId, &ChatMessage) -> ();

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
    pub callback: Box<TriggerCallback>,
}

static TEXT_COMMAND_CALLBACK: TriggerCallback = |_app_state , _trigger_id, _chat_message| {
    // logger.debug("Executing text command {}", commandId);
    // var template = templateService.getTemplateByCommandId(commandId);
    // if (template.isEmpty()) {
    //     logger.error("Could not find template id for command id {}", commandId);
    //     return;
    // }
    // TODO add message and other things to context, but currently we can't handle records
    // Out.Twitch.sendRawTemplate(template.get().template, null);
};

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
                self.execute_trigger_if_matching(app_state.deref(), trigger, &message)
            }
        }
    }

    fn execute_trigger_if_matching(&self, app_state: &AppState, trigger: &CommandTrigger, chat_message: &ChatMessage) {
        if chat_message.user.permission < trigger.permission {
            // logger.debug("User {} with {}, missing {} permission for command {}", message.user().name(), message.user().permission(), trigger.permission(), trigger.id());
            return;
        }

        if !trigger.patterns.iter().any(|r| r.is_match(&chat_message.message)) {
            return;
        }

        let cooldown_res = self.cooldown_service.check_update_cooldown(chat_message, trigger.id.as_ref(), &trigger.user_cooldown, &trigger.global_cooldown);
        if cooldown_res.is_some() {
            // logger.debug("Call to command {} from {} rejected because of global cooldowns", trigger.id(), message.user().name());
            // logger.debug("Call to command {} from {} rejected because of user cooldowns", trigger.id(), message.user().name());
            return;
        }

        (trigger.callback)(app_state, trigger.id.as_ref(), chat_message);
    }
}

