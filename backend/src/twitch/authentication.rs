use std::str::FromStr;
use serde_with::DurationSeconds;
use std::time::Duration;
use anyhow::Context;
use axum::http::method::Method;
use axum::http::StatusCode;
use serde::Deserialize;
use serde_with::serde_as;
use url::Url;
use crate::axum::url_encode;
use crate::service_oauth::oauth_service::OAuthService;

#[serde_as]
#[derive(Deserialize)]
pub struct ValidationReturn {
    pub client_id: String,
    pub scopes: Vec<String>,
    pub user_id: String,
    #[serde(rename = "login")]
    pub user_name: String,
    #[serde_as(as = "DurationSeconds")]
    pub expires_in: Duration,
}

/// Validate an accessToken with Twitch. Ok(None) represents a successful validation, but the token being invalid
pub async fn validate_token(access_token: &str) -> anyhow::Result<Option<ValidationReturn>> {
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

pub async fn refresh_token(_access_token: &str) -> anyhow::Result<Option<ValidationReturn>> {
    todo!()
}

pub fn authorization_url(client_id: String, redirect_url: String) -> (Url, String) {
    let scopes = ["channel:bot", "user:bot", "moderator:read:chatters", "moderator:read:moderators", "user:read:chat", "user:manage:chat_color"];
    let state = OAuthService::random_state();
    let query = format!(r#"
        ?response_type=code
        &client_id={}
        &redirect_uri={}
        &scope={}
        &state={}
    "#, client_id, redirect_url, scopes.map(url_encode).join("+"), state);
    let mut url = Url::from_str("https://id.twitch.tv/oauth2/authorize").unwrap();
    url.set_query(Some(query.as_str()));
    println!("url: {:?}", url.as_str());
    (url, state)
}