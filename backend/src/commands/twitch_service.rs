use crate::commands::command_executor_service::ChatMessage;
use crate::db::ProdDB;
use crate::service_oauth::oauth_endpoint::get_redirect_url;
use crate::state::{FullState, L1State};
use crate::twitch::authentication::{authorization_url, refresh_token, validate_token};
use anyhow::{anyhow, Context};
use asknothingx2_util::oauth::{AccessToken, ClientId};
use serde::Deserialize;
use sqlx::types::chrono::{DateTime, Local};
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::runtime::Handle;
use tokio::sync::broadcast::Sender;
use tokio::sync::{Mutex, RwLock};
use tower::MakeService;
use twitch_highway::eventsub::events::chat::ChannelChatMessage;
use twitch_highway::eventsub::websocket;
use twitch_highway::eventsub::websocket::extract::{Event, State};
use twitch_highway::eventsub::websocket::routes::{channel_chat_message, revocation, welcome};
use twitch_highway::eventsub::websocket::{Request, Revocation, Router, Welcome};
use twitch_highway::types::UserId;
use twitch_highway::users::{User, UserAPI};
use twitch_highway::TwitchAPI;

#[derive(Default, Deserialize, Clone)]
pub(crate) struct OauthCredential {
    pub access_token: String,
    pub refresh_token: String,
    pub scopes: Vec<String>,
    pub expires_at: DateTime<Local>,
}

#[derive(Default, Clone, Deserialize)]
pub(crate) struct TwitchConfig {
    channel_name: String,
    chat_account_name: String,
    send_to: String,
    client_id: String,
    client_secret: String,
}

struct TwitchCredentialStatus {
    valid_checked_at: Instant,
    was_valid_at_check: bool,
    credential: OauthCredential,
}

pub struct TwitchService {
    twitch_api: Arc<RwLock<TwitchAPI>>,
    token_validation: Mutex<TwitchCredentialStatus>,
    config: TwitchConfig,
    l1: Arc<L1State>,
}

impl TwitchService {
    pub async fn new(l1: Arc<L1State>, twitch_config: TwitchConfig) -> Result<TwitchService, anyhow::Error> {
        let cred_from_db: OauthCredential = !;
        //TODO get credentials from db (our caller does that)
        let oauth = match Self::check_or_get_oauth(&cred_from_db, &l1.prod_db, &twitch_config).await {
            Ok(oauth) => oauth,
            Err(e) => {
                // log err
                TwitchCredentialStatus {
                    was_valid_at_check: true,
                    valid_checked_at: Instant::now(),
                    credential: Self::request_new_oauth(l1, twitch_config.clone()).await
                }
            },
        };
        let api = TwitchAPI::new(
            AccessToken::from(oauth.credential.access_token.as_str()),
            ClientId::from(twitch_config.client_id.as_str()),
        );

        //TODO setup periodic token refresher
        //TODO refresh and save token on shutdown

        // let userId = UserId::from("userId");
        // let d = api.get_users().ids(&[userId.clone()]).logins(&["test"]).build().json().await.unwrap().data;
        // let target_channel = BroadcasterId::from("");

        // let d = api.send_chat_message(&target_channel, &userId, "").for_source_only(true).send().await;
        // let d = api.websocket_subscription(SubscriptionType::ChannelChatMessage, SessionId::from("")).send().await.unwrap();

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
        Ok(TwitchService {
            twitch_api: Arc::new(RwLock::new(api)),
            token_validation: Mutex::new(oauth),
            config: twitch_config,
            l1,
        })
    }

