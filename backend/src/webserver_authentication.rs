use crate::axum::AxumState;
pub(crate) use crate::panel_user::Moderator;
use crate::panel_user::PanelUserService;
use anyhow::Context;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use reqwest::Method;
use serde::Deserialize;
use std::time::Instant;

#[derive(Deserialize)]
struct ValidationReturn {
    #[allow(dead_code)]
    #[serde(rename = "login")]
    user_name: String,
    #[serde(rename = "user_id")]
    user_id: String
}

/// Validate an accessToken with Twitch. Ok(None) represents a successful validation, but the token being invalid
async fn validate_token(access_token: &str) -> anyhow::Result<Option<ValidationReturn>> {
    // prob move client into app state
    let response = reqwest::Client::builder()
        .build()
        .context("Failed to build client builder for twitch access token validation")?
        .request(Method::GET, "https://id.twitch.tv/oauth2/validate")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .context("Failed to send access token validation request to twitch")?;
    if response.status() == StatusCode::UNAUTHORIZED {
        return Ok(None)
    }
    let response_body = response.text()
        .await
        .context("Failed to decode response body from twitch access token validation")?;
    let validation_return: ValidationReturn = serde_json::from_str(&response_body)
        .context("Failed to deserialize response body from twitch access token validation")
        .context("Original response body: ".to_string() + &response_body)?;
    Ok(Some(validation_return))
}

impl FromRequestParts<AxumState> for Moderator {
    type Rejection = (StatusCode, String);

    //this is correct for routes that would definitely need to know which user this was, but we would still want to implement the authentication bypass
    fn from_request_parts(parts: &mut Parts, state: &AxumState) -> impl Future<Output=Result<Self, Self::Rejection>> + Send {

        fn get_header(headers: &HeaderMap<HeaderValue>, key: &str) -> Result<String, (StatusCode, String)> {
            headers
                .get(key).ok_or((StatusCode::UNAUTHORIZED, format!("Missing '{}' authentication header", key)))
                .map(|value| value.to_str().map_err(|_| (StatusCode::BAD_REQUEST, "malformed authentication header token, non ascii string".to_string())))
                .and_then(|value| value)
                .map(|s| s.to_string())
        }

        async {
            let access_token = get_header(&parts.headers, "token")?;
            let user_agent = get_header(&parts.headers,"User-Agent")?;

             match state.session_service.get_by_access_token(&access_token) {
                Some(session) => {
                    if session.user_agent != user_agent {
                        Err((StatusCode::UNAUTHORIZED, "Reauthenticate with access token".to_string()))
                    } else if session.last_refreshed_at + state.session_service.session_timeout() < Instant::now() {
                        state.session_service.delete_by_access_token(&access_token);
                        Err((StatusCode::UNAUTHORIZED, "Reauthenticate with access token".to_string()))
                    } else {
                        let moderator = Moderator::new(session.panel_user)
                            .ok_or((StatusCode::FORBIDDEN, "user lacks required permissions".to_string()))?;
                        _ = state.session_service.refresh_session(access_token);
                        Ok(moderator)
                    }
                }
                None => {
                    let validated = validate_token(&access_token).await
                        .map_err(|_err| {
                            // log errors
                            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error. Retry authentication".to_string())
                        })?
                        .ok_or((StatusCode::UNAUTHORIZED, "Invalid access token, Reauthenticate".to_string()))?;
                    let user = PanelUserService::find_by_id(&state.prod_db, validated.user_id)
                        .await
                        .map_err(|_err| {
                            // log error
                            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error. Retry authentication".to_string())
                        })?
                        .ok_or((StatusCode::FORBIDDEN, "user lacks required permissions".to_string()))?;
                    let moderator = Moderator::new(user.clone())
                        .ok_or((StatusCode::FORBIDDEN, "user lacks required permissions".to_string()))?;
                    state.session_service.create_session(user, access_token, user_agent);
                    Ok(moderator)
                }
            }
        }
    }
}
