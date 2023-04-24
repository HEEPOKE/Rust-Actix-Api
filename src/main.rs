use actix_web::{App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use std::io;
use std::env;

// mod controllers;
// mod models;
// mod routes;
mod schema;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() ->  io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}