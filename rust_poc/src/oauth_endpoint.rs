use crate::axum::{url_encode, AxumState};
use crate::oauth_service::OauthReturnError;
use crate::webserver_authentication::Moderator;
use crate::PANEL_BASE_URL;
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

/// Get redirect url for a particular service. The url is fully formed with the host accessible from the outside.
///
/// `service_name` is the string name/id of the service that you use to in [OAuthService::new_oauth_request]
pub fn get_redirect_url(bot_base_url_config: String, service_name: &str) -> String {
    bot_base_url_config + "/auth/" + service_name
}

/// Get the public url to the panel oauth setup page
pub fn get_oauth_setup_url(bot_base_url_config: String) -> String {
    bot_base_url_config + "/auth"
}

#[derive(Deserialize)]
pub struct ReceiveOAuthQuery {
    state: String,
    scope: Option<String>,
    code: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

pub async fn receive_oauth(
    State(state): State<AxumState>,
    Path(service): Path<String>,
    Query(query): Query<ReceiveOAuthQuery>,
) -> impl IntoResponse {
    // Body:new( should be Redirect:to(& but for testing with postman, this is deactivated
    if query.error.is_some() || query.error_description.is_some() {
        return Body::new(format!("{}?success=false&error={}", PANEL_BASE_URL, url_encode(&query.error_description.unwrap_or(query.error.unwrap()))));
    }
    if query.scope.is_none() {
        return Body::new(format!("{}?success=false&error={}", PANEL_BASE_URL, url_encode("Query param scope is required for non error Oauth response")))
    }
    if query.code.is_none() {
        return Body::new(format!("{}?success=false&error={}", PANEL_BASE_URL, url_encode("Query param code is required for non error Oauth response")))
    }
    Body::new(format!("{PANEL_BASE_URL}{}", match state.oauth_service.return_oauth(service, query.state, query.scope.unwrap(), query.code.unwrap()) {
        Ok(()) => format!("{}?success=true", PANEL_BASE_URL),
        Err(OauthReturnError::NotRequested) => format!("{}?success=false&error={}", PANEL_BASE_URL, url_encode("This oauth was never requested from the bot")),
        Err(OauthReturnError::ReturnChannelClosed) => {
            eprintln!("Failed to process auth code, return channel was closed");
            format!("{}?success=false&error={}", PANEL_BASE_URL, url_encode("Could not process auth code. Internal server error"))
        }
    }))
}

pub async fn list_oauth(_: Moderator, State(state): State<AxumState>) -> impl IntoResponse {
    Json(state.oauth_service.get_active_requests())
}
