use crate::axum::AxumState;
use crate::commands::command_controller::{Command, CooldownType, MessagePattern};
use crate::commands::command_executor_service::{TriggerId, TwitchUserPermission};
use crate::commands::template_service::StringTemplate;
use crate::db::ProdDB;
use anyhow::Context;
use num_traits::FromPrimitive;
use sqlx::mysql::MySqlQueryResult;
use sqlx::{query, query_as, MySql, MySqlExecutor, Transaction};
use std::ops::{Deref, DerefMut};

pub struct CommandTable {
    id: String,
    description: String,
    permission: u8,
    global_cooldown_amount: u16,
    global_cooldown_type: u8,
    user_cooldown_amount: u16,
    user_cooldown_type: u8,
    is_auto_generated: bool,
    template_id: Option<String>,
    template: Option<String>,
    message_color: Option<String>,
}

struct PreviewCommand {
    id: String,
    first_pattern: String,
    is_enabled: bool,
    is_visible: bool,
    template: Option<String>,
}

async fn get_search_results(prod_db: &ProdDB, search: &str ) -> anyhow::Result<Vec<PreviewCommand>> {
    let search_str = format!("%{}%", search);
    query_as!(PreviewCommand, r#"
            SELECT pattern as first_pattern, is_enabled as `is_enabled: _`, is_visible as `is_visible: _`, c.id as id, template
            FROM `sys-chat_trigger-patterns` as p
            JOIN talium.`sys-chat_trigger-trigger` as c on p.parent_trigger_id = c.id
            LEFT JOIN `sys-string_templates` as t ON c.template_id = t.id
            WHERE c.id like ?
            OR c.`description` like ?
            OR p.pattern like ?
            OR t.template like ?
            group by c.id
        "#, search_str, search_str, search_str, search_str
    )
        .fetch_all(prod_db.deref())
        .await
        .context("failed to search table commands for PreviewCommand")
}

