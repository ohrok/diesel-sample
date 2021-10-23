table! {
    posts (id) {
        id -> Uuid,
        title -> Text,
        body -> Text,
        user_id -> Uuid,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        username -> Text,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
