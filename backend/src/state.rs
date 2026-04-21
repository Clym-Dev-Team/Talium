use std::convert::Infallible;
use std::ops::Deref;
use crate::commands::command_executor_service::{CommandExecutorService};
use crate::twitch::twitch_service::{TwitchConfig, TwitchService};
use crate::db::ProdDB;
use crate::service_oauth::oauth_service::OAuthService;
use crate::session_service::SessionService;
use crate::WebserverConfig;
use axum::extract::{FromRequestParts};
use std::sync::{Arc, OnceLock, RwLock, RwLockReadGuard};
use anyhow::Context;
use axum::http::request::Parts;
use log::{error};
use reqwest::StatusCode;
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

// For Axum FromRequestParts
pub struct L1Arc(Arc<L1State>);
pub struct L2Arc(Arc<L2State>);

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
            l1,
            transition_lock: Default::default(),
            l2: Default::default(),
        }
    }
    //TODO move these state change methods on an enum that wraps the WebserverState
    // that way the state changes are type safe, and stuff like ::AlreadyInitialized and ::SkippedLowerInitialization can be removed
    pub async fn init_l2(&self, twitch_config: TwitchConfig) -> InitializationResult {
        let _l = self.transition_lock.lock().await;
        if self.l2.get().is_some() {
             return Err(InitializationError::AlreadyInitialized)
        }
        let service = TwitchService::new(self.l1.clone(), twitch_config)
            .await
            .context("Unable to initialize Twitch service")?;
        let l2 = Arc::new(L2State {
            twitch_service: service,
        });
        // if let Err(e) = l2.default_full().await {
            // semantically, only InitializationError makes sense here, SkippedLowerInitialization and AlreadyInitialized
            // should never occur here, because we just now established the lower level. It would be nice if we could encode that into the typesystem
            // info!("Unable to eagerly initialize full state: {:?}", e);
        // };
        self.l2.set(l2)
            .map_err(|_| {()})// so that we don't print the l2State
            .expect("l2 to be uninitialized because we hold the lock and l2 was checked to be uninitialized");
        Ok(())
    }

    pub async fn default_full(&self) -> InitializationResult {
        let l2 = match self.l2.get() {
            Some(l2) => l2.clone(),
            None => return Err(InitializationError::SkippedLowerInitialization),
        };
        let full = Arc::new(FullState {
            l1: self.l1.clone(),
            l2: l2.clone(),
        });

        l2.twitch_service.start_receiving_events(l2.clone(), full);
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