table! {
    spotify_tokens (id) {
        id -> Int4,
        username -> Varchar,
        access_token -> Varchar,
        expires_in_seconds -> Int8,
        expires_at -> Nullable<Timestamp>,
        refresh_token -> Nullable<Varchar>,
        scopes -> Array<Text>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        last_login -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    spotify_tokens,
    users,
);
