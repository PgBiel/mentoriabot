// @generated automatically by Diesel CLI.

diesel::table! {
    availability (id) {
        id -> Int8,
        teacher_id -> Int8,
        weekday -> Int2,
        time_start -> Time,
        expired -> Bool,
        duration -> Int2,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int8,
        teacher_id -> Int8,
        student_id -> Varchar,
        availability_id -> Int8,
        summary -> Nullable<Text>,
        notified -> Bool,
        meet_id -> Nullable<Varchar>,
        start_at -> Timestamptz,
        end_at -> Timestamptz,
    }
}

diesel::table! {
    teachers (id) {
        id -> Int8,
        name -> Varchar,
        email -> Varchar,
        specialty -> Varchar,
        applied_at -> Nullable<Timestamptz>,
        bio -> Nullable<Varchar>,
        course_info -> Nullable<Varchar>,
        company -> Nullable<Varchar>,
        company_role -> Nullable<Varchar>,
        whatsapp -> Nullable<Varchar>,
        linkedin -> Nullable<Varchar>,
        comment_general -> Nullable<Varchar>,
        comment_experience -> Nullable<Varchar>,
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

diesel::allow_tables_to_appear_in_same_query!(availability, sessions, teachers, users,);
