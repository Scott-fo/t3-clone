// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_digest -> Varchar,
        version -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}
