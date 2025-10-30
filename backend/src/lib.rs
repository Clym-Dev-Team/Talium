use crate::commands::command_executor_service::{ChatMessage, CommandExecutorService};
use crate::commands::twitch_service::TwitchService;
use crate::db::ProdDB;
use crate::session_service::SessionService;
use serde::{Deserialize, Serialize};
use service_oauth::oauth_service::OAuthService;
use sqlx::MySqlPool;
use std::str::FromStr;
use std::sync::{Arc, OnceLock, RwLock};
use tokio::runtime::Handle;
use tokio::sync::broadcast::error::RecvError;
use url::Url;

mod webserver_authentication;
mod axum;
mod db;
mod session_service;
mod panel_user;
mod commands;
mod panel_frontend;
mod service_oauth;
mod twitch;

pub async fn start() {
    //check if all mandatory configuration values are set
    //start mini webserver

    // L0, mini-webserver stage
    let db_connection = MySqlPool::connect(std::env::var("DATABASE_URL").unwrap().as_str()).await.unwrap();
    let prod_db = ProdDB::new(db_connection);
    println!("established db connection");

    // 1st Stage
    let l1 = Arc::new(L1State {
        webserver_config: RwLock::new(WebserverConfig {
            panel_base_url: Url::from_str("http://localhost:4771/panel").unwrap(),
            server_base_url: Url::from_str("http://localhost:4771").unwrap(),
            panel_auth_twitch_client_id: "zmxjjn3xmncg8ewew6tjk08tub26bb".to_string()
        }),
        oauth_service: Default::default(),
        session_service: Default::default(),
        prod_db,
    });

    // Start Prod Webserver
    let webserver_state = Arc::new(WebserverState {
        l1: l1.clone(),
        l2: OnceLock::new(),
    });
    let a2 = webserver_state.clone();
    Handle::current().spawn(async { axum::axum(4771, a2).await });

    // 2nc Stage
    let service = TwitchService::new(l1.clone(), ()).await.unwrap();
    let l2 = Arc::new(L2State {
        twitch_service: service,
        command_executor_service: Default::default(),
    });

    // 3rd. (Full) Stage
    let _ = webserver_state.l2.set(l2.clone());
    let full = Arc::new(FullState {
        l1,
        l2,
    });

    let oauth = full.l1.oauth_service.new_oauth_request("twitch".to_string(), "account".to_string(), "".to_string(), OAuthService::random_state());
    println!("oauth: {:?}", oauth);

    let (command_channel, _) = tokio::sync::broadcast::channel::<Box<ChatMessage>>(20);
    // fanout of messages
    let full2 = full.clone();
    let sender2 = command_channel.clone();
    Handle::current().spawn(async { TwitchService::start_websocket(full2, sender2) });
    let mut receiver = command_channel.subscribe();
    let full2 = full.clone();
    Handle::current().spawn(async {
        loop {
            let message = match receiver.recv().await {
                Ok(m) => *m,
                Err(RecvError::Closed) => return,
                Err(RecvError::Lagged(_s)) => {
                    // log skip
                    continue;
                }
            };
            let full = full2.clone();
            Handle::current().spawn(async move {
                CommandExecutorService::process_chat_message(&full.l2.command_executor_service, full, message).await;
            });
        }
    });
}

#[derive(Clone, Deserialize, Serialize)]
struct DbConfig {
    db_host: String,
    db_port: u16,
    db_username: String,
    db_password: String,
    db_database: String,
}


/// Very basic, but can already be saved in the Database
#[derive(Clone)]
struct WebserverConfig {
    panel_base_url: Url,
    server_base_url: Url,
    panel_auth_twitch_client_id: String,
    // maybe we need to add stuff like cors and disable auth here
}

struct L1State {
    pub prod_db: ProdDB,
    pub session_service: SessionService,
    pub oauth_service: OAuthService,
    pub webserver_config: RwLock<WebserverConfig>,
}

struct L2State {
    pub twitch_service: TwitchService,
    pub command_executor_service: CommandExecutorService
}

struct WebserverState {
    pub l1: Arc<L1State>,
    pub l2: OnceLock<Arc<L2State>>,
}

struct FullState {
    pub l1: Arc<L1State>,
    pub l2: Arc<L2State>,
}

#[allow(dead_code)]
mod _services {
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
