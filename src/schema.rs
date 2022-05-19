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
