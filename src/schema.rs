// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> i32,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        tel -> Varchar,
        created_at -> Timestamp,
        update_at -> Timestamp,
    }
}