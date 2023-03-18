// @generated automatically by Diesel CLI.

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
        start_at -> Timestamptz,
        end_at -> Timestamptz,
    }
}

diesel::table! {
    users (discord_id) {
        discord_id -> Varchar,
        name -> Varchar,
        bio -> Nullable<Text>,
    }
}

diesel::joinable!(lecture_students -> lectures (lecture_id));
diesel::joinable!(lecture_students -> users (user_id));
diesel::joinable!(lectures -> users (teacher_id));

diesel::allow_tables_to_appear_in_same_query!(lecture_students, lectures, users,);
