use std::str::FromStr;
use crate::commands::command_controller::{delete_by_id, get_all_commands, get_all_user_commands, get_by_trigger_id, save, set_enabled, set_visible};
use crate::dynamic_index_html_handler::DynamicIndexHtmlHandlerService;
use crate::oauth_endpoint::{list_oauth, receive_oauth};
use crate::AppState;
use axum::routing::{any, delete, get, post};
use axum::Router;
use std::sync::Arc;
use tower_http::body::Full;
use tower_http::cors::CorsLayer;
use tower_http::services::{Redirect, ServeDir};
use url::form_urlencoded;

pub type AxumState = Arc<AppState>;

pub async fn axum(on_port: u16, state: AxumState) {
    let (panel_base_url, server_base_url) = {
        let guard = state.webserver_config.read().unwrap();
        (guard.panel_base_url.clone(), guard.server_base_url.clone())
    };
    println!("Hosting embedded panel at: {}panel", server_base_url);

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

    let server_panel = ServeDir::new("target/debug/panel_dist")
        .append_index_html_on_directories(false)
        .fallback(DynamicIndexHtmlHandlerService::new(state.clone()));

    let app = Router::new()
        .nest_service("/panel", server_panel)
        .nest("/bot", bot_router)
        .route_service("/", Redirect::<Full>::temporary(http::Uri::from_str(server_base_url.join("/panel").unwrap().as_str()).unwrap()))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", on_port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn url_encode<'a>(input: &str) -> String {
    form_urlencoded::byte_serialize(input.as_bytes()).collect()
}
