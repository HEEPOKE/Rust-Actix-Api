diesel::table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        tel -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}