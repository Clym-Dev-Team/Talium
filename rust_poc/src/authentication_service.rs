use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};

/// Add this to the mapping function to add authentication to it
#[derive(Eq, PartialEq)]
pub struct Moderator {
    twitch_user_id: String,
}

#[derive(Eq, PartialEq)]
pub enum User {
    Moderator(Moderator),
}

pub fn authenticate(access_token: Option<String>, user_agent: Option<String>) -> Option<User> {
    if access_token.is_none() {
        return None;
    }
    // if access_token.unwrap() == "moderator" {
        return Some(User::Moderator(Moderator {twitch_user_id: "SOMEUSERID".to_string(), }))
    // }
    // None
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
