table! {
    posts (id) {
        id -> Uuid,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        username -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
