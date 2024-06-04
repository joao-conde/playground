use crate::error::InternalError;
use log::LevelFilter;
use std::{env, str::FromStr};

const HOST: &str = "127.0.0.1";
const PORT: u16 = 8080;
const DATABASE_URL: &str = "sqlite://todos.db";
const RUST_LOG: LevelFilter = LevelFilter::Debug;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub db_url: String,
    pub log_level: LevelFilter,
}

impl Config {
    pub fn from_env() -> Result<Self, InternalError> {
        let host = env_var("HOST", HOST.to_string())?;
        let port = env_var("PORT", PORT)?;
        let db_url = env_var("DATABASE_URL", DATABASE_URL.to_string())?;
        let log_level = env_var("RUST_LOG", RUST_LOG)?;
        Ok(Self {
            host,
            port,
            db_url,
            log_level,
        })
    }
}

fn env_var<T: FromStr>(key: &str, default: T) -> Result<T, InternalError> {
    match env::var(key) {
        Err(_) => Ok(default),
        Ok(var) => var
            .parse::<T>()
            .map_err(|_| InternalError::ParseConfig(format!("Invalid {key}"))),
    }
}
