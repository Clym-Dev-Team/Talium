use super::command_executor_service::{ChatCooldown, ChatMessage, TriggerId, TwitchUserId};
use std::collections::BTreeMap;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Mutex, MutexGuard};
use std::time::Instant;

#[derive(Default)]
pub struct CooldownService {
    global_message_counter: AtomicU64,
    state: Mutex<InnerState>
}

type SequentialMessageId = u64;

//TODO test if we can replace this lock and inner solution with concurrent: dashmap

#[derive(Default)]
struct InnerState {
    global_message: BTreeMap<Box<TriggerId>, SequentialMessageId>,
    global_seconds: BTreeMap<Box<TriggerId>, Instant>,
    user_message: BTreeMap<(Box<TriggerId>, Box<TwitchUserId>), SequentialMessageId>,
    user_seconds: BTreeMap<(Box<TriggerId>, Box<TwitchUserId>), Instant>,
}

pub enum RejectionReason {
    GlobalCooldown,
    UserCooldown
}

impl CooldownService {
    /// Checks and updates the cooldowns in one operation to reduce blocking of the locks
    /// ## **Only call this function once for each message**
    /// No deduplication is performed, and the same message will be interpreted as two new messages.
    /// This will lead to errors in the behavior of the cooldowns
    pub fn check_update_cooldown(
        &self,
        chat_message: &ChatMessage,
        trigger_id: &TriggerId,
        user_cooldown: &ChatCooldown,
        global_cooldown: &ChatCooldown,
    ) -> Option<RejectionReason> {
        let mut l = self.lock();
        let sequential_message_id = self.global_message_counter.fetch_add(1, Relaxed) + 1;
        match global_cooldown {
            ChatCooldown::MESSAGES(amount) => {
                let last_sequential_message_id = l.global_message.get(trigger_id);
                if last_sequential_message_id.is_some() {
                    let delta = sequential_message_id - last_sequential_message_id.unwrap();
                    if delta <= *amount as u64 {
                        return Some(RejectionReason::GlobalCooldown);
                    }
                }
                l.global_message.insert(Box::from(trigger_id), sequential_message_id);
            }
            ChatCooldown::SECONDS(seconds) => {
                let last_instant = l.global_seconds.get(trigger_id);
                if last_instant.is_some() {
                    let seconds_between = last_instant.unwrap().duration_since(chat_message.send_at).as_secs();
                    if seconds_between <= *seconds as u64 {
                        return Some(RejectionReason::GlobalCooldown);
                    }
                }
                l.global_seconds.insert(Box::from(trigger_id), chat_message.send_at);
            }
        };
        match user_cooldown {
            ChatCooldown::MESSAGES(amount) => {
                let key = (Box::from(trigger_id), chat_message.user.id.clone());
                let last_sequential_message_id = l.user_message.get(&key);
                if last_sequential_message_id.is_some() {
                    let delta = sequential_message_id - last_sequential_message_id.unwrap();
                    if delta <= *amount as u64 {
                        return Some(RejectionReason::UserCooldown);
                    }
                }
                l.user_message.insert(key, sequential_message_id);
            }
            ChatCooldown::SECONDS(seconds) => {
                let key = (Box::from(trigger_id), chat_message.user.id.clone());
                let last_instant = l.user_seconds.get(&key);
                if last_instant.is_some() {
                    let seconds_between = last_instant.unwrap().duration_since(chat_message.send_at).as_secs();
                    if seconds_between <= *seconds as u64 {
                        return Some(RejectionReason::UserCooldown);
                    }
                }
                l.user_seconds.insert(key, chat_message.send_at);
            }
        };
        None
    }

    fn lock(&self) -> MutexGuard<'_, InnerState> {
        match self.state.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                self.state.clear_poison();
                let mut wg = poisoned.into_inner();
                *wg = InnerState::default();
                wg
            },
        }
    }
}
