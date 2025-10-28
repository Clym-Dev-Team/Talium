use crate::axum::AxumState;
use crate::db::ProdDB;
use crate::service_oauth::oauth_service::OAuthService;
use crate::twitch::authentication::{authorization_url, refresh_token, validate_token};
use anyhow::Context;
use asknothingx2_util::oauth::{AccessToken, ClientId};
use serde::Deserialize;
use sqlx::types::chrono::NaiveDateTime;
use std::any::Any;
use std::collections::HashMap;
use std::io::Stderr;
use std::sync::Mutex;
use std::time::Instant;
use tower::{Layer, MakeService};
use tower_http::follow_redirect::policy::PolicyExt;
use tower_service::Service;
use twitch_highway::chat::ChatAPI;
use twitch_highway::eventsub::events::chat::ChannelChatMessage;
use twitch_highway::eventsub::websocket::extract::{Event, State};
use twitch_highway::eventsub::websocket::routes::{channel_chat_message, revocation, welcome};
use twitch_highway::eventsub::websocket::{Request, Revocation, Router, Welcome};
use twitch_highway::eventsub::{websocket, EventSubAPI, SubscriptionType};
use twitch_highway::types::{BroadcasterId, SessionId, UserId};
use twitch_highway::users::{User, UserAPI, UsersInfoResponse};
use twitch_highway::{Error, TwitchAPI};
use crate::service_oauth::oauth_endpoint::get_redirect_url;
use crate::WebserverConfig;

#[derive(Default, Deserialize)]
struct OauthCredential {
    access_token: String,
    refresh_token: String,
    scopes: Vec<String>,
    expires_at: NaiveDateTime,
}

#[derive(Default, Deserialize)]
struct TwitchConfig {
    channel_name: String,
    chat_account_name: String,
    send_to: String,
    credential: OauthCredential,
    client_id: String,
    client_secret: String,
}

pub struct TwitchService {
    twitch_api: TwitchAPI,
    token_validation: Mutex<(Instant, bool)>
}

impl TwitchService {
    pub async fn new(db: ProdDB, oauth_service: OAuthService, webserver_config: WebserverConfig, twitch_config: TwitchConfig) -> Result<TwitchService, anyhow::Error> {
        //TODO get credentials from db (our caller does that)
        let token = if let Some(refreshed) = refresh_token(twitch_config.credential.access_token.as_str())
            .await
            .context("Unable to start twitch input because oauth could not be refreshed")? {
            refreshed
        } else {
            let (url, state) = authorization_url(twitch_config.client_id, get_redirect_url(webserver_config.panel_base_url.to_string(), "twitch"));
            let code = oauth_service.new_oauth_request("twitch", twitch_config.chat_account_name, url, state);
            let token = refresh_token(code.as_str()).await
                .context("Unable to get oauth tokens for auth flow code")?
                .context("Auth flow code invalid, unable to start twitch client, could not get oauth token")?;
            //TODO save to db
            token
        };
        let api = TwitchAPI::new(
            AccessToken::from("your_access_token"),
            ClientId::from("your_client_id"),
        );

        //TODO setup websocket
        //TODO setup websocket subscriptions
        //TODO setup periodic token refresher
        //TODO refresh and save token on shutdown
        let userId = UserId::from("userId");
        let d = api.get_users().ids(&[userId.clone()]).logins(&["test"]).build().json().await.unwrap().data;
        let target_channel = BroadcasterId::from("");

        let d = api.send_chat_message(&target_channel, &userId, "").for_source_only(true).send().await;
        let d = api.websocket_subscription(SubscriptionType::ChannelChatMessage, SessionId::from("")).send().await.unwrap();

        todo!()
        // AccessToken Validation
        // Problem 1: This library does not tell us what really happened. We will get a 401 from twitch,
        // but that can have more reasons than just a timed out token.
        // We are required anyway to validate our token hourly.
        // But what if we miss that, or just something happens with that alone does not keep our token alive
        // so we should at least check if the reason why we failed was because the token was just invalid.
        //
        // One Idea would be to impl our own functions on our service that call the library api.
        // We could then use a function or even macro to make this checking (and maybe even retrying) a one-liner.
        //
        // Note: we should probably impl some spam prevention for checking the validity. A Mutex, and something to then return to others if we were successfully, or the token is just completely fucked.
        // The Mutex would need to Values, the Time-of-last-Validation, and the result, (new-)good-token, token-fucked.
        // A Thread would acquire the lock, waiting indicates that another tread is already validating.
        // Then we check the Time-of-last-Validation, if it is earlier than lets say 15min, we revalidate.
        // if it was recent enough, we check the resul if we have a (maybe new) good token now, or the token is permanently fucked and decide if we are going to retry.
        //
        // Validating the Panel tokens is something I would need to thing about longer.
        // realistically we would know after a few hours at most, because we currently require one "action" every 30min to keep the session alive.
        // But currently it is technically possible to keep a session alive indefinitely
        //
        // We could maybe check the validity if the last time we checked is more than 15min ago.
        // The trick is to not do that while validating the request, because that would delay the validation, and thats why we have the sessions.
        // But if we allow the request for now anyway, and then in the background check if the token is valid, and remove it for the next request.
        // One additional benefit is that we are not constantly checking the validity for each user, only when they actually do something.
        //
        // This would mean that our session are always in schrödinger-state of validity (although they are currently anyway)
    }

