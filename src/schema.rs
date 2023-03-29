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
    lecture_students (user_id, lecture_id) {
        lecture_id -> Int8,
        user_id -> Varchar,
    }
}

diesel::table! {
    lectures (id) {
        id -> Int8,
        teacher_id -> Varchar,
        name -> Varchar,
        description -> Text,
        notified -> Bool,
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
diesel::joinable!(lecture_students -> lectures (lecture_id));
diesel::joinable!(lecture_students -> users (user_id));
diesel::joinable!(lectures -> users (teacher_id));
diesel::joinable!(teachers -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    availability,
    lecture_students,
    lectures,
    teachers,
    users,
);
