table! {
    customers (refcli) {
        refcli -> Int4,
        name -> Text,
        address -> Text,
        postal -> Text,
        town -> Text,
    }
}

table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
    }
}

table! {
    reviews (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
        original -> Text,
        thumbnail -> Text,
        web -> Text,
        deleted -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    customers,
    posts,
    reviews,
    users,
);
