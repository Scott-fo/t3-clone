// @generated automatically by Diesel CLI.

diesel::table! {
    active_models (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 255]
        provider -> Varchar,
        #[max_length = 255]
        model -> Varchar,
        version -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    chats (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        archived -> Bool,
        pinned -> Bool,
        version -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    messages (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        chat_id -> Varchar,
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 255]
        role -> Varchar,
        body -> Text,
        version -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    replicache_client_groups (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        user_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        cvr_version -> Integer,
    }
}

diesel::table! {
    replicache_clients (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        client_group_id -> Varchar,
        last_mutation_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sessions (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        user_id -> Varchar,
        expired_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    active_models,
    chats,
    messages,
    replicache_client_groups,
    replicache_clients,
    sessions,
    users,
);
