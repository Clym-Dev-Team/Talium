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
use tokio::runtime::Handle;
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
    pub command_executor_service: CommandExecutorService,
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

pub struct ApplicationState {
    pub l1: Arc<L1State>,
    pub l2: OnceLock<Arc<L2State>>,
    // ensures only one transition can happen at once, even when transitions multiple levels at one time. Also stops from the full state being initialized more than once
    transition_lock: tokio::sync::Mutex<()>,
}

// State Stage Transitions
pub struct L1Application(Arc<ApplicationState>);
pub struct L2Application(Arc<ApplicationState>);
pub struct FullApplication(Arc<ApplicationState>);

impl L1Application {
    pub async fn new(prod_db: ProdDB, webserver_config: WebserverConfig) -> L1Application {
        let l1 = L1State {
            command_executor_service: CommandExecutorService::new(&prod_db).await,
            session_service: SessionService::new(),
            oauth_service: OAuthService::new(),
            webserver_config: RwLock::new(webserver_config),
            prod_db,
        };
        L1Application(Arc::new(ApplicationState {
            l1: Arc::new(l1),
            l2: Default::default(),
            transition_lock: Default::default(),
        }))
    }
    pub async fn upgrade(self, twitch_config: TwitchConfig, webserver_port: u16) -> Result<L2Application, (Self, anyhow::Error)> {
        let l = self.0.transition_lock.lock().await;
        assert!(self.0.l2.get().is_none());
        let service = match TwitchService::new(self.0.l1.clone(), twitch_config).await {
            Ok(service) => service,
            Err(e) => {
                drop(l);
                return Err((self, e.context("Unable to initialize Twitch service")));
            }
        };
        let l2 = Arc::new(L2State {
            twitch_service: service,
        });
        self.0.l2.set(l2.clone())
            .map_err(|_| { () }) // so that we don't print the l2State
            .expect("l2 to be uninitialized because we hold the lock and l2 was checked to be uninitialized");
        let axum_state = self.0.clone();
        Handle::current().spawn(async move { crate::axum::axum(webserver_port, axum_state).await });
        drop(l);
        Ok(L2Application(self.0))
    }
}

impl L2Application {
    pub async fn upgrade(self) -> FullApplication {
        let l2 = self.0.l2.get().expect("L2Application should have l2State");
        let full = Arc::new(FullState {
            l1: self.0.l1.clone(),
            l2: l2.clone(),
        });

        l2.twitch_service.start_receiving_events(l2.clone(), full);
        FullApplication(self.0)
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