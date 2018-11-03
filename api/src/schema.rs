table! {
    tasks (task_id) {
        task_id -> Int8,
        owner_id -> Int8,
        name -> Varchar,
        description -> Text,
        created -> Int8,
        due -> Int8,
        importance -> Float8,
        in_progress -> Bool,
        progress -> Float8,
        completed -> Bool,
    }
}

table! {
    users (user_id) {
        user_id -> Int8,
        name -> Varchar,
        password -> Varchar,
    }
}

table! {
    work (work_id) {
        work_id -> Int8,
        task_id -> Int8,
        start_time -> Int8,
        end_time -> Int8,
        progress -> Float8,
        finished -> Bool,
        music -> Nullable<Bool>,
        interruptions -> Nullable<Int8>,
        noise -> Nullable<Float8>,
        meetings -> Nullable<Int8>,
        breaks -> Nullable<Int8>,
    }
}

allow_tables_to_appear_in_same_query!(
    tasks,
    users,
    work,
);
