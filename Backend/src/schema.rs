// @generated automatically by Diesel CLI.

table! {
    users (id) {
        id -> i32,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}