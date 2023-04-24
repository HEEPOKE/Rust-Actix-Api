use crate::schema::users;
use diesel::prelude::*;
use chrono::NaiveDateTime;
 
#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}