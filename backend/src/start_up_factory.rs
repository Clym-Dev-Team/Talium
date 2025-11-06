use std::sync::{Arc, Mutex, OnceLock, RwLock};
use lazy_static::lazy_static;
use log::warn;
use sqlx::MySqlPool;
use tokio::runtime::Handle;
use crate::commands::command_executor_service::CommandExecutorService;
use crate::db::ProdDB;
use crate::service_oauth::oauth_service::OAuthService;
use crate::session_service::SessionService;
use crate::state::{FullState, L1Arc, L1State, L2Arc, WebserverState};
use crate::twitch::twitch_service::TwitchConfig;
use crate::WebserverConfig;

lazy_static! {
    static ref STARTUPFACTORY: Mutex<StartUpFactory> = Mutex::new(StartUpFactory::new());
}

//TODO the problem with this entire thing is that we have duplicated the state: In the webserver State, and in this struct
// this makes it difficult to call these methods here, because we would need a reference to this, but to call the methods on this, this struct would need a reference to the websever config
// So maybe we just move these Methods onto the webserver config. This shouldn't be more or less safe.
// We could still check if some state would need to be initialized by checking if we can get a lx State Level
// We could still try to initialize the next stages if the config values exist, when we initialize any single stage

//TODO so, remove enum, transfer the Methods on the StartUpFactory1 onto the WebserverState
pub enum StartUpFactory {
    Default(StartStage),
    DB(DbStage),
    Webserver(WebserverStage),
    Twitch(TwitchStage),
    Full(FinalStage),
}

impl StartUpFactory {
    pub fn new() -> StartUpFactory {
        StartUpFactory::Default(StartStage::new())
    }
}
struct StartStage;
struct DbStage(ProdDB);
struct WebserverStage(Arc<WebserverState>);
struct TwitchStage(L2Arc);
struct FinalStage(FullState);

impl StartStage {
    fn new() -> Self {
        StartStage
    }

    pub async fn setup_db(self, connection_str: &str) -> Result<StartUpFactory, (Self, sqlx::Error)> {
        let db_connection = MySqlPool::connect(connection_str).await.map_err(|e| (Self, e))?;
        Ok(StartUpFactory::DB(DbStage(ProdDB::new(db_connection))))
    }
}

impl DbStage {
    pub async fn setup_webserver(self, webserver_config: WebserverConfig) -> Result<StartUpFactory, (Self, ())> {
        let db = self.0;
        let l1 = Arc::new(L1State {
            webserver_config: RwLock::new(webserver_config),
            command_executor_service: CommandExecutorService::new(&db).await,
            oauth_service: OAuthService::new(),
            session_service: SessionService::new(),
            prod_db: db,
        });

        let webserver_state = Arc::new(WebserverState::new(l1.clone()));
        let s = WebserverStage(webserver_state.clone());
        Handle::current().spawn(crate::axum::axum(4771, webserver_state));
        Ok(StartUpFactory::Webserver(s))
    }
}

impl WebserverStage {
    pub async fn setup_twitch(&self, twitch_config: TwitchConfig) -> Result<StartUpFactory, (Self, anyhow::Error)> {
        todo!()
    }
}

struct StartUpFactory1 {
    db: OnceLock<ProdDB>,
    l1: OnceLock<L1Arc>,
    webserver_state: OnceLock<Arc<WebserverState>>,
    l2: OnceLock<L2Arc>,
}



impl StartUpFactory1 {
    pub fn new() -> StartUpFactory1 {
        StartUpFactory1 {
            db: Default::default(),
            l1: Default::default(),
            webserver_state: Default::default(),
            l2: Default::default(),
        }
    }

    //TODO at the end of each method try to setup the next state with values from the default places

    pub async fn setup_db(&self, connection_str: &str) -> Result<(), sqlx::Error> {
        let db_connection = MySqlPool::connect(connection_str).await?;
        if self.db.set(ProdDB::new(db_connection)).is_err() {
            warn!("Tried to initialize DB twice");
        };
        Ok(())
    }

    pub async fn setup_webserver(&self, webserver_config: WebserverConfig) -> Result<(), ()> {
        let db = match self.db.get() {
            None => return Err(()),
            Some(v) => v,
        };
        let l1 = Arc::new(L1State {
            webserver_config: RwLock::new(webserver_config),
            command_executor_service: CommandExecutorService::new(&db).await,
            oauth_service: OAuthService::new(),
            session_service: SessionService::new(),
            prod_db: db.clone(),
        });

        let webserver_state = Arc::new(WebserverState::new(l1.clone()));
        if self.webserver_state.set(webserver_state.clone()).is_err() {
            warn!("Tried to set webserver twice");
        }
        Handle::current().spawn(crate::axum::axum(4771, webserver_state));
        Ok(())
    }

    pub async fn setup_twitch(&self, twitch_config: TwitchConfig) -> Result<(), ()> {
        todo!()
    }

    async fn finalize(&self) {

    }

}