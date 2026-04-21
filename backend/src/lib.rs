use crate::db::ProdDB;
use crate::state::L1Application;
use crate::twitch::twitch_service::TwitchService;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::str::FromStr;
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

    let webserver_config = WebserverConfig {
        panel_base_url: Url::from_str("http://localhost:4771/panel").unwrap(),
        server_base_url: Url::from_str("http://localhost:4771").unwrap(),
        panel_auth_twitch_client_id: "zmxjjn3xmncg8ewew6tjk08tub26bb".to_string()
    };
    let app = L1Application::new(prod_db, webserver_config).await;

    let app = app.upgrade(TwitchService::get_config_from_env(), 4771).await
        .map_err(move |(_, e)| e)
        .unwrap();
    let app = app.upgrade().await;
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
