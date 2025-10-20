use crate::authentication_service::{authenticate, Moderator, User};
use crate::oauth_service::OAuthService;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{get, routes, Request, State};
use std::sync::Arc;

pub async fn rocket(on_port: u16, oauth_service: Arc<OAuthService>) {
    let server = rocket::build()
        .mount("/", routes![])
        .manage(oauth_service)
        .ignite()
        .await
        .unwrap()
        .launch()
        .await;
}

#[get("/setup/auth/list")]
fn list_oauth(
    // _m: Moderator,
    oauth_service: &State<Arc<OAuthService>>
) -> String {
    "test".to_string()
}


#[get("/auth/<service>")]
async fn receive_oauth(
    service: &str,
    oauth_service: &State<Arc<OAuthService>>
) {

}

#[derive(Debug)]
pub enum AuthFailure {
    AuthenticationFailure,
    AuthorizationFailure,
}

// impl<'r> FromRequest<'r> for Moderator {
//     type Error = AuthFailure;
//
//     async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//         let token = request.headers().get_one("token").map(str::to_string);
//         let user_agent = request.headers().get_one("user_agent").map(str::to_string);
//         let user = authenticate(token, user_agent);
//
//         #[allow(unreachable_patterns)]
//         match user {
//             None => Outcome::Error((Status::Unauthorized, AuthFailure::AuthenticationFailure)),
//             Some(User::Moderator(m)) => Outcome::Success(m),
//             Some(_) => Outcome::Error((Status::Forbidden, AuthFailure::AuthorizationFailure)),
//         }
//     }
// }