use crate::oauth_endpoint::{list_oauth, receive_oauth};
use crate::AppState;
use axum::routing::{any, get};
use axum::Router;
use std::sync::Arc;
use url::form_urlencoded;

pub type AxumState = Arc<AppState>;

pub async fn axum(on_port: u16, oauth_service: AxumState) {
    let app = Router::new()
        .route("/auth/{service}", any(receive_oauth))
        .route("/setup/auth/list", get(list_oauth))
        .with_state(oauth_service);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", on_port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn url_encode<'a>(input: &str) -> String {
    form_urlencoded::byte_serialize(input.as_bytes()).collect()
}
