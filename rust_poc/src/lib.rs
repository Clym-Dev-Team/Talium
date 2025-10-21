use crate::oauth_service::OAuthService;
use rand::distr::Alphanumeric;
use rand::Rng;
use std::sync::Arc;
use tokio::runtime::Runtime;

mod oauth_service;
mod authentication_service;
mod oauth_endpoint;
mod axum;

pub(crate) static PANEL_BASE_URL: &'static str = "http://localhost:5173";

pub fn start() {
    let rt  = Runtime::new().unwrap();

    let oauth_service: Arc<OAuthService> = Arc::default();
    let o1 = oauth_service.clone();
    rt.spawn(async { axum::axum(5000, o1).await });

    let state = rand::rng().sample_iter(&Alphanumeric).take(30).map(char::from).collect();
    println!("state: {:?}", state);
    let oauth = oauth_service.new_oauth_request("twitch".to_string(), "account".to_string(), "".to_string(), state);
    println!("oauth: {:?}", oauth);
}