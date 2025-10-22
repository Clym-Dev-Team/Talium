use crate::db::ProdDB;
use crate::oauth_service::OAuthService;
use crate::session_service::SessionService;
use rand::distr::Alphanumeric;
use rand::Rng;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::runtime::Handle;

mod oauth_service;
mod webserver_authentication;
mod oauth_endpoint;
mod axum;
mod db;
mod session_service;
mod panel_user;
mod panel_user_service;
mod commands;

pub(crate) static PANEL_BASE_URL: &'static str = "http://localhost:5173";

pub async fn start() {
    //check if all mandatory configuration values are set
    //start mini webserver

    let connection_options = MySqlConnectOptions::new()
        ;
    let db_connection = MySqlPool::connect_with(connection_options).await.unwrap();
    let prod_db = ProdDB::new(db_connection);
    println!("established db connection");

    let app_state = Arc::new(AppState {
        session_service: SessionService::default(),
        oauth_service: OAuthService::default(),
        prod_db,
    });
    let a2 = app_state.clone();
    Handle::current().spawn(async { axum::axum(5000, a2).await });

    let state = rand::rng().sample_iter(&Alphanumeric).take(30).map(char::from).collect();
    println!("state: {:?}", state);
    let oauth = app_state.oauth_service.new_oauth_request("twitch".to_string(), "account".to_string(), "".to_string(), state);
    println!("oauth: {:?}", oauth);
}

struct AppState {
    pub prod_db: ProdDB,
    pub session_service: SessionService,
    pub oauth_service: OAuthService,
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
    struct CommandsService;
    struct TimerService;
    struct GiveawayService;
    struct StreamInfoEditorService;
}