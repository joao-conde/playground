pub mod routes;

mod db;
mod error;

use sqlx::SqlitePool;

pub struct AppData {
    pub db_pool: SqlitePool,
}
