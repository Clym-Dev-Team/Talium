use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use twitch_highway::users::{User, UserAPI};
use twitch_highway::types::UserId;
use tokio::sync::broadcast::Sender;
use twitch_highway::eventsub::websocket::extract::{Event, State};
use twitch_highway::eventsub::events::chat::ChannelChatMessage;
use twitch_highway::eventsub::websocket::{Request, Revocation, Router, Welcome};
use twitch_highway::eventsub::{websocket, EventSubAPI, SubscriptionType};
use tower::MakeService;
use twitch_highway::eventsub::websocket::routes::{channel_chat_message, revocation, welcome};
use anyhow::Context;
use log::{debug, error};
use crate::commands::command_executor_service::{ChatMessage, TwitchUser, TwitchUserPermission};
use crate::state::{L2State};
use crate::twitch::twitch_service::TwitchService;

impl TwitchService {
    pub(super) async fn start_websocket(state: Arc<L2State>, commands_sender: Sender<Box<ChatMessage>>) {
        let process_messages = async move |state: State<Arc<L2State>>, message: Event<ChannelChatMessage>| {
            //TODO deduplicate message (ids) with ringbuffer
            let message = message.0;
            let c = ChatMessage {
                message_id: message.message_id.into_boxed_str(),
                message: message.message.text.into_boxed_str(),
                user: TwitchUser {
                    id: Box::from(message.chatter_user_id.as_str()),
                    name: Box::from(message.chatter_user_name.as_str()),
                    //TODO do proper permission detection using badges
                    permission: TwitchUserPermission::Everyone,
                    subscriber_months: 0,
                    subscription_tier: 0,
                },
                get_custom_reward_id: message.channel_points_custom_reward_id.map(String::into_boxed_str),
                reply_to_message_id: message.reply.map(|t| t.parent_message_id.into_boxed_str()),
                channel_id: message.source_broadcaster_user_id
                    .map(|t1| Box::from(t1.as_str()))
                    .unwrap_or(state.twitch_service.config.channel_name.clone().into_boxed_str()),
                received_at: Instant::now(),
            };
            debug!("Received Twitch Messages: {} on {}, with badges: {}", c.user.name, c.channel_id, message.badges.iter().map(|x| x.set_id.as_str()).collect::<Vec<_>>().join(", "));
            let _ = commands_sender.send(Box::from(c));
        };

        async fn process_welcome(state: State<Arc<L2State>>, Event(welcome): Event<Welcome>) {
            let session_id = welcome.payload.session.id;
            //TODO do we get called again when automatically reconnecting, and if so, is it okay that we are subscribing again
            state.twitch_service.twitch_api.read().await.websocket_subscription(SubscriptionType::ChannelChatMessage, session_id);
        }
        async fn process_revocation(_state: State<Arc<L2State>>, _welcome: Event<Revocation>) {}

        let twitch_router = <Router as MakeService<(), Request>>::into_service(Router::<Arc<L2State>>::new()
            .route(welcome(process_welcome))
            .route(revocation(process_revocation))
            .route(channel_chat_message(process_messages))
            .with_state(state));

        let _ws = websocket::client("wss://eventsub.wss.twitch.tv/ws", twitch_router).await;
        if let Err(e) = _ws {
            error!("Unable to connect to twitch websocket: {:?}", e);
        }
    }
}

impl TwitchService {
    pub async fn get_user_by_id(&self, id: String) -> Result<Option<User>, anyhow::Error> {
        let ids = [UserId::from(id)];
        let req = || async {
            self.twitch_api
                .read()
                .await
                .get_users()
                .ids(&ids)
                .json()
                .await
        };
        let mut res = self.handle_401_and_retry(req).await.context("failed to get user by id")?;
        Ok(res.data.pop())
    }

    pub fn send_raw_template(&self, _template: &str, _values: HashMap<String, Box<dyn Any>>) {
        // all errors should just be logged
        todo!()
    }
}