pub async fn get_all_commands_by_auto_generated(prod_db: &ProdDB, is_auto_generated: bool) -> anyhow::Result<Vec<Command>> {
    let query_res = query_as!(CommandTable, r#"
        SELECT
            c.id,
            c.description,
            c.permission,
            c.global_cooldown_amount,
            c.global_cooldown_type,
            c.user_cooldown_amount,
            c.user_cooldown_type,
            c.is_auto_generated as `is_auto_generated: _`,
            c.template_id,
            t.template,
            t.message_color
        FROM `sys-chat_trigger-trigger` as c
        JOIN `sys-string_templates` as t ON c.template_id = t.id
        WHERE c.is_auto_generated = ?
    "#, is_auto_generated)
        .fetch_all(prod_db.deref())
        .await
        .context("failed to query table for commands by is_auto_generated")?;
    get_patterns(prod_db, query_res).await
}

pub async fn get_all_commands(prod_db: &ProdDB) -> anyhow::Result<Vec<Command>> {
    let query_res = query_as!(CommandTable, r#"
        SELECT
            c.id,
            c.description,
            c.permission,
            c.global_cooldown_amount,
            c.global_cooldown_type,
            c.user_cooldown_amount,
            c.user_cooldown_type,
            c.is_auto_generated as `is_auto_generated: _`,
            c.template_id,
            t.template,
            t.message_color
        FROM `sys-chat_trigger-trigger` as c
        JOIN `sys-string_templates` as t ON c.template_id = t.id
    "#, )
        .fetch_all(prod_db.deref())
        .await
        .context("failed to query all commands")?;
    get_patterns(prod_db, query_res).await
}

pub async fn search_all_commands(prod_db: &ProdDB, search: &str) -> anyhow::Result<Vec<Command>> {
    let search_str = format!("%{}%", search);
    let query_res = query_as!(CommandTable, r#"
        SELECT
            c.id,
            c.description,
            c.permission,
            c.global_cooldown_amount,
            c.global_cooldown_type,
            c.user_cooldown_amount,
            c.user_cooldown_type,
            c.is_auto_generated as `is_auto_generated: _`,
            t.id as template_id,
            t.template,
            t.message_color
        FROM `sys-chat_trigger-patterns` as p
        JOIN talium.`sys-chat_trigger-trigger` as c on p.parent_trigger_id = c.id
        LEFT JOIN `sys-string_templates` as t ON c.template_id = t.id
        WHERE c.id like ?
        OR c.`description` like ?
        OR p.pattern like ?
        OR t.template like ?
        group by c.id
    "#, search_str, search_str, search_str, search_str)
        .fetch_all(prod_db.deref())
        .await
        .context("failed to search table for all commands")?;
    get_patterns(prod_db, query_res).await
}

pub async fn search_all_commands_by_is_auto_generated(prod_db: &ProdDB, search: &str, is_auto_generated: bool) -> anyhow::Result<Vec<Command>> {
    let search_str = format!("%{}%", search);
    let query_res = query_as!(CommandTable, r#"
        SELECT
            c.id,
            c.description,
            c.permission,
            c.global_cooldown_amount,
            c.global_cooldown_type,
            c.user_cooldown_amount,
            c.user_cooldown_type,
            c.is_auto_generated as `is_auto_generated: _`,
            t.id as template_id,
            t.template,
            t.message_color
        FROM `sys-chat_trigger-patterns` as p
        JOIN talium.`sys-chat_trigger-trigger` as c on p.parent_trigger_id = c.id
        LEFT JOIN `sys-string_templates` as t ON c.template_id = t.id
        WHERE c.id like ?
        OR c.`description` like ?
        OR p.pattern like ?
        OR t.template like ?
        AND c.is_auto_generated = ?
        group by c.id
    "#, search_str, search_str, search_str, search_str, is_auto_generated)
        .fetch_all(prod_db.deref())
        .await
        .context("failed to search table for all commands by is_auto_generated")?;
    get_patterns(prod_db, query_res).await
}

pub async fn get_by_id(prod_db: &ProdDB, trigger_id: &TriggerId) -> anyhow::Result<Vec<Command>> {
    let query_res = query_as!(CommandTable, r#"
        SELECT
            c.id,
            c.description,
            c.permission,
            c.global_cooldown_amount,
            c.global_cooldown_type,
            c.user_cooldown_amount,
            c.user_cooldown_type,
            c.is_auto_generated as `is_auto_generated: _`,
            c.template_id,
            t.template,
            t.message_color
        FROM `sys-chat_trigger-trigger` as c
        JOIN `sys-string_templates` as t ON c.template_id = t.id
        WHERE c.id like ?
    "#, trigger_id)
        .fetch_all(prod_db.deref())
        .await
        .context("failed to get by id for all commands")?;
    get_patterns(prod_db, query_res).await
}


async fn get_patterns(prod_db: &ProdDB, d: Vec<CommandTable>) -> anyhow::Result<Vec<Command>> {
    let mut commands = Vec::with_capacity(d.len());
    for command in d.into_iter() {
        let patterns = query_as!(MessagePattern, r#"SELECT
                is_regex as `is_regex: _`,
                is_enabled as `is_enabled: _`,
                is_visible as `is_visible: _`,
                pattern
            FROM `sys-chat_trigger-patterns` WHERE parent_trigger_id = ?"#, &command.id)
            .fetch_all(prod_db.deref())
            .await
            .with_context(|| format!("failed to query patterns for command id {}", command.id))?;
        let template = if command.template_id.is_some() {
            Some(StringTemplate {
                id: command.template_id.unwrap(),
                template: command.template.unwrap_or_default(),
                message_color: command.message_color,
            })
        } else { None };
        let command = Command {
            id: command.id,
            description: command.description,
            patterns: patterns.into_boxed_slice(),
            permission: TwitchUserPermission::from_u8(command.permission).context("Invalid TwitchUserPermission enum variant form db")?,
            is_auto_generated: command.is_auto_generated,
            global_cooldown_amount: command.global_cooldown_amount,
            global_cooldown_type: CooldownType::from_u8(command.global_cooldown_type).context("Invalid CooldownType enum variant form db")?,
            user_cooldown_amount: command.user_cooldown_amount,
            user_cooldown_type: CooldownType::from_u8(command.user_cooldown_type).context("Invalid CooldownType enum variant form db")?,
            template,
        };
        commands.push(command);
    }
    Ok(commands)
}

pub(crate) async fn set_enabled(state: AxumState, trigger_id: &TriggerId, enabled: bool) -> anyhow::Result<Option<()>> {
    let mut transaction = state.prod_db.begin().await?;
    let affected = query!("UPDATE `sys-chat_trigger-patterns` SET is_enabled = ? WHERE parent_trigger_id = ?", enabled, trigger_id)
        .execute(transaction.deref_mut())
        .await
        .context("failed to set enabled on command patterns")?
        .rows_affected();
    if affected == 0 {
        return Ok(None)
    }
    state.command_executor_service.refresh_patterns(&state.prod_db, trigger_id);
    transaction.commit().await?;
    Ok(Some(()))
}

pub(crate) async fn set_visible(prod_db: &ProdDB, trigger_id: &TriggerId, visible: bool) -> anyhow::Result<Option<()>> {
    let affected = query!("UPDATE `sys-chat_trigger-patterns` SET is_visible = ? WHERE parent_trigger_id = ?", visible, trigger_id)
        .execute(prod_db.deref())
        .await
        .context("failed to set visible on command patterns")?
        .rows_affected();
    if affected == 0 {
        return Ok(None)
    }
    Ok(Some(()))
}

pub(crate) enum SaveCommandError {
    DbError(anyhow::Error),
    RegexError(regex::Error),
}

pub(crate) async fn save(state: AxumState, command: &Command) -> Result<(), SaveCommandError> {
    let mut transaction = state.prod_db.begin().await.context("failed to start save command transaction")
        .map_err(|e| SaveCommandError::DbError(e))?;
    save_command(&command, &mut transaction).await
        .map_err(|e| SaveCommandError::DbError(e))?;

    state.command_executor_service.upsert_command(&command)
        .map_err(|e| SaveCommandError::RegexError(e))?;
    transaction.commit().await.context("failed to commit save command transaction")
        .map_err(|e| SaveCommandError::DbError(e))?;
    Ok(())
}

async fn save_command<'a>(command: &Command, transaction: &mut Transaction<'a, MySql>) -> anyhow::Result<()> {
    if let Some(template) = &command.template {
        save_template(transaction.deref_mut(), template).await?;
    }
    for pattern in command.patterns.iter() {
        save_pattern(transaction.deref_mut(), pattern, command.id.as_ref()).await?;
    }
    let template_id = command.template.as_ref().map(|template| template.id.as_str());
    query!(r#"INSERT `sys-chat_trigger-trigger` (id, is_auto_generated, description, global_cooldown_amount, global_cooldown_type, permission, user_cooldown_amount, user_cooldown_type, template_id)
            VALUE (?, ?, ?, ?, ?, ?, ?, ?, ?) ON DUPLICATE KEY UPDATE
            id = ?, is_auto_generated = ?, description = ?, global_cooldown_amount = ?, global_cooldown_type = ?, permission = ?, user_cooldown_amount = ?, user_cooldown_type = ?, template_id = ?
            "#, command.id, command.is_auto_generated, command.description, command.global_cooldown_amount, command.global_cooldown_type, command.permission, command.user_cooldown_amount, command.user_cooldown_type, template_id,
        command.id, command.is_auto_generated, command.description, command.global_cooldown_amount, command.global_cooldown_type, command.permission, command.user_cooldown_amount, command.user_cooldown_type, template_id,
    ).execute(transaction.deref_mut())
        .await
        .context("failed to set visible on command patterns")?;
    Ok(())
}

async fn save_pattern<'a, E: MySqlExecutor<'a>>(prod_db: E, pattern: &MessagePattern, command_id: &str) -> Result<MySqlQueryResult, anyhow::Error> {
    query!(r#"INSERT `sys-chat_trigger-patterns` (pattern, is_enabled, is_regex, is_visible, parent_trigger_id)
                VALUE (?, ?, ?, ? , ?) ON DUPLICATE KEY UPDATE
                pattern = ?, is_enabled = ?, is_regex = ?, is_visible = ?, parent_trigger_id = ?
            "#, pattern.pattern, pattern.is_enabled, pattern.is_regex, pattern.is_visible, command_id,
                pattern.pattern, pattern.is_enabled, pattern.is_regex, pattern.is_visible, command_id
        ).execute(prod_db)
        .await
        .context("failed to save command template")
}

async fn save_template<'a, E: MySqlExecutor<'a>>(prod_db: E, template: &StringTemplate) -> Result<MySqlQueryResult, anyhow::Error> {
    query!("INSERT `sys-string_templates` (id, message_color, template) VALUE (?, ?, ?) ON DUPLICATE KEY UPDATE template = ?, message_color = ?",
            template.id, template.template, template.message_color, template.template, template.message_color)
        .execute(prod_db)
        .await
        .context("failed to save command template")
}

pub(crate) async fn delete_by_id(state: AxumState, trigger_id: &TriggerId) -> anyhow::Result<()> {
    let pool = state.prod_db.deref();
    query!("DELETE FROM `sys-string_templates` WHERE id = (SELECT template_id FROM `sys-chat_trigger-trigger` WHERE id = ?)", trigger_id)
        .execute(pool)
        .await
        .context("failed to delete string template")?;
     query!("DELETE FROM `sys-chat_trigger-trigger` WHERE id = ?", trigger_id)
        .execute(pool)
        .await
        .context("failed to delete command trigger")?;
    state.command_executor_service.remove_command(&trigger_id);
    Ok(())
}

