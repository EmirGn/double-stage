// @generated automatically by Diesel CLI.

diesel::table! {
    chats (id) {
        id -> Text,
        user_id -> Nullable<Text>,
        #[max_length = 50]
        title -> Varchar,
        history -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        #[max_length = 50]
        username -> Varchar,
    }
}

diesel::joinable!(chats -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    chats,
    users,
);
