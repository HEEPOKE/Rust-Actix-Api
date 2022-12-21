use diesel::prelude::*;

#[derive(Insertable, Debug)]
#[table_name = "products"]
pub struct ProductModel {
    pub id: i32,
    pub name: String,
    pub detail: String,
    pub color: String,
    pub price: i64,
    pub created_at: Timestamp,
}