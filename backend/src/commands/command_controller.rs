use crate::axum::AxumState;
use axum::extract::{Path, Query, State};
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use sqlx::{query_as, FromRow};
use crate::commands::command_executor_service::{ChatCooldown, TwitchUserPermission};
use crate::commands::template_service::StringTemplate;

#[derive(Deserialize)]
struct MessagePattern {
    pattern: String,
    is_regex: bool,
    is_visible: bool,
    is_enabled: bool,
}

#[derive(Deserialize, FromRow)]
pub struct CommandDTO {
    id: String,
    description: String,
    #[sqlx(skip)]
    patterns: Box<[MessagePattern]>,
    permission: TwitchUserPermission,
    #[sqlx(flatten)]
    user_cooldown: ChatCooldown,
    global_cooldown: ChatCooldown,
    is_auto_generated: bool,
    template: StringTemplate,
}

pub async fn get_all_user_commands(
    State(state): State<AxumState>,
    Query(search): Query<String>,
) -> impl IntoResponse {
    // query_as::<_, Command>("SELECT * FROM `sys-chat_trigger-trigger` as t")
}

pub async fn get_all_commands(
    State(state): State<AxumState>,
    Query(search): Query<String>,
) -> impl IntoResponse {}

pub async fn get_by_trigger_id(
    State(state): State<AxumState>,
    Path(triggerId): Path<String>,
) -> impl IntoResponse {}

pub async fn set_enabled(
    State(state): State<AxumState>,
    Query(triggerId): Query<String>,
    body: String
) -> impl IntoResponse {}

pub async fn set_visible(
    State(state): State<AxumState>,
    Query(triggerId): Query<String>,
    body: String
) -> impl IntoResponse {}

pub async fn save(
    State(state): State<AxumState>,
    Json(to_save): Json<CommandDTO>,
) -> impl IntoResponse {}

pub async fn delete_by_id(
    State(state): State<AxumState>,
    Path(triggerId): Path<String>,
) -> impl IntoResponse {}
