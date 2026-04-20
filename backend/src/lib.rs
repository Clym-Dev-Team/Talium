use crate::commands::command_executor_service::CommandExecutorService;
use crate::db::ProdDB;
use crate::service_oauth::oauth_service::OAuthService;
use crate::session_service::SessionService;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use state::{L1State, WebserverState};
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use log::error;
use tokio::runtime::Handle;
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
mod state;

pub async fn start() {
    env_logger::init();

    // TODO configure slqx tls
    let db_connection = MySqlPool::connect(std::env::var("DATABASE_URL").unwrap().as_str()).await.unwrap();
    let prod_db = ProdDB::new(db_connection);
    println!("established db connection");

    let l1 = Arc::new(L1State {
        webserver_config: RwLock::new(WebserverConfig {
            panel_base_url: Url::from_str("http://localhost:4771/panel").unwrap(),
            server_base_url: Url::from_str("http://localhost:4771").unwrap(),
            panel_auth_twitch_client_id: "zmxjjn3xmncg8ewew6tjk08tub26bb".to_string()
        }),
        command_executor_service: CommandExecutorService::new(&prod_db).await,
        oauth_service: OAuthService::new(),
        session_service: SessionService::new(),
        prod_db,
    });

    let webserver_state = Arc::new(WebserverState::new(l1.clone()));
    let a2 = webserver_state.clone();
    Handle::current().spawn(async { axum::axum(4771, a2).await });
    if let Err(e) = webserver_state.default_twitch().await {
        error!("Error Starting default twitch {:?}", e);
    }
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
