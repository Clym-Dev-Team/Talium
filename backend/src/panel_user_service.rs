use crate::db::ProdDB;
use crate::panel_user::{PanelUser, UserPermission};
use anyhow::Context;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{query_as, FromRow};
use std::ops::Deref;

#[derive(Default)]
pub struct PanelUserService;

#[derive(Debug, Clone, FromRow)]
pub struct PanelUserEntity {
    pub twitch_user_id: String,
    pub account_creation_time: NaiveDateTime,
}

impl PanelUserService {
    pub async fn find_by_id(connection: &ProdDB, twitch_user_id: String) -> anyhow::Result<Option<PanelUser>> {
        Ok(query_as!(PanelUserEntity, "SELECT * FROM `sys_paneluser` WHERE twitch_user_id = ?", twitch_user_id)
            .fetch_optional(connection.deref())
            .await
            .context("Failed to find user with panel_id")?
            .map(|user| PanelUser {
                twitch_user_id: user.twitch_user_id,
                account_creation_time: user.account_creation_time,
                permissions: UserPermission::Moderator,
            }))
    }
}