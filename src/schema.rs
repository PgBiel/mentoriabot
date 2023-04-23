// @generated automatically by Diesel CLI.

diesel::table! {
    availability (id) {
        id -> Int8,
        teacher_id -> Varchar,
        weekday -> Int2,
        time_start -> Time,
        time_end -> Time,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int8,
        teacher_id -> Varchar,
        student_id -> Varchar,
        name -> Varchar,
        description -> Text,
        notified -> Bool,
        availability_id -> Nullable<Int8>,
        start_at -> Timestamptz,
        end_at -> Timestamptz,
    }
}

diesel::table! {
    teachers (user_id) {
        user_id -> Varchar,
        email -> Nullable<Varchar>,
        specialty -> Varchar,
    }
}

diesel::table! {
    users (discord_id) {
        discord_id -> Varchar,
        name -> Varchar,
        bio -> Nullable<Text>,
    }
}

diesel::joinable!(availability -> teachers (teacher_id));
diesel::joinable!(sessions -> availability (availability_id));
diesel::joinable!(sessions -> teachers (teacher_id));
diesel::joinable!(sessions -> users (student_id));
diesel::joinable!(teachers -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    availability,
    sessions,
    teachers,
    users,
);
