table! {
    events (event_id) {
        event_id -> Int8,
        user_id -> Int8,
        name -> Varchar,
        description -> Nullable<Text>,
        begin_timestamp -> Varchar,
        end_timestamp -> Varchar,
        recurrence -> Varchar,
        importance -> Int8,
    }
}

table! {
    users (user_id) {
        user_id -> Int8,
        name -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    events,
    users,
);
