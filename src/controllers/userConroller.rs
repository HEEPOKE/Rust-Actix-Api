use super::super::models::{NewUser, Role, User};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::io::Error;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

impl UserController {
    pub fn new(pool: Pool) -> Self {
        UserController { pool }
    }

    pub fn create_user(&self, user: NewUser) -> Result<User, Error> {
        let conn = self.pool.get()?;
        let user = diesel::insert_into(users::table)
            .values(&user)
            .get_result(&conn)?;
        Ok(user)
    }

    pub fn list_users(&self) -> Result<Vec<User>, Error> {
        let conn = self.pool.get()?;
        let users = users::table.load::<User>(&conn)?;
        Ok(users)
    }
}

impl NewUser<'_> {
    pub fn with_role<'a>(self, role: Role) -> NewUser<'a> {
        NewUser {
            username: self.username,
            email: self.email,
            password: self.password,
            tel: self.tel,
            role,
        }
    }
}