// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
        server_id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        attributes -> Nullable<Jsonb>,
    }
}

diesel::table! {
    channels (id) {
        id -> Int8,
        #[max_length = 50]
        name -> Varchar,
        server_id -> Uuid,
        category_id -> Nullable<Uuid>,
        #[sql_name = "type"]
        #[max_length = 12]
        type_ -> Varchar,
        order_in_ui -> Int4,
        attributes -> Nullable<Jsonb>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    direct_messages (id) {
        id -> Uuid,
        sender_id -> Uuid,
        receiver_id -> Uuid,
        content -> Text,
        attachments -> Nullable<Jsonb>,
        reactions -> Nullable<Jsonb>,
        edited_at -> Nullable<Timestamptz>,
        reference_id -> Nullable<Uuid>,
        attributes -> Nullable<Jsonb>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    memberships (id) {
        id -> Uuid,
        user_id -> Uuid,
        server_id -> Uuid,
        joined_at -> Nullable<Timestamptz>,
        role_ids -> Nullable<Array<Nullable<Uuid>>>,
        #[max_length = 32]
        nickname -> Nullable<Varchar>,
        attributes -> Nullable<Jsonb>,
    }
}

diesel::table! {
    messages (id) {
        id -> Int8,
        channel_id -> Int8,
        user_id -> Uuid,
        content -> Text,
        created_at -> Nullable<Timestamptz>,
        edited_at -> Nullable<Timestamptz>,
        reference_id -> Nullable<Int8>,
        attachments -> Nullable<Jsonb>,
        reactions -> Nullable<Jsonb>,
        attributes -> Nullable<Jsonb>,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 7]
        color -> Nullable<Varchar>,
        permissions -> Nullable<Int8>,
        attributes -> Nullable<Jsonb>,
        server_id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    servers (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        owner_id -> Uuid,
        #[max_length = 256]
        icon_url -> Nullable<Varchar>,
        #[max_length = 10]
        region -> Varchar,
        attributes -> Nullable<Jsonb>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 32]
        username -> Varchar,
        #[max_length = 64]
        email -> Varchar,
        #[max_length = 256]
        avatar_url -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
        last_active -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(categories -> servers (server_id));
diesel::joinable!(channels -> categories (category_id));
diesel::joinable!(channels -> servers (server_id));
diesel::joinable!(memberships -> servers (server_id));
diesel::joinable!(memberships -> users (user_id));
diesel::joinable!(messages -> channels (channel_id));
diesel::joinable!(messages -> users (user_id));
diesel::joinable!(roles -> servers (server_id));
diesel::joinable!(servers -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    channels,
    direct_messages,
    memberships,
    messages,
    roles,
    servers,
    users,
);
