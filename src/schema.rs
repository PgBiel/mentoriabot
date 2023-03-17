// @generated automatically by Diesel CLI.

diesel::table! {
    users (discord_id) {
        discord_id -> Varchar,
        name -> Varchar,
        bio -> Nullable<Text>,
    }
}
