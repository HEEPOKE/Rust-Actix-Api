use super::super::models::{NewUser, User};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::io::Error;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_user(user: NewUser, pool: &Pool) -> Result<User, Error> {
    let conn = pool.get()?;
    let user = diesel::insert_into(users::table)
        .values(&user)
        .get_result(&conn)?;
    Ok(user)
}

pub fn list_users(pool: &Pool) -> Result<Vec<User>, Error> {
    let conn = pool.get()?;
    let users = users::table.load::<User>(&conn)?;
    Ok(users)
}
