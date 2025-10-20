use crate::authentication_service::{authenticate, Moderator, User};
use crate::oauth_service::{OAuthService, OauthReturnError};
use crate::PANEL_BASE_URL;
use axum::body::Body;
use axum::extract::{FromRequestParts, Path, Query, State};
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::routing::{any, get};
use axum::{Json, Router};
use rocket::serde::Deserialize;
use std::sync::Arc;
use url::form_urlencoded;

type AxumState = Arc<OAuthService>;

pub async fn axum(on_port: u16, oauth_service: AxumState) {
    let app = Router::new()
        .route("/auth/{service}", any(receive_oauth))
        .route("/setup/auth/list", get(list_oauth))
        .with_state(oauth_service);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", on_port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn url_encode<'a>(input: &str) -> String {
    form_urlencoded::byte_serialize(input.as_bytes()).collect()
}

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
struct ReceiveOAuthQuery {
    state: String,
    scope: Option<String>,
    code: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

async fn receive_oauth(
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
    Body::new(format!("{PANEL_BASE_URL}{}", match state.return_oauth(service, query.state, query.scope.unwrap(), query.code.unwrap()) {
        Ok(()) => format!("{}?success=true", PANEL_BASE_URL),
        Err(OauthReturnError::NotRequested) => format!("{}?success=false&error={}", PANEL_BASE_URL, url_encode("This oauth was never requested from the bot")),
        Err(OauthReturnError::ReturnChannelClosed) => {
            eprintln!("Failed to process auth code, return channel was closed");
            format!("{}?success=false&error={}", PANEL_BASE_URL, url_encode("Could not process auth code. Internal server error"))
        }
    }))
}

async fn list_oauth(_: Moderator, State(state): State<AxumState>) -> impl IntoResponse {
    Json(state.get_active_requests())
}

pub enum AuthFailure {
    AuthenticationFailure,
    AuthorizationFailure,
}
impl IntoResponse for AuthFailure {
    fn into_response(self) -> Response {
        // this is not the correct status code
        Body::new(match self {
            AuthFailure::AuthenticationFailure => "Authentication failure",
            AuthFailure::AuthorizationFailure => "Authorization failure",
        }.to_string()).into_response()
    }
}

impl<S> FromRequestParts<S> for Moderator
where
    S: Send + Sync,
{
    type Rejection = AuthFailure;

    fn from_request_parts(parts: &mut Parts, _: &S) -> impl Future<Output=Result<Self, Self::Rejection>> + Send {
        async {
            println!("headers: {:?}", parts.headers);
            let access_token = parts.headers.get("token").map(|value| value.to_str().unwrap().to_string());
            let user_agent = parts.headers.get("User-Agent").map(|value| value.to_str().unwrap().to_string());
            let user = authenticate(access_token, user_agent).ok_or(AuthFailure::AuthenticationFailure)?;
            #[allow(unreachable_patterns)]
            match user {
                User::Moderator(m) => Ok(m),
                _ => Err(AuthFailure::AuthorizationFailure)
            }
        }
    }
}
