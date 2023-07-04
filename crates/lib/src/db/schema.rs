// @generated automatically by Diesel CLI.

diesel::table! {
    availability (id) {
        id -> Int8,
        teacher_id -> Varchar,
        weekday -> Int2,
        time_start -> Time,
        expired -> Bool,
        duration -> Int2,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int8,
        teacher_id -> Varchar,
        student_id -> Varchar,
        availability_id -> Int8,
        summary -> Nullable<Text>,
        notified -> Bool,
        start_at -> Timestamptz,
        end_at -> Timestamptz,
    }
}

diesel::table! {
    teachers (user_id) {
        user_id -> Varchar,
        email -> Varchar,
        specialty -> Varchar,
        company -> Nullable<Varchar>,
        company_role -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (discord_id) {
        discord_id -> Varchar,
        name -> Varchar,
        email -> Varchar,
        bio -> Nullable<Text>,
    }
}

diesel::joinable!(availability -> teachers (teacher_id));
diesel::joinable!(sessions -> availability (availability_id));
diesel::joinable!(sessions -> teachers (teacher_id));
diesel::joinable!(sessions -> users (student_id));
diesel::joinable!(teachers -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(availability, sessions, teachers, users,);
