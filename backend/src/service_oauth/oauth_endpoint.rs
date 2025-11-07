use crate::axum::{url_encode, AxumState};
use crate::service_oauth::oauth_service::OauthReturnError;
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::response::Result as AxResult;
use axum::Json;
use log::error;
use serde::Deserialize;
use crate::state::L1Arc;

#[derive(Deserialize)]
pub struct ReceiveOAuthQuery {
    state: String,
    scope: Option<String>,
    code: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

pub async fn receive_oauth(
    l1: L1Arc,
    Path(service): Path<String>,
    Query(query): Query<ReceiveOAuthQuery>,
) -> AxResult<impl IntoResponse> {
    let panel_base_url = if let Ok(v) = l1.webserver_config.read() {
        v.panel_base_url.as_str().to_string()
    } else {
        error!("Error, webserver_config lock poisoned, unable to set panel redirect url");
        "".to_string()
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
    let result = l1.oauth_service.return_oauth(service, query.state, query.scope.unwrap(), query.code.unwrap());
    Ok(Body::new(format!("{}{}", panel_base_url, match result {
        Ok(()) => format!("{}?success=true", panel_base_url),
        Err(OauthReturnError::NotRequested) => format!("{}?success=false&error={}", panel_base_url, url_encode("This oauth was never requested from the bot")),
        Err(OauthReturnError::ReturnChannelClosed) => {
            error!("Error, failed to process auth code, return channel was unexpectedly closed");
            format!("{}?success=false&error={}", panel_base_url, url_encode("Could not process auth code. Internal server error"))
        }
    })))
}

pub async fn list_oauth(l1: L1Arc) -> impl IntoResponse {
    Json(l1.oauth_service.get_active_requests())
}
