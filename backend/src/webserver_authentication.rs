use crate::twitch::authentication::validate_token;
use crate::axum::AxumState;
pub(crate) use crate::panel_user::Moderator;
use crate::panel_user::{PanelUser, PanelUserService};
use axum::extract::{FromRequestParts, Request, State};
use axum::http::request::Parts;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Result as AxResult;
use std::time::Instant;
use log::error;

const FORBIDDEN: (StatusCode, &'static str) = (StatusCode::FORBIDDEN, "user lacks required permissions");

#[allow(dead_code)]
pub async fn auth_user(State(state): State<AxumState>, request: Request, next: Next) -> AxResult<impl IntoResponse> {
    let (mut request, user) = authenticate_user(state, request).await?;

    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}

pub async fn auth_mod(State(state): State<AxumState>, request: Request, next: Next) -> AxResult<impl IntoResponse> {
    let (mut request, user) = match request
        .extensions()
        .get::<PanelUser>()
        .cloned() {
        Some(user) => (request, user),
        None =>  authenticate_user(state, request).await?,
    };

    let moderator = Moderator::new(user).ok_or(FORBIDDEN)?;
    request.extensions_mut().insert(moderator);
    Ok(next.run(request).await)
}

async fn authenticate_user(state: AxumState, request: Request) -> AxResult<(Request, PanelUser)> {
    let access_token = get_header(request.headers(), "token")?;
    let user_agent = get_header(request.headers(), "User-Agent")?;
    // How are we going to revalidate the auth tokens we use for login/sessions. Maybe twitch has some docs on how you are supposed to do this.
    // if a token gets revoked, realistically we would know after a few hours at most, because currently we require one "action" every 30min to keep the session alive.
    // But it is currently possible to keep a session alive indefinitely.
    //
    // We could maybe check the validity if the last time we checked is more than 15min ago.
    // The trick is to not do that while validating the request, because that would delay the validation, and that's why we have the sessions.
    // But if we allow the request for now anyway, and then in the background check if the token is valid, and remove it for the next request.
    // One additional benefit is that we are not constantly checking the validity for each user, only when they actually do something.

    //we would still want to implement the authentication bypass, although handling those anonymous users for extractors would be a challenge
    match state.l1.session_service.get_by_access_token(&access_token) {
        Some(session) => {
            if session.user_agent != user_agent {
                Err((StatusCode::UNAUTHORIZED, "Reauthenticate with access token"))?
            } else if session.last_refreshed_at + state.l1.session_service.session_timeout() < Instant::now() {
                state.l1.session_service.delete_by_access_token(&access_token);
                Err((StatusCode::UNAUTHORIZED, "Reauthenticate with access token"))?
            } else {
                _ = state.l1.session_service.refresh_session(access_token);
                Ok((request, session.panel_user))
            }
        }
        None => {
            let validated = validate_token(&access_token).await
                .map_err(|err| {
                    error!("Failed to validate session token with twitch: {:?}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error. Retry authentication")
                })?
                .ok_or((StatusCode::UNAUTHORIZED, "Invalid access token, Reauthenticate"))?;
            let user = PanelUserService::find_by_id(&state.l1.prod_db, validated.user_id)
                .await
                .map_err(|err| {
                    error!("Failed to fetch panelUser for userId: {:?}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error. Retry authentication")
                })?
                .ok_or(FORBIDDEN)?;
            state.l1.session_service.create_session(user.clone(), access_token, user_agent);
            Ok((request, user))
        }
    }
}

fn get_header(headers: &HeaderMap<HeaderValue>, key: &str) -> Result<String, (StatusCode, String)> {
    headers
        .get(key).ok_or((StatusCode::UNAUTHORIZED, format!("Missing '{}' authentication header", key)))
        .map(|value| value.to_str().map_err(|_| (StatusCode::BAD_REQUEST, "malformed authentication header token, non ascii string".to_string())))
        .and_then(|value| value)
        .map(|s| s.to_string())
}

impl FromRequestParts<AxumState> for Moderator {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &AxumState) -> Result<Self, Self::Rejection> {
        match parts.extensions.get::<Moderator>() {
            Some(extension) => Ok(extension.clone()),
            None => match parts.extensions.get::<PanelUser>() {
                None => Err(FORBIDDEN),
                Some(u) => Moderator::new(u.clone()).ok_or(FORBIDDEN),
            }
        }
    }
}
