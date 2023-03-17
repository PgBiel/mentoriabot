use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

use super::DiscordId;
use crate::schema::*;

/// Represents a registered User of our bot.
#[derive(Debug, Queryable, Identifiable, Clone, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub name: String,

    // custom deserialization as it's a String in the database
    // due to integer size limitations in Postgres
    pub discord_userid: DiscordId,
    pub bio: Option<String>,
}

/// Represents data for a new User in our database.
#[derive(Debug, Insertable, AsChangeset, Clone, PartialEq, Eq)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub discord_userid: DiscordId,
    pub bio: Option<String>,
}
