use crate::commands::command_executor_service::{ChatMessage, CommandExecutorService};
use twitch::twitch_service::{TwitchConfig, TwitchService};
use crate::db::ProdDB;
use serde::{Deserialize, Serialize};
use service_oauth::oauth_service::OAuthService;
use sqlx::MySqlPool;
use std::str::FromStr;
use std::sync::{Arc, OnceLock, RwLock};
use tokio::runtime::Handle;
use tokio::sync::broadcast::error::RecvError;
use url::Url;
use state::{FullState, L1State, L2State, WebserverState};

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
        command_executor_service: CommandExecutorService::new(&prod_db).await,
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
    let twitch_config = TwitchConfig {
        channel_name: std::env::var("TWITCH_LISTEN_CHANNEL").unwrap(),
        chat_account_name: std::env::var("TWITCH_ACCOUNT_NAME").unwrap(),
        send_to: std::env::var("TWITCH_SEND_TO").unwrap(),
        client_id: std::env::var("TWITCH_CLIENT_ID").unwrap(),
        client_secret: std::env::var("TWITCH_CLIENT_SECRET").unwrap(),
    };
    let service = TwitchService::new(l1.clone(), twitch_config).await.unwrap();
    let l2 = Arc::new(L2State {
        twitch_service: service,
    });

    // 3rd. (Full) Stage
    let _ = webserver_state.l2.set(l2.clone());
    let full = Arc::new(FullState {
        l1,
        l2,
    });

    let oauth = full.l1.oauth_service.new_oauth_request("twitch".to_string(), "account".to_string(), |_x, _x1| "".to_string());
    println!("oauth: {:?}", oauth);

    let (command_channel, _) = tokio::sync::broadcast::channel::<Box<ChatMessage>>(20);
    // fanout of messages
    let full2 = full.clone();
    let sender2 = command_channel.clone();
    Handle::current().spawn(async { TwitchService::start_websocket(full2, sender2) });
    let mut receiver = command_channel.subscribe();
    let full2 = full.clone();
    Handle::current().spawn(async move {
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
                CommandExecutorService::process_chat_message(full, message).await;
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
