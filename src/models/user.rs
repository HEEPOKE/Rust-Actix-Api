use diesel::prelude::*;
use super::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, EnumString, DieselTypes)]
pub enum Role {
    Admin,
    User,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub tel: String,
    pub role: Role,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub tel: &'a str,
    pub role: Role,
}