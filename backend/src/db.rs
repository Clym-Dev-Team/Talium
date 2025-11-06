use sqlx::{MySql, Pool};
use std::ops::Deref;

#[derive(Clone)]
pub(crate) struct ProdDB(Pool<MySql>);

impl ProdDB {
    pub(crate) fn new(pool: Pool<MySql>) -> ProdDB {
        ProdDB(pool)
    }
}

impl Deref for ProdDB {
    type Target = Pool<MySql>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}