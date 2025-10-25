use std::str::FromStr;
use crate::commands::command_controller::{delete_by_id, get_all_commands, get_all_user_commands, get_by_trigger_id, save, set_enabled, set_visible};
use crate::oauth_endpoint::{list_oauth, receive_oauth};
use crate::AppState;
use axum::routing::{any, delete, get, post};
use axum::Router;
use axum_proxy::AppendPrefix;
use std::sync::Arc;
use tower_http::body::Full;
use tower_http::cors::CorsLayer;
use tower_http::services::{Redirect, ServeDir, ServeFile};
use url::form_urlencoded;
use crate::websocket_proxy::websocket_proxy;

pub type AxumState = Arc<AppState>;

pub async fn axum(on_port: u16, state: AxumState) {
    let static_server = ServeDir::new(":/example").fallback(ServeFile::new("./example/index.html"));
    let (panel_base_url, server_base_url) = {
        let guard = state.webserver_config.read().unwrap();
        (guard.panel_base_url.clone(), guard.server_base_url.clone())
    };
    println!("Proxying panel development server: {}", panel_base_url);
    // let panel_ws_proxy = axum_proxy::builder(axum_proxy::client::http_default(), "ws", authority.as_str()).unwrap().build(AppendPrefix("/panel"));
    // let panel_proxy = axum_proxy::builder_http(panel_base_url.authority()).unwrap().build(AppendPrefix("/panel"));
    // let public_panel_url = http::Uri::from_str(server_base_url.join("/panel").unwrap().as_str());
    // println!("redirecting / to {:?}", public_panel_url);
    // let re = Redirect::<Full>::temporary(public_panel_url.unwrap());

    let cors_allow_all = CorsLayer::very_permissive();

    let bot_router = Router::new()
        .route("/commands/userAll", get(get_all_user_commands))
        .route("/commands/all", get(get_all_commands))
        .route("/commands/id/{triggerId}", get(get_by_trigger_id))
        .route("/commands/id/{triggerId}/set/enabled", post(set_enabled))
        .route("/commands/id/{triggerId}/set/visible", post(set_visible))
        .route("/commands/save", post(save))
        .route("/commands/delete/{id}", delete(delete_by_id))

        .route("/setup/auth/list", get(list_oauth))
        // apply auth middleware to all above here
        .route("/auth/{service}", any(receive_oauth))
        .layer(cors_allow_all);
    let app = Router::new()
        .nest("/bot", bot_router)
        // .nest_service("/panel", static_server)
        // .route_service("/panel/", panel_ws_proxy)
        // .nest_service("/panel", panel_proxy.clone())
        // .route_service("/", re)
        .route("/panel", any(websocket_proxy))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", on_port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn url_encode<'a>(input: &str) -> String {
    form_urlencoded::byte_serialize(input.as_bytes()).collect()
}
