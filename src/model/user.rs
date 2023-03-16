use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use crate::schema::*;

use super::DiscordIdField;

#[derive(Debug, Queryable, Identifiable, Clone, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub name: String,

    // custom deserialization as it's a String in the database
    // due to integer size limitations in Postgres
    #[diesel(deserialize_as = DiscordIdField)]
    pub discord_userid: u64,
    pub bio: Option<String>,
}

#[derive(Debug, Insertable, AsChangeset, Clone, PartialEq, Eq)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub discord_userid: String,
    pub bio: Option<String>,
}
