use crate::panel_user::PanelUser;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Session {
    // we really shouldn't use the access token as the session token
    pub access_token: String,
    pub user_agent: String,
    pub last_refreshed_at: Instant,
    pub panel_user: PanelUser,
}

#[derive(Debug)]
pub struct SessionService {
    sessions: RwLock<Vec<Session>>,
    session_timeout: Duration,
}

impl Default for SessionService {
    fn default() -> Self {
        Self {
            sessions: RwLock::default(),
            session_timeout: Duration::from_secs(60 * 30),
        }
    }
}

impl SessionService {
    pub fn new() -> Self {
        // instead get session timeout from database or something
        Self::default()
    }

    pub fn session_timeout(&self) -> Duration {
        self.session_timeout
    }

    pub fn create_session(&self, panel_user: PanelUser, access_token: String, user_agent: String) {
        let mut wg = self.write_guard();
        wg.push(Session {
            user_agent,
            access_token,
            panel_user,
            last_refreshed_at: Instant::now(),
        })
    }

    pub fn refresh_session(&self, access_token: String) -> Option<()>{
        let mut wg = self.write_guard();
        let index = wg.iter().enumerate().find(|(_, s)| s.access_token == access_token)?.0;
        wg.get_mut(index).unwrap().last_refreshed_at = Instant::now();
        Some(())
    }

    pub fn get_by_access_token(&self, access_token: &str) -> Option<Session> {
        let rg = self.read_guard();
        rg.iter().find(|s| s.access_token == access_token).cloned()
    }
    pub fn get_by_user_id(&self, twitch_user_id: &str) -> Option<Session> {
        let rg = self.read_guard();
        rg.iter().find(|s| s.panel_user.twitch_user_id == twitch_user_id).cloned()
    }

    pub fn delete_by_access_token(&self, access_token: &str) {
        let mut wg = self.write_guard();
        wg.retain(|s| s.access_token != access_token);
    }

    fn read_guard(&self) -> RwLockReadGuard<'_, Vec<Session>> {
        match self.sessions.read() {
            Ok(guard) => guard,
            Err(poisoned) => {
                self.sessions.clear_poison();
                drop(poisoned);
                let mut wg = self.sessions.write().unwrap();
                *wg = vec![];
                self.sessions.read().unwrap()
            },
        }
    }

    fn write_guard(&self) -> RwLockWriteGuard<'_, Vec<Session>> {
        match self.sessions.write() {
            Ok(guard) => guard,
            Err(poisoned) => {
                self.sessions.clear_poison();
                drop(poisoned);
                let mut wg = self.sessions.write().unwrap();
                *wg = vec![];
                wg
            },
        }
    }
}