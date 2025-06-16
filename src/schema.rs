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
        #[max_length = 255]
        reasoning -> Nullable<Varchar>,
        version -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    api_keys (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 64]
        provider -> Varchar,
        #[max_length = 512]
        encrypted_key -> Varbinary,
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
        forked -> Bool,
        version -> Integer,
        pinned_at -> Nullable<Timestamp>,
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
        reasoning -> Nullable<Text>,
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
    shared_chats (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        original_chat_id -> Varchar,
        #[max_length = 255]
        owner_user_id -> Varchar,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    shared_messages (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        shared_chat_id -> Varchar,
        #[max_length = 255]
        role -> Varchar,
        body -> Text,
        reasoning -> Nullable<Text>,
        created_at -> Timestamp,
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
    api_keys,
    chats,
    messages,
    replicache_client_groups,
    replicache_clients,
    sessions,
    shared_chats,
    shared_messages,
    users,
);
