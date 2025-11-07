use crate::axum::AxumState;
use crate::commands::command_executor_service::TwitchUserPermission;
use crate::commands::command_repo;
use crate::commands::template_service::StringTemplate;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Result as AxResult;
use axum::{debug_handler, Json};
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
use log::{error, warn};
use crate::commands::command_repo::SaveCommandError;
use crate::state::L1Arc;

#[derive(Deserialize, Serialize)]
pub struct MessagePattern {
    pub pattern: String,
    pub is_regex: bool,
    pub is_visible: bool,
    pub is_enabled: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Command {
    pub id: String,
    pub description: String,
    pub patterns: Box<[MessagePattern]>,
    pub permission: TwitchUserPermission,
    pub is_auto_generated: bool,
    pub global_cooldown_amount: u16,
    pub global_cooldown_type: CooldownType,
    pub user_cooldown_amount: u16,
    pub user_cooldown_type: CooldownType,
    pub template: Option<StringTemplate>,
}

/// Do not modify order, this will break the type in the db
#[derive(Deserialize, Serialize, Type, FromPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum CooldownType {
    SECONDS = 0,
    MESSAGES = 1,
}

#[derive(Deserialize)]
pub struct SearchQuery{
    search: Option<String>,
}

//TODO use new return type that only returns enough information to render commands table, request the entire object on edit open
pub async fn get_all_user_commands(
    l1: L1Arc,
    Query(search): Query<SearchQuery>,
) -> AxResult<impl IntoResponse> {
    let commands = match search.search.as_deref() {
        Some("") | None => command_repo::get_all_commands_by_auto_generated(&l1.prod_db, false).await,
        Some(search) => command_repo::search_all_commands_by_is_auto_generated(&l1.prod_db, search, false).await,
    }.map_err(|e| {
        error!("{:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(commands))
}

#[debug_handler(state = AxumState)]
pub async fn get_all_commands(
    l1: L1Arc,
    Query(search): Query<SearchQuery>,
) -> AxResult<impl IntoResponse> {
    let commands = match search.search.as_deref() {
        Some("") | None => command_repo::get_all_commands(&l1.prod_db).await,
        Some(search) => command_repo::search_all_commands(&l1.prod_db, search).await,
    }.map_err(|e| {
        error!("{:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(commands))
}

pub async fn get_by_trigger_id(
    l1: L1Arc,
    Path(trigger_id): Path<String>,
) -> AxResult<impl IntoResponse> {
    Ok(Json(command_repo::get_by_id(&l1.prod_db, trigger_id.as_ref())
        .await
        .map_err(|e| {
            error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?))
}

pub async fn set_enabled(
    l1: L1Arc,
    Path(trigger_id): Path<String>,
    body: String
) -> AxResult<impl IntoResponse> {
    //TODO move into query parameter
    let enabled = bool::from_str(body.as_ref()).map_err(|_| StatusCode::BAD_REQUEST)?;
    command_repo::set_enabled(l1.as_ref(), trigger_id.as_ref(), enabled)
        .await
        .map_err(|e| {
            error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(())
}

pub async fn set_visible(
    l1: L1Arc,
    Path(trigger_id): Path<String>,
    body: String
) -> AxResult<impl IntoResponse> {
    let visible = bool::from_str(body.as_str()).map_err(|_| StatusCode::BAD_REQUEST)?;
    command_repo::set_visible(&l1.prod_db, trigger_id.as_str(), visible)
        .await
        .map_err(|e| {
            error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(())
}

pub async fn save(
    l1: L1Arc,
    Json(to_save): Json<Command>,
) -> impl IntoResponse {
    match command_repo::save(l1.as_ref(), &to_save).await {
        Ok(()) => StatusCode::OK,
        Err(SaveCommandError::DbError(db_error)) => {
            error!("Error saving command from panel: {:?}", db_error);
            StatusCode::INTERNAL_SERVER_ERROR
        }
        Err(SaveCommandError::RegexError(r)) => {
            warn!("User tried to save invalid regex: {:?}", r);
            //TODO figure out how to return this error to the user
            StatusCode::BAD_REQUEST
        }
    }
}

pub async fn delete_by_id(
    l1: L1Arc,
    Path(trigger_id): Path<String>,
) -> AxResult<impl IntoResponse> {
    command_repo::delete_by_id(l1.as_ref(), trigger_id.as_ref()).await.map_err(|e| {
        error!("Unable to execute command deletion: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(())
}

