// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        user_id -> Varchar,
        expired_at -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

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

diesel::allow_tables_to_appear_in_same_query!(sessions, users,);
