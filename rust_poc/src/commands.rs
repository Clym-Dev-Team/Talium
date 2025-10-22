use crate::AppState;
use regex::Regex;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;

// ChatMessage

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

#[derive(Clone)]
pub struct TwitchUser {
    pub id: String,
    pub name: String,
    pub permission: TwitchUserPermission,
    pub subscriber_months: u16,
    pub subscription_tier: u16,
}

#[derive(Clone)]
pub struct ChatMessage {
    pub message_id: String,
    pub user_message_index: u32,
    pub global_message_index: u32,
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

type TriggerId<'a> = &'a str;
type TriggerCallback = fn(&AppState, TriggerId, &ChatMessage) -> ();

pub enum CooldownType {
    SECONDS,
    MESSAGES
}

pub struct ChatCooldown {
    cooldown_type: CooldownType,
    amount: u16,
}

pub struct CommandTrigger {
    id: String,
    patterns: Vec<Regex>,
    permission: TwitchUserPermission,
    user_cooldown: ChatCooldown,
    global_cooldown: ChatCooldown,
    callback: Box<TriggerCallback>,
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

pub struct CommandService {
    triggers: Vec<CommandTrigger>,
}

struct ReceiverClosed;

impl CommandService {
    pub async fn receive_commands(&self, app_state: Arc<AppState>, mut receiver: Receiver<ChatMessage>) -> Result<(), ReceiverClosed> {
        loop {
            let message = match receiver.recv().await {
                Ok(m) => m,
                Err(RecvError::Closed) => return Err(ReceiverClosed),
                Err(RecvError::Lagged(s)) => {
                    // log skip
                    continue;
                }
            };
            for trigger in self.triggers.iter() {
                CommandService::execute_trigger_if_matching(app_state.deref(), trigger, &message)
            }
        }
        Ok(())
    }

    fn execute_trigger_if_matching(app_state: &AppState, trigger: &CommandTrigger, chat_message: &ChatMessage) {
        if chat_message.user.permission < trigger.permission {
            // logger.debug("User {} with {}, missing {} permission for command {}", message.user().name(), message.user().permission(), trigger.permission(), trigger.id());
            return;
        }

        if !trigger.patterns.iter().any(|r| r.is_match(&chat_message.message)) {
            return;
        }

        // if (inGlobalCooldown(message, trigger.id(), trigger.globalCooldown())) {
        //     logger.debug("Call to command {} from {} rejected because of global cooldowns", trigger.id(), message.user().name());
        //     return;
        // }
        // if (inUserCooldown(message, trigger.id(), trigger.userCooldown())) {
        //     logger.debug("Call to command {} from {} rejected because of user cooldowns", trigger.id(), message.user().name());
        //     return;
        // }
        // updateCooldownState(message, trigger.id(), trigger.globalCooldown(), trigger.userCooldown());

        (trigger.callback)(app_state, trigger.id.as_str(), chat_message);
    }
}

