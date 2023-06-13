
mod error;

use sqlx::{Postgres, Pool};
pub use self::error::{Error, Result};
use sqlx::postgres::PgPoolOptions;
use crate::config::config;

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config().DB_URL)
        .await
        .map_err(|e| Error::UnableToCreateDbPool(e.to_string()))
}