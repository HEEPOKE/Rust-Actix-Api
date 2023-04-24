use dotenvy::dotenv;
use std::env;

use dotenv::dotenv;
use std::env;

pub struct Config {
    pub port: String,
    pub database_url: String,
    pub endpoint: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            port: env::var("PORT").unwrap_or("6476".to_string()),
            database_url: env::var("DATABASE_URL"),
            endpoint: env::var("ENDPOINT").unwrap_or("http://localhost:5173".to_string()),
        }
    }
}