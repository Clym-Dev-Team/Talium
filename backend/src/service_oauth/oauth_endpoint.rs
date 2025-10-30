use crate::axum::{url_encode, AxumState};
use crate::service_oauth::oauth_service::OauthReturnError;
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::response::Result as AxResult;
use axum::Json;
use reqwest::StatusCode;
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
) -> AxResult<impl IntoResponse> {
    let panel_base_url = if let Ok(v) = state.l1.webserver_config.read() {
        v.panel_base_url.as_str().to_string()
    } else {
        // log lock poisoned
        return Err(StatusCode::INTERNAL_SERVER_ERROR)?;
    };
    // Body:new( should be Redirect:to(& but for testing with postman, this is deactivated
    if query.error.is_some() || query.error_description.is_some() {
        return Ok(Body::new(format!("{}?success=false&error={}", panel_base_url, url_encode(&query.error_description.unwrap_or(query.error.unwrap())))));
    }
    if query.scope.is_none() {
        return Ok(Body::new(format!("{}?success=false&error={}", panel_base_url, url_encode("Query param scope is required for non error Oauth response"))))
    }
    if query.code.is_none() {
        return Ok(Body::new(format!("{}?success=false&error={}", panel_base_url, url_encode("Query param code is required for non error Oauth response"))))
    }
    Ok(Body::new(format!("{}{}", panel_base_url, match state.l1.oauth_service.return_oauth(service, query.state, query.scope.unwrap(), query.code.unwrap()) {
        Ok(()) => format!("{}?success=true", panel_base_url),
        Err(OauthReturnError::NotRequested) => format!("{}?success=false&error={}", panel_base_url, url_encode("This oauth was never requested from the bot")),
        Err(OauthReturnError::ReturnChannelClosed) => {
            eprintln!("Failed to process auth code, return channel was closed");
            format!("{}?success=false&error={}", panel_base_url, url_encode("Could not process auth code. Internal server error"))
        }
    })))
}

pub async fn list_oauth(State(state): State<AxumState>) -> impl IntoResponse {
    Json(state.l1.oauth_service.get_active_requests())
}
