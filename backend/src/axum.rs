use crate::oauth_endpoint::{list_oauth, receive_oauth};
use crate::AppState;
use axum::routing::{any, delete, get, post};
use axum::Router;
use std::sync::Arc;
use url::form_urlencoded;
use crate::commands::command_controller::{get_all_user_commands, delete_by_id, get_all_commands, get_by_trigger_id, save, set_enabled, set_visible};

pub type AxumState = Arc<AppState>;

pub async fn axum(on_port: u16, oauth_service: AxumState) {
    let app = Router::new()
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
        .with_state(oauth_service);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", on_port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn url_encode<'a>(input: &str) -> String {
    form_urlencoded::byte_serialize(input.as_bytes()).collect()
}
