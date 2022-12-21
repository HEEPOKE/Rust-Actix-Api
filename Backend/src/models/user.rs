use diesel::prelude::*;
use crate::schema::*;

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Timestamp,
}