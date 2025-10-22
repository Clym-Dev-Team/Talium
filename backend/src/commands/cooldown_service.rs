use super::command_executor_service::{ChatCooldown, ChatMessage, TriggerId, TwitchUserId};
use std::collections::BTreeMap;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::Instant;

#[derive(Default)]
pub struct CooldownService {
    global_message_counter: AtomicU64,
    state: RwLock<InnerState>
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
        let mut wg = self.write_guard();
        let sequential_message_id = self.global_message_counter.fetch_add(1, Relaxed) + 1;
        match global_cooldown {
            ChatCooldown::MESSAGES(amount) => {
                let last_sequential_message_id = wg.global_message.get(trigger_id);
                if last_sequential_message_id.is_some() {
                    let delta = sequential_message_id - last_sequential_message_id.unwrap();
                    if delta <= *amount as u64 {
                        return Some(RejectionReason::GlobalCooldown);
                    }
                }
                wg.global_message.insert(Box::from(trigger_id), sequential_message_id);
            }
            ChatCooldown::SECONDS(seconds) => {
                let last_instant = wg.global_seconds.get(trigger_id);
                if last_instant.is_some() {
                    let seconds_between = last_instant.unwrap().duration_since(chat_message.send_at).as_secs();
                    if seconds_between <= *seconds as u64 {
                        return Some(RejectionReason::GlobalCooldown);
                    }
                }
                wg.global_seconds.insert(Box::from(trigger_id), chat_message.send_at);
            }
        };
        match user_cooldown {
            ChatCooldown::MESSAGES(amount) => {
                let key = (Box::from(trigger_id), chat_message.user.id.clone());
                let last_sequential_message_id = wg.user_message.get(&key);
                if last_sequential_message_id.is_some() {
                    let delta = sequential_message_id - last_sequential_message_id.unwrap();
                    if delta <= *amount as u64 {
                        return Some(RejectionReason::UserCooldown);
                    }
                }
                wg.user_message.insert(key, sequential_message_id);
            }
            ChatCooldown::SECONDS(seconds) => {
                let key = (Box::from(trigger_id), chat_message.user.id.clone());
                let last_instant = wg.user_seconds.get(&key);
                if last_instant.is_some() {
                    let seconds_between = last_instant.unwrap().duration_since(chat_message.send_at).as_secs();
                    if seconds_between <= *seconds as u64 {
                        return Some(RejectionReason::UserCooldown);
                    }
                }
                wg.user_seconds.insert(key, chat_message.send_at);
            }
        };
        None
    }

    fn read_guard(&self) -> RwLockReadGuard<InnerState> {
        match self.state.read() {
            Ok(guard) => guard,
            Err(poisoned) => {
                self.state.clear_poison();
                drop(poisoned);
                let mut wg = self.state.write().unwrap();
                *wg = InnerState::default();
                self.state.read().unwrap()
            },
        }
    }

    fn write_guard(&self) -> RwLockWriteGuard<InnerState> {
        match self.state.write() {
            Ok(guard) => guard,
            Err(poisoned) => {
                self.state.clear_poison();
                drop(poisoned);
                let mut wg = self.state.write().unwrap();
                *wg = InnerState::default();
                wg
            },
        }
    }

}