    async fn check_or_get_oauth(cred: &OauthCredential, db: &ProdDB, twitch_config: &TwitchConfig) -> Result<TwitchCredentialStatus, anyhow::Error> {
        const FINAL_ERROR: &str = "Could not get new oauth token, bad credentials, needs reauthentication";
        // is first thread, do validation/refreshing
        let mut errors = vec![];
        match validate_token(cred.access_token.as_str()).await {
            Ok(Some(_validation)) => {
                return Ok(TwitchCredentialStatus {
                    valid_checked_at: Instant::now(),
                    was_valid_at_check: true,
                    credential: cred.clone(),
                })
            },
            Err(e) =>errors.push(e),
            Ok(None) => {}
        };
        match refresh_token(cred.refresh_token.as_str(), twitch_config.client_id.as_str(), twitch_config.client_secret.as_str()).await {
            Ok(Some(refreshed)) => {
                //TODO save to db
                return Ok(TwitchCredentialStatus {
                    valid_checked_at: Instant::now(),
                    was_valid_at_check: true,
                    credential: refreshed.into(),
                })
            }
            Err(e) =>errors.push(e),
            Ok(None) => {}
        }
        //TODO add errors
        Err(anyhow!(FINAL_ERROR))
    }

    async fn request_new_oauth(l1: Arc<L1State>, twitch_config: TwitchConfig) -> OauthCredential {
        let (url, state) = authorization_url(twitch_config.client_id.as_str(), get_redirect_url("".to_string(), "twitch"));
        //TODO make the url kida a builder, that gets evaluated when the api is actually called
        let code = l1.oauth_service.new_oauth_request("twitch", twitch_config.chat_account_name.as_str(), url, state);

        let refreshed = refresh_token(code, twitch_config.client_id.as_str(), twitch_config.client_secret.as_str()).await;
        //TODO save to db
        todo!()
    }

    pub async fn start_websocket(state: Arc<FullState>, commands_sender: Sender<Box<ChatMessage>>) {
        async fn process_messages(state: State<Arc<FullState>>, message: Event<ChannelChatMessage>) {
            //TODO deduplicate message (ids) with ringbuffer
            //TODO push into sender
        }
        async fn process_welcome(state: State<Arc<FullState>>, welcome: Event<Welcome>) {
            // here we need to use the sessionId from the welcome message and set it into the client for later use
            // after that we need to register all the topic we need to our sessionId, for that we would need to use the client
        }
        async fn process_revocation(state: State<Arc<FullState>>, welcome: Event<Revocation>) {}

        let twitch_router = <Router as MakeService<(), Request>>::into_service(Router::<Arc<FullState>>::new()
            .route(welcome(process_welcome))
            .route(revocation(process_revocation))
            .route(channel_chat_message(process_messages))
            .with_state(state));

        let ws = websocket::client("wss://eventsub.wss.twitch.tv/ws", twitch_router).await;

        todo!()
    }

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

    #[inline(always)]
    async fn handle_401_and_retry<R, F, Fut>(&self, req: F) -> Result<R, anyhow::Error> where
        F: Fn() -> Fut,
        Fut: Future<Output=Result<R, twitch_highway::Error>>
    {
        match req().await {
            Err(e) if e.is_api() && e.message().is_some_and(|t1| t1.starts_with("HTTP 401")) => {
                let mut cred_lock = self.token_validation.lock().await;
                if cred_lock.valid_checked_at.elapsed() > std::time::Duration::from_secs(15 * 60) {
                     match Self::check_or_get_oauth(&cred_lock.credential, &self.l1.prod_db, &self.config).await {
                        Ok(d) => {
                            let mut api_lock = self.twitch_api.write().await;
                            *api_lock = api_lock.clone().set_access_token(AccessToken::from(d.credential.access_token.clone()));
                            *cred_lock = d;
                        },
                        Err(e) => {
                            cred_lock.valid_checked_at = Instant::now();
                            cred_lock.was_valid_at_check = false;
                            let config = self.config.clone();
                            let l1 = self.l1.clone();
                            Handle::current().spawn(async {
                                let credential = TwitchService::request_new_oauth(l1, config);
                                //TODO update state
                            });
                            return Err(e).context("Credentials Bad, not retrying");
                        }
                    };
                };
                if !cred_lock.was_valid_at_check {
                    return Err(anyhow!("Could not get new oauth token, bad credentials, needs reauthentication"));
                }
                drop(cred_lock);
                req().await.context("Retry also failed")
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