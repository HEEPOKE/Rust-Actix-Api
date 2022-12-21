use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub detail: String,
    pub color: String,
    pub price: i64,
}