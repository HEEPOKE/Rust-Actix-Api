use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

// pub mod models;
pub mod schema;

pub fn db_connection() -> Result<PgConnection, String> {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => return Err("DATABASE_URL must be set".into()),
    };

    PgConnection::establish(&database_url)
        .map_err(|error| format!("Error connecting to {}: {:?}", database_url, error))
}
