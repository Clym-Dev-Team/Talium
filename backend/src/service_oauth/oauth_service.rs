use serde::Serialize;
use std::sync::mpsc::{channel, Sender};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use rand::distr::Alphanumeric;
use rand::Rng;

struct OauthRequest {
    state: String,
    url: String,
    account_name: String,
    service_name: String,
    return_channel: Sender<String>,
}

#[derive(Default)]
pub struct OAuthService {
    active_requests: RwLock<Vec<OauthRequest>>,
}

pub enum OauthReturnError {
    NotRequested,
    ReturnChannelClosed,
}

#[derive(Serialize)]
pub struct OauthRequestDisplay {
    account_name: String,
    service_name: String,
    url: String,
}

impl OAuthService {
    pub fn new_oauth_request(&self, service_name: impl Into<String>, account_name: impl Into<String>, authorization_url: impl Into<String>, state: impl Into<String>) -> String {
        // if the RwLock for the requests gets poisoned, the sender for our channel will get dropped
        // which will lead to our .recv quitting. In that case we will just add our request to the
        // reinitialized request list and receive again
        let service_name = service_name.into();
        let account_name = account_name.into();
        let authorization_url = authorization_url.into();
        let state = state.into();
        let mut res = None;
        while res.is_none() {
            let (sender, receiver) = channel();
            let request = OauthRequest {
                state: state.clone(),
                url: authorization_url.clone(),
                account_name: account_name.clone(),
                service_name: service_name.clone(),
                return_channel: sender,
            };
            {
                let mut guard = self.write_lock();
                guard.push(request);
            }
            res = receiver.recv().ok();
        }
        res.unwrap()
    }

    #[allow(unused_variables)]
    pub fn return_oauth(&self, service: String, state: String, scope: String, code: String) -> Result<(), OauthReturnError> {
        let mut guard = self.write_lock();
        let (index, request) = guard
            .iter()
            .enumerate()
            .filter(|(_, r)| r.service_name == service && r.state == state)
            .next()
            .ok_or(OauthReturnError::NotRequested)?;

        // check if scopes are the same

        // ignore error, because if sender is orphan, then we should remove it
        let res = request.return_channel.send(code);

        guard.remove(index);
        drop(guard);

        res.map_err(|_| OauthReturnError::ReturnChannelClosed)
    }

    pub fn random_state() -> String {
        rand::rng().sample_iter(&Alphanumeric).take(32).map(char::from).collect()
    }

    #[cfg(test)]
    pub fn corrupt_lock(&self) {
        let _guard = self.active_requests.write().unwrap();
        panic!("Purposely panic while holding guard");
    }

    pub fn get_active_requests(&self) -> Vec<OauthRequestDisplay> {
        self.read_lock()
            .iter()
            .map(|r| OauthRequestDisplay {
                service_name: r.service_name.clone(),
                account_name: r.account_name.clone(),
                url: r.url.clone(),
            })
            .collect()
    }

    fn read_lock(&self) -> RwLockReadGuard<'_, Vec<OauthRequest>> {
        match self.active_requests.read().ok() {
            Some(guard) => guard,
            None => {
                self.active_requests.clear_poison();
                {
                    let mut wg = self.active_requests.write().unwrap();
                    *wg = vec![];
                }
                self.active_requests.read().unwrap()
            }
        }
    }

    fn write_lock(&self) -> RwLockWriteGuard<'_, Vec<OauthRequest>> {
        match self.active_requests.write() {
            Ok(guard) => guard,
            Err(e) => {
                self.active_requests.clear_poison();
                let mut wg = e.into_inner();
                *wg = vec![];
                wg
            }
        }
    }
}
