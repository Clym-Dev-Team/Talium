use std::convert::Infallible;
use std::ops::Deref;
use crate::commands::command_executor_service::{ChatMessage, CommandExecutorService};
use crate::twitch::twitch_service::{TwitchConfig, TwitchService};
use crate::db::ProdDB;
use crate::service_oauth::oauth_service::OAuthService;
use crate::session_service::SessionService;
use crate::WebserverConfig;
use axum::extract::{FromRequestParts};
use std::sync::{Arc, RwLock, RwLockReadGuard};
use anyhow::Context;
use axum::http::request::Parts;
use log::{error, info};
use reqwest::StatusCode;
use tokio::runtime::Handle;
use tokio::sync::broadcast::error::RecvError;
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

pub struct L1State {
    pub prod_db: ProdDB,
    pub session_service: SessionService,
    pub oauth_service: OAuthService,
    pub webserver_config: RwLock<WebserverConfig>,
    pub command_executor_service: CommandExecutorService
}

impl L1State {
    pub fn read_webserver_config(&self) -> RwLockReadGuard<'_, WebserverConfig> {
        if let Ok(v) = self.webserver_config.read() {
            v
        } else {
            error!("webserver_config lock poisoned, clearing poison and hoping for the best!");
            self.webserver_config.clear_poison();
            self.webserver_config.read().unwrap()
        }
    }
}

pub struct L2State {
    pub twitch_service: TwitchService,
}

pub struct L1Arc(Arc<L1State>);
pub struct L2Arc(Arc<L2State>);

pub struct FullState {
    pub l1: Arc<L1State>,
    pub l2: Arc<L2State>,
}

type CompletedFull = bool;

struct L1Stage {
    pub l1: Arc<L1State>,
}

struct L2Stage {
    pub l1: Arc<L1State>,
    pub l2: Arc<L2State>,
}

struct CompleteStage {
    pub l1: Arc<L1State>,
    pub l2: Arc<L2State>,
}

pub struct WebserverState {
    state: tokio::sync::RwLock<InnerState>,
    // ensures only one transition can happen at once, even when transitions multiple levels at one time. Also stops from the full state being initialized more than once
    transition_lock: tokio::sync::Mutex<CompletedFull>,
}

pub enum InnerState {
    L1Stage(L1Stage),
    L2Stage(L2Stage),
    CompleteStage(CompleteStage),
}

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
        WebserverState{
            state: tokio::sync::RwLock::from(InnerState::L1Stage(L1Stage { l1 })),
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
        self.setup_twitch(twitch_config).await
    }

    pub async fn setup_twitch(&self, twitch_config: TwitchConfig) -> InitializationResult {
        let _l = self.transition_lock.lock().await;
        let l2 = match self.state.read().await.deref() {
            InnerState::L1Stage(l1) => l1.init_l2(twitch_config).await?,
            _ => return Err(InitializationError::AlreadyInitialized),
        };
        {
            *self.state.write().await = InnerState::L2Stage(l2);
            // drop write lock before mutex
        }
        Ok(())
    }

    pub async fn get_l1(&self) -> Arc<L1State> {
        match self.state.read().await.deref() {
            InnerState::L1Stage(l1) => l1.l1.clone(),
            InnerState::L2Stage(l2) => l2.l1.clone(),
            InnerState::CompleteStage(c) => c.l1.clone(),
        }
    }

    pub async fn get_l2(&self) -> Option<Arc<L2State>> {
        match self.state.read().await.deref() {
            InnerState::L1Stage(l1) => None,
            InnerState::L2Stage(l2) => Some(l2.l2.clone()),
            InnerState::CompleteStage(c) => Some(c.l2.clone()),
        }
    }
}

impl L1Stage {
    async fn init_l2(&self, twitch_config: TwitchConfig) -> anyhow::Result<L2Stage> {
        let service = TwitchService::new(self.l1.clone(), twitch_config)
            .await
            .context("Unable to initialize Twitch service")?;
        let l2 = Arc::new(L2State {
            twitch_service: service,
        });
        let l2 = L2Stage { l2, l1: self.l1.clone() };
        if let Err(e) = l2.default_full().await {
            // semantically, only InitializationError makes sense here, SkippedLowerInitialization and AlreadyInitialized
            // should never occur here, because we just now established the lower level. It would be nice if we could encode that into the typesystem
            info!("Unable to eagerly initialize full state: {:?}", e);
        };
        Ok(l2)
    }
}

impl L2Stage {
    async fn default_full(&self) -> anyhow::Result<CompleteStage> {
        let full = Arc::new(FullState {
            l1: self.l1.clone(),
            l2: self.l2.clone(),
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

        Ok(CompleteStage { l1: self.l1.clone(), l2: self.l2.clone() })
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
        Ok(L1Arc(state.get_l1().await))
    }
}

impl FromRequestParts<AxumState> for L2Arc {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(_parts: &mut Parts, state: &AxumState) -> Result<Self, Self::Rejection> {
        state.get_l2().await
            .ok_or((StatusCode::SERVICE_UNAVAILABLE, "This feature is currently not available. This can also be the case on server startup"))
            .map(|t| L2Arc(t.clone()))
    }
}