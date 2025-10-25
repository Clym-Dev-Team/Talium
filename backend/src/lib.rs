use crate::commands::twitch_service::TwitchService;
use crate::db::ProdDB;
use crate::oauth_service::OAuthService;
use crate::session_service::SessionService;
use rand::distr::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlConnectOptions;
use sqlx::MySqlPool;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use tokio::runtime::Handle;
use url::Url;
use crate::commands::command_executor_service::CommandExecutorService;

mod oauth_service;
mod webserver_authentication;
mod oauth_endpoint;
mod axum;
mod db;
mod session_service;
mod panel_user;
mod panel_user_service;
mod commands;
mod websocket_proxy;

pub async fn start() {
    //check if all mandatory configuration values are set
    //start mini webserver

    let db_connection = MySqlPool::connect(std::env::var("DATABASE_URL").unwrap().as_str()).await.unwrap();
    let prod_db = ProdDB::new(db_connection);
    println!("established db connection");

    let app_state = Arc::new(AppState {
        session_service: SessionService::default(),
        oauth_service: OAuthService::default(),
        twitch_service: TwitchService,
        prod_db,
        webserver_config: RwLock::new(WebserverConfig {
            panel_base_url: Url::from_str("http://localhost:5173").unwrap(),
            server_base_url: Url::from_str("http://localhost:4771").unwrap(),
        }),
        command_executor_service: CommandExecutorService::default(),
    });
    let a2 = app_state.clone();
    Handle::current().spawn(async { axum::axum(4771, a2).await });

    let state = rand::rng().sample_iter(&Alphanumeric).take(30).map(char::from).collect();
    println!("state: {:?}", state);
    let oauth = app_state.oauth_service.new_oauth_request("twitch".to_string(), "account".to_string(), "".to_string(), state);
    println!("oauth: {:?}", oauth);
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
struct WebserverConfig {
    panel_base_url: Url,
    server_base_url: Url
    // maybe we need to add stuff like cors and disable auth here
}

struct AppState {
    pub prod_db: ProdDB,
    pub session_service: SessionService,
    pub oauth_service: OAuthService,
    pub webserver_config: RwLock<WebserverConfig>,
    pub twitch_service: TwitchService,
    pub command_executor_service: CommandExecutorService
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
    struct PanelWebServer;
    struct WatchtimeService;
    // CommandsService;
    struct TimerService;
    struct GiveawayService;
    struct StreamInfoEditorService;
}