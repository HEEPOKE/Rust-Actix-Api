use super::super::controllers::user::{create_user, list_users};
use super::super::models::{NewUser, User};
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::io::Error;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn create(new_user: web::Json<NewUser>, pool: web::Data<Pool>) -> impl Responder {
    match create_user(new_user.into_inner(), &pool) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

async fn list(pool: web::Data<Pool>) -> impl Responder {
    match list
    users(&pool) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
        }
        }
        
        pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(
        web::resource("/users")
        .route(web::post().to(create))
        .route(web::get().to(list)),
        );
        }