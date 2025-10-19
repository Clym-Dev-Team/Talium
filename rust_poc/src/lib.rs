use rand::distr::Alphanumeric;
use rand::Rng;
use tokio::runtime::Runtime;

mod servers;
mod oauth_service;

pub(crate) static PANEL_BASE_URL: &'static str = "http://localhost:5173";

pub fn start() {
    let rt  = Runtime::new().unwrap();

    rt.spawn(async { servers::axum::axum(5000).await });

    let state = rand::rng().sample_iter(&Alphanumeric).take(30).map(char::from).collect();
    println!("state: {:?}", state);
    let oauth = oauth_service::new_auth_request("twitch".to_string(), "account".to_string(), "".to_string(), state);
    println!("oauth: {:?}", oauth);
}