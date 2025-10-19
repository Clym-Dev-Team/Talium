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
