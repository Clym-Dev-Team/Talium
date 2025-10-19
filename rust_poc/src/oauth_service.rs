use serde::Serialize;
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;

struct OauthRequest {
    state: String,
    url: String,
    account_name: String,
    service_name: String,
    code: Option<String>,
    return_channel: Sender<String>
}

static ACTIVE_REQUESTS: Mutex<Vec<OauthRequest>> = Mutex::new(Vec::new());

pub fn new_auth_request(service_name: String, account_name: String, authorization_url: String, state: String) -> Option<String> {
    let (sender, receiver) = channel();
    let request = OauthRequest {
        state,
        url: authorization_url,
        account_name,
        service_name,
        code: None,
        return_channel: sender
    };
    // timeout after some time
    // handle this unwrap
    ACTIVE_REQUESTS.lock().unwrap().push(request);
    receiver.recv().ok()
}

pub enum OauthReturnError {
    NotRequested,
    ReturnChannelClosed
}

#[allow(unused_variables)]
pub fn return_oauth(service: String, state: String, scope: String, code: String) -> Result<(), OauthReturnError> {
    let mut guard = ACTIVE_REQUESTS
        .lock()
        .unwrap();
    let (index, request) = guard
        .iter()
        .enumerate()
        .filter(|(_,r)| r.service_name == service && r.state == state)
        .next()
        .ok_or(OauthReturnError::NotRequested)?;

    // check if scopes are the same

    request.return_channel.send(code).map_err(|_| OauthReturnError::ReturnChannelClosed)?;

    guard.remove(index);
    Ok(())
}

#[derive(Serialize)]
pub struct OauthRequestDisplay {
    account_name: String,
    service_name: String,
    url: String,
}

pub fn get_active_requests() -> Vec<OauthRequestDisplay> {
    let mut guard = ACTIVE_REQUESTS.lock().unwrap();
    guard.iter().map(|r| {
        OauthRequestDisplay {
            service_name: r.service_name.clone(),
            account_name: r.account_name.clone(),
            url: r.url.clone(),
        }
    }).collect()
}