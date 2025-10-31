use crate::db::ProdDB;
use crate::service_oauth::oauth_endpoint::get_redirect_url;
use crate::state::L1State;
use crate::twitch::authentication::{authorization_url, refresh_token, validate_token};
use anyhow::{anyhow, Context};
use asknothingx2_util::oauth::{AccessToken, ClientId};
use serde::Deserialize;
use sqlx::types::chrono::{DateTime, Local};
use std::sync::Arc;
use std::time::Instant;
use tokio::runtime::Handle;
use tokio::sync::{Mutex, RwLock};
use twitch_highway::TwitchAPI;

#[derive(Default, Deserialize, Clone)]
pub(crate) struct OauthCredential {
    pub access_token: String,
    pub refresh_token: String,
    pub scopes: Vec<String>,
    pub expires_at: DateTime<Local>,
}

#[derive(Default, Clone, Deserialize)]
pub(crate) struct TwitchConfig {
    pub channel_name: String,
    pub chat_account_name: String,
    pub send_to: String,
    pub client_id: String,
    pub client_secret: String,
}

struct TwitchCredentialStatus {
    valid_checked_at: Instant,
    was_valid_at_check: bool,
    credential: OauthCredential,
}

pub struct TwitchService {
    pub(super) twitch_api: Arc<RwLock<TwitchAPI>>,
    pub(super) token_validation: Arc<Mutex<TwitchCredentialStatus>>,
    pub(super) config: TwitchConfig,
    pub(super) l1: Arc<L1State>,
}

impl TwitchService {
    pub async fn new(l1: Arc<L1State>, twitch_config: TwitchConfig) -> Result<TwitchService, anyhow::Error> {
        //TODO get credentials from db
        let cred_from_db: OauthCredential = OauthCredential {
            access_token: std::env::var("TWITCH_ACCESS_TOKEN").unwrap(),
            refresh_token: std::env::var("TWITCH_REFRESH_TOKEN").unwrap(),
            scopes: vec![],
            expires_at: Default::default(),
        };
        let oauth = match Self::check_or_get_oauth(&cred_from_db, &l1.prod_db, &twitch_config).await {
            Ok(oauth) => oauth,
            Err(_e) => {
                // log err
                TwitchCredentialStatus {
                    was_valid_at_check: true,
                    valid_checked_at: Instant::now(),
                    credential: Self::request_new_oauth(&l1, twitch_config.clone()).await
                }
            },
        };
        let api = TwitchAPI::new(
            AccessToken::from(oauth.credential.access_token.as_str()),
            ClientId::from(twitch_config.client_id.as_str()),
        );

        //TODO setup periodic token refresher
        //TODO refresh and save token on shutdown

        Ok(TwitchService {
            twitch_api: Arc::new(RwLock::new(api)),
            token_validation: Arc::new(Mutex::new(oauth)),
            config: twitch_config,
            l1,
        })
    }
}

impl TwitchService {
    #[inline(always)]
    pub(super) async fn handle_401_and_retry<R, F, Fut>(&self, req: F) -> Result<R, anyhow::Error> where
        F: Fn() -> Fut,
        Fut: Future<Output=Result<R, twitch_highway::Error>>
    {
        match req().await {
            Err(e) if e.is_api() && e.message().is_some_and(|t1| t1.starts_with("HTTP 401")) => {
                let mut cred_lock = self.token_validation.lock().await;
                if cred_lock.valid_checked_at.elapsed() > std::time::Duration::from_secs(15 * 60) {
                    match Self::check_or_get_oauth(&cred_lock.credential, &self.l1.prod_db, &self.config).await {
                        Ok(d) => {
                            let mut api_lock = self.twitch_api.write().await;
                            *api_lock = api_lock.clone().set_access_token(AccessToken::from(d.credential.access_token.clone()));
                            *cred_lock = d;
                        },
                        Err(e) => {
                            cred_lock.valid_checked_at = Instant::now();
                            cred_lock.was_valid_at_check = false;
                            let config = self.config.clone();
                            let l1 = self.l1.clone();
                            let twitch_api = self.twitch_api.clone();
                            let token_validation = self.token_validation.clone();
                            Handle::current().spawn(async move {
                                let credential = TwitchService::request_new_oauth(&l1, config).await;
                                {
                                    let mut api_lock = twitch_api.write().await;
                                    *api_lock = api_lock.clone().set_access_token(AccessToken::from(credential.access_token.clone()));
                                }
                                {
                                    let mut token_lock = token_validation.lock().await;
                                    token_lock.valid_checked_at = Instant::now();
                                    token_lock.was_valid_at_check = true;
                                    token_lock.credential = credential;
                                }
                                //log
                            });
                            return Err(e).context("Credentials Bad, not retrying");
                        }
                    };
                };
                if !cred_lock.was_valid_at_check {
                    return Err(anyhow!("Could not get new oauth token, bad credentials, needs reauthentication"));
                }
                drop(cred_lock);
                req().await.context("Retry also failed")
            },
            Err(e) => Err(e.into()),
            Ok(r) => Ok(r),
        }
    }

    async fn check_or_get_oauth(cred: &OauthCredential, _db: &ProdDB, twitch_config: &TwitchConfig) -> Result<TwitchCredentialStatus, anyhow::Error> {
        const FINAL_ERROR: &str = "Could not get new oauth token, bad credentials, needs reauthentication";
        // is first thread, do validation/refreshing
        let mut errors = vec![];
        match validate_token(cred.access_token.as_str()).await {
            Ok(Some(_validation)) => {
                return Ok(TwitchCredentialStatus {
                    valid_checked_at: Instant::now(),
                    was_valid_at_check: true,
                    credential: cred.clone(),
                })
            },
            Err(e) => errors.push(e),
            Ok(None) => {}
        };
        match refresh_token(cred.refresh_token.as_str(), twitch_config.client_id.as_str(), twitch_config.client_secret.as_str()).await {
            Ok(Some(refreshed)) => {
                //TODO save to db
                return Ok(TwitchCredentialStatus {
                    valid_checked_at: Instant::now(),
                    was_valid_at_check: true,
                    credential: refreshed.into(),
                })
            }
            Err(e) => errors.push(e),
            Ok(None) => {}
        }
        //TODO add errors
        Err(anyhow!(FINAL_ERROR))
    }

    async fn request_new_oauth(l1: &L1State, twitch_config: TwitchConfig) -> OauthCredential {
        let (url, state) = authorization_url(twitch_config.client_id.as_str(), get_redirect_url("".to_string(), "twitch"));
        //TODO make the url kida a builder, that gets evaluated when the api is actually called
        let code = l1.oauth_service.new_oauth_request("twitch", twitch_config.chat_account_name.as_str(), url, state);

        let _refreshed = refresh_token(code, twitch_config.client_id.as_str(), twitch_config.client_secret.as_str()).await;
        //TODO save to db
        todo!()
    }
}
