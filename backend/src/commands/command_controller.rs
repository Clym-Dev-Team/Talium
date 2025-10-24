use crate::axum::AxumState;
use crate::commands::command_executor_service::TwitchUserPermission;
use crate::commands::command_repo;
use crate::commands::template_service::StringTemplate;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Result as AxResult;
use axum::Json;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

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
#[derive(Deserialize, Serialize, Type, FromPrimitive)]
#[repr(u8)]
pub enum CooldownType {
    Seconds = 0,
    Messages = 1,
}

//TODO use new return type that only returns enough information to render commands table, request the entire object on edit open
//TODO update command_executor
pub async fn get_all_user_commands(
    State(state): State<AxumState>,
    Query(search): Query<String>,
) -> AxResult<impl IntoResponse> {
    let commands = match search.is_empty() {
        true => command_repo::get_all_commands_by_auto_generated(&state.prod_db, false).await,
        false => command_repo::search_all_commands_by_is_auto_generated(&state.prod_db, search.as_ref(), false).await
    }.map_err(|_| {
        // log
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(commands))
}

pub async fn get_all_commands(
    State(state): State<AxumState>,
    Query(search): Query<String>,
) -> AxResult<impl IntoResponse> {
    Ok(Json(command_repo::search_all_commands(&state.prod_db, search.as_ref())
        .await
        .map_err(|_| {
            // log
            StatusCode::INTERNAL_SERVER_ERROR
        })?))
}

pub async fn get_by_trigger_id(
    State(state): State<AxumState>,
    Path(trigger_id): Path<String>,
) -> AxResult<impl IntoResponse> {
    Ok(Json(command_repo::get_by_id(&state.prod_db, trigger_id.as_ref())
        .await
        .map_err(|_| {
            // log
            StatusCode::INTERNAL_SERVER_ERROR
        })?))
}

pub async fn set_enabled(
    State(state): State<AxumState>,
    Query(trigger_id): Query<String>,
    body: String
) -> AxResult<impl IntoResponse> {
    //TODO move into query parameter
    let enabled = bool::from_str(body.as_ref()).map_err(|_| StatusCode::BAD_REQUEST)?;
    command_repo::set_enabled(&state.prod_db, trigger_id.as_ref(), enabled)
        .await
        .map_err(|_| {
            // log
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(())
}

pub async fn set_visible(
    State(state): State<AxumState>,
    Query(trigger_id): Query<String>,
    body: String
) -> AxResult<impl IntoResponse> {
    let visible = bool::from_str(body.as_ref()).map_err(|_| StatusCode::BAD_REQUEST)?;
    command_repo::set_visible(&state.prod_db, trigger_id.as_ref(), visible)
        .await
        .map_err(|_| {
            // log
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(())
}

pub async fn save(
    State(state): State<AxumState>,
    Json(to_save): Json<Command>,
) -> AxResult<impl IntoResponse> {
    command_repo::save(&state.prod_db, &to_save).await.map_err(|_| {
        // log
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(())
}

pub async fn delete_by_id(
    State(state): State<AxumState>,
    Path(trigger_id): Path<String>,
) -> AxResult<impl IntoResponse> {
    command_repo::delete_by_id(&state.prod_db, trigger_id.as_ref()).await.map_err(|_| {
        // log
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(())
}

