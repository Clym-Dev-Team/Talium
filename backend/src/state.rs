use std::convert::Infallible;
use std::ops::Deref;
use crate::commands::command_executor_service::{ChatMessage, CommandExecutorService};
use crate::twitch::twitch_service::{TwitchConfig, TwitchService};
use crate::db::ProdDB;
use crate::service_oauth::oauth_service::OAuthService;
use crate::session_service::SessionService;
use crate::WebserverConfig;
use axum::extract::{FromRequestParts};
use std::sync::{Arc, OnceLock, RwLock};
use anyhow::Context;
use axum::http::request::Parts;
use log::{error, info, warn};
use reqwest::StatusCode;
use tokio::runtime::Handle;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::MutexGuard;
use crate::axum::AxumState;

#[allow(dead_code)]
mod _theoretical_services_checklist {
    struct SetupWebserver;
    struct PreMigrationsDB;
    // ProdDB
    struct Webconsole;
    struct WebAlerting;
    struct DiscordAlerting;
    struct TwitchClient;
    // AuthenticationService;
    // embedded_panel_server;
    struct WatchtimeService;
    // CommandsService;
    struct TimerService;
    struct GiveawayService;
    struct StreamInfoEditorService;
}

pub struct FullState {
    pub l1: Arc<L1State>,
    pub l2: Arc<L2State>,
}

type CompletedFull = bool;
pub struct WebserverState {
    pub l1: Arc<L1State>,
    pub l2: OnceLock<Arc<L2State>>,
    // ensures only one transition can happen at once, even when transitions multiple levels at one time. Also stops from the full state being initialized more than once
    transition_lock: tokio::sync::Mutex<CompletedFull>,
}

pub struct L1State {
    pub prod_db: ProdDB,
    pub session_service: SessionService,
    pub oauth_service: OAuthService,
    pub webserver_config: RwLock<WebserverConfig>,
    pub command_executor_service: CommandExecutorService
}

pub struct L2State {
    pub twitch_service: TwitchService,
}

pub struct L1Arc(Arc<L1State>);
pub struct L2Arc(Arc<L2State>);

#[derive(Debug)]
pub enum InitializationError {
    SkippedLowerInitialization,
    AlreadyInitialized,
    InitializationError(anyhow::Error),
}
pub type InitializationResult = Result<(), InitializationError>;

impl From<anyhow::Error> for InitializationError {
    fn from(value: anyhow::Error) -> Self {
        InitializationError::InitializationError(value)
    }
}


// State Stage Transitions
impl WebserverState {
    pub fn new(l1: Arc<L1State>) -> Self {
        WebserverState {
            l1,
            l2: Default::default(),
            transition_lock: Default::default(),
        }
    }

    pub async fn default_twitch(&self) -> InitializationResult {
        // todo get twitch config from database
        let twitch_config = TwitchConfig {
            channel_name: std::env::var("TWITCH_LISTEN_CHANNEL").unwrap(),
            chat_account_name: std::env::var("TWITCH_ACCOUNT_NAME").unwrap(),
            send_to: std::env::var("TWITCH_SEND_TO").unwrap(),
            client_id: std::env::var("TWITCH_CLIENT_ID").unwrap(),
            client_secret: std::env::var("TWITCH_CLIENT_SECRET").unwrap(),
        };
        let l = self.transition_lock.lock().await;
        self.init_l2(l, twitch_config).await
    }

    pub async fn setup_twitch(&self, twitch_config: TwitchConfig) -> InitializationResult {
        let l = self.transition_lock.lock().await;
        self.init_l2(l, twitch_config).await
    }

    async fn init_l2(&self, transition_lock: MutexGuard<'_, CompletedFull>, twitch_config: TwitchConfig) -> InitializationResult {
        if self.l2.get().is_some() {
            return Err(InitializationError::AlreadyInitialized);
        }
        let service = TwitchService::new(self.l1.clone(), twitch_config)
            .await
            .context("Unable to initialize Twitch service")?;
        let l2 = Arc::new(L2State {
            twitch_service: service,
        });
        self.l2.set(l2).map_err(|_| {
            // this code path should never happen, all state changes should respect the lock
            error!("Tried to initialize more than once after is already initialized check, and while holding the lock!");
            InitializationError::AlreadyInitialized
        })?;
        if let Err(e) = self.default_full(transition_lock).await {
            // semantically, only InitializationError makes sense here, SkippedLowerInitialization and AlreadyInitialized
            // should never occur here, because we just now established the lower level. It would be nice if we could encode that into the typesystem
            info!("Unable to eagerly initialize full state: {:?}", e);
        };
        Ok(())
    }

    async fn default_full(&self, mut transition_lock: MutexGuard<'_, CompletedFull>) -> InitializationResult {
        let Some(l2)  =  self.l2.get() else {
            return Err(InitializationError::SkippedLowerInitialization);
        };
        if *transition_lock {
            return Err(InitializationError::AlreadyInitialized);
        }
        let full = Arc::new(FullState {
            l1: self.l1.clone(),
            l2: l2.clone(),
        });
        let (chat_channel, _) = tokio::sync::broadcast::channel::<Box<ChatMessage>>(20);
        // fanout of messages
        let full2 = full.clone();
        let sender2 = chat_channel.clone();
        Handle::current().spawn(async { TwitchService::start_websocket(full2, sender2) });

        let mut receiver = chat_channel.subscribe();
        let full2 = full.clone();
        Handle::current().spawn(async move {
            loop {
                let message = match receiver.recv().await {
                    Ok(m) => *m,
                    Err(RecvError::Closed) => return,
                    Err(RecvError::Lagged(skipped)) => {
                        error!("Twitch ChatMessage channel lagged, skipped {} messages!", skipped);
                        continue;
                    }
                };
                let full = full2.clone();
                Handle::current().spawn(async move {
                    CommandExecutorService::process_chat_message(full, message).await;
                });
            }
        });
        *transition_lock = true;
        drop(transition_lock);
        Ok(())
    }
}

// LxArc Impls
impl Deref for L1Arc {
    type Target = Arc<L1State>;

    fn deref(&self) -> &Self::Target {
       &self.0
    }
}

impl Deref for L2Arc {
    type Target = Arc<L2State>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequestParts<AxumState> for L1Arc {
    type Rejection = Infallible;

    async fn from_request_parts(_parts: &mut Parts, state: &AxumState) -> Result<Self, Self::Rejection> {
        Ok(L1Arc(state.l1.clone()))
    }
}

impl FromRequestParts<AxumState> for L2Arc {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(_parts: &mut Parts, state: &AxumState) -> Result<Self, Self::Rejection> {
        state.l2.get()
            .ok_or((StatusCode::SERVICE_UNAVAILABLE, "This feature is currently not available. This can also be the case on server startup"))
            .map(|t| L2Arc(t.clone()))
    }
}