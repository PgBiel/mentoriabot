// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        discord_userid -> Int8,
        bio -> Nullable<Text>,
    }
}
