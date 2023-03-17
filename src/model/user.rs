use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

use super::DiscordId;
use crate::schema::*;

/// Represents a registered User of our bot.
#[derive(Debug, Queryable, Identifiable, Clone, PartialEq, Eq)]
#[diesel(primary_key(discord_id))]
pub struct User {
    pub discord_id: DiscordId,
    pub name: String,
    pub bio: Option<String>,
}

/// Represents data for a new User in our database.
#[derive(Debug, Insertable, AsChangeset, Clone, PartialEq, Eq)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub discord_id: DiscordId,
    pub name: String,
    pub bio: Option<String>,
}
