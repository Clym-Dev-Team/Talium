use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub enum UserPermission {
    Moderator,
}

#[derive(Debug, Clone)]
pub struct PanelUser {
    pub twitch_user_id: String,
    pub account_creation_time: NaiveDateTime,
    pub permissions: UserPermission,
}

/// Add this to the mapping function to add authentication to it
pub struct Moderator(PanelUser);

impl Moderator {
    pub fn new(user: PanelUser) -> Option<Moderator> {
        match user.permissions {
            UserPermission::Moderator => Some(Moderator(user)),
        }
    }
}
