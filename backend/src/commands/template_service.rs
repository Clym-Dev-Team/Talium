use crate::commands::command_executor_service::TriggerId;
use crate::db::ProdDB;
use anyhow::Context;
use sqlx::{query_as, FromRow};
use std::ops::Deref;
use serde::{Deserialize, Serialize};

pub struct TemplateService;

#[derive(Deserialize, Serialize, FromRow)]
pub struct StringTemplate {
    pub id: String,
    pub template: String,
    pub message_color: Option<String>,
}

impl TemplateService {
    pub async fn get_template_by_trigger_id(db: &ProdDB, id: &TriggerId) -> anyhow::Result<Option<StringTemplate>> {
        query_as!(StringTemplate, "SELECT * FROM `sys-string_templates` WHERE id = ?", id)
            .fetch_optional(db.deref())
            .await
            .context("Failed to get template by trigger id")
    }
}