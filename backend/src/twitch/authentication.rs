use serde_with::DefaultOnNull;
use crate::twitch::twitch_service::OauthCredential;
use anyhow::Context;
use axum::http::method::Method;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DurationSeconds;
use sqlx::types::chrono::Local;
use std::time::Duration;

#[serde_as]
#[derive(Deserialize)]
pub struct ValidationReturn {
    pub client_id: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub scopes: Vec<String>,
    pub user_id: String,
    #[serde(rename = "login")]
    pub user_name: String,
    #[serde_as(as = "DurationSeconds")]
    pub expires_in: Duration,
}

#[serde_as]
#[derive(Deserialize)]
pub struct RefreshTokenReturn {
    pub access_token: String,
    pub refresh_token: String,
    #[serde_as(as = "DurationSeconds")]
    pub expires_in: Duration,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub scopes: Vec<String>,
    pub token_type: String,
}

impl Into<OauthCredential> for RefreshTokenReturn {
    fn into(self) -> OauthCredential {
       OauthCredential {
           refresh_token: self.refresh_token,
           scopes: self.scopes,
           access_token: self.access_token,
           expires_at: Local::now() + self.expires_in
       }
    }
}

/// Validate an accessToken with Twitch. Ok(None) represents a successful validation, but the token being invalid
pub async fn validate_token(access_token: impl AsRef<str>) -> anyhow::Result<Option<ValidationReturn>> {
    // prob move client into app state
    let response = reqwest::Client::builder()
        .build()
        .context("Failed to build client builder for twitch access token validation")?
        .request(Method::GET, "https://id.twitch.tv/oauth2/validate")
        .header("Authorization", format!("Bearer {}", access_token.as_ref()))
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

pub async fn refresh_token(refresh_token: impl AsRef<str>, client_id: impl AsRef<str>, client_secret: impl AsRef<str>) -> anyhow::Result<Option<RefreshTokenReturn>> {
    #[derive(Serialize)]
    struct RefreshRequest<'a> {
        client_id: &'a str,
        client_secret: &'a str,
        grant_type: &'a str,
        refresh_token: &'a str,
    }
    let response = reqwest::Client::builder()
        .build()
        .context("Failed to build client builder for refreshing twitch oauth token")?
        .request(Method::POST, "https://id.twitch.tv/oauth2/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        //TODO body should be x-www-urlencoded
        .json(&RefreshRequest {
            refresh_token: refresh_token.as_ref(),
            client_id: client_id.as_ref(),
            client_secret: client_secret.as_ref(),
            grant_type: "refresh_token",
        })
        .send()
        .await
        .context("Failed to send request to validate oauth token")?;
    if response.status() == StatusCode::BAD_REQUEST {
        return Ok(None)
    }
    let response_body = response.text()
        .await
        .context("Failed to decode response body from refreshing twitch oauth token")?;
    let validation_return: RefreshTokenReturn = serde_json::from_str(&response_body)
        .context("Failed to deserialize response body from refreshing twitch oauth token")
        .context("Original response body: ".to_string() + &response_body)?;
    Ok(Some(validation_return))
}
