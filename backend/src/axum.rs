use crate::commands::command_controller::{delete_by_id, get_all_commands, get_all_user_commands, get_by_trigger_id, save, set_enabled, set_visible};
use crate::panel_frontend::embedded_panel_server::{embedded_panel_service, to_panel_redirect, SERVER_PANEL_PATH};
use crate::service_oauth::oauth_endpoint::{list_oauth, receive_oauth};
use crate::webserver_authentication::auth_mod;
use axum::middleware::from_fn_with_state;
use axum::routing::{any, delete, get, post};
use axum::Router;
use axum::http::Uri;
use std::str::FromStr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use url::{form_urlencoded, Url};
use crate::state::WebserverState;

pub type AxumState = Arc<WebserverState>;

pub async fn axum(on_port: u16, state: AxumState) {
    let l1 = state.get_l1().await;
    let webserver_config = {
        l1.read_webserver_config().clone()
    };

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
        .layer(from_fn_with_state(state.clone(), auth_mod))
        .route("/auth/{service}", any(receive_oauth))
        .layer(cors_allow_all);

    let app = Router::new()
        .nest_service(SERVER_PANEL_PATH, embedded_panel_service(l1))
        .route_service("/", to_panel_redirect(webserver_config.server_base_url))
        .nest("/bot", bot_router)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", on_port)).await.unwrap();
    axum::serve(listener, app).await.expect("axum::serve to never return");
}

pub fn url_encode<'a>(input: &str) -> String {
    form_urlencoded::byte_serialize(input.as_bytes()).collect()
}

pub fn convert_url(input: Url) -> Uri {
    Uri::from_str(input.as_str()).expect("url::Url url should be valid http::Uri")
}