    pub async fn start_websocket(state: AxumState) {
        async fn process_messages(state: State<AxumState>, message: Event<ChannelChatMessage>) {
            //TODO
            //TODO deduplicate message (ids) with ringbuffer
        }
        async fn process_welcome(state: State<AxumState>, welcome: Event<Welcome>) {
            // here we need to use the sessionId from the welcome message and set it into the client for later use
            // after that we need to register all the topic we need to our sessionId, for that we would need to use the client
        }
        async fn process_revocation(state: State<AxumState>, welcome: Event<Revocation>) {}

        let twitch_router = <twitch_highway::eventsub::websocket::Router as MakeService<(), Request>>::into_service(Router::<AxumState>::new()
            .route(welcome(process_welcome))
            .route(revocation(process_revocation))
            .route(channel_chat_message(process_messages))
            .with_state(state));

        let ws = websocket::client("wss://eventsub.wss.twitch.tv/ws", twitch_router).await;

        todo!()
    }

    pub async fn get_user_by_id(&self, id: String) -> Result<Option<User>, anyhow::Error> {
        let ids = [UserId::from(id)];
        let req = || {
            self.twitch_api
                .get_users()
                .ids(&ids)
                .json()
        };
        let mut res = self.handle_401_and_retry(req).await.context("failed to get user by id")?;
        Ok(res.data.pop())
    }

    #[inline(always)]
    async fn handle_401_and_retry<R, F, Fut>(&self, req: F) -> Result<R, anyhow::Error> where
        F: Fn() -> Fut,
        Fut: Future<Output=Result<R, twitch_highway::Error>>
    {
        match req().await {
            Err(e) if e.is_api() && e.message().is_some_and(|t1| t1.starts_with("HTTP 401")) => {
                //TODO handle unwrap
                let lock = self.token_validation.lock().unwrap();
                if lock.0.elapsed() > std::time::Duration::from_secs(15 * 60) {
                    let validation_result = validate_token(self.twitch_api.access_token()).await?;
                    //TODO if okay, set result and retry
                    // if Err, request new oauth in new thread
                    todo!()
                } else if lock.1 {
                    req().await.context("Retry also failed")
                } else {
                    Err(e).context("Credentials Bad, not retrying")
                }
            },
            Err(e) => Err(e.into()),
            Ok(r) => Ok(r),
        }
    }

    pub fn send_raw_template(&self, _template: &str, _values: HashMap<String, Box<dyn Any>>) {
        // all errors should just be logged
        todo!()
    }
}