// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        discord_userid -> Varchar,
        bio -> Nullable<Text>,
    }
}
