use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

use super::DiscordId;
use crate::schema::*;

/// Represents a registered User of our bot.
#[derive(Queryable, Identifiable, Insertable, AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(primary_key(discord_id))]
pub struct User {
    pub discord_id: DiscordId,
    pub name: String,
    pub bio: Option<String>,
}

pub type NewUser = User; // same fields
