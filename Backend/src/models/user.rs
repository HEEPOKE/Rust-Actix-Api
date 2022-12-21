use diesel::prelude::*;

#[derive(Queryable)]
pub struct UserModel {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
}