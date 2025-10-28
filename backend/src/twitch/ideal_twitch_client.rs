use reqwest::StatusCode;

struct Helix;
struct User;

enum SendChatError {
    BadRequest(Box<str>),
    Unauthorized(Box<str>),
    Forbidden(Box<str>),
    UnprocessableEntity(Box<str>),
    UnknownStatus(Box<(StatusCode, Box<str>)>),
}

impl Helix {
    pub async fn send_chat_message(broadcaster_id: impl AsRef<str>, sender_id: impl AsRef<str>, message: impl AsRef<str>, reply_parent_message_id: Option<impl AsRef<str>>, for_source_only: bool) -> Result<Vec<User>, SendChatError> {
        todo!()
    }
}