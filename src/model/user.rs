use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

use super::DiscordId;
use crate::schema::*;

/// Represents a registered User of our bot.
#[derive(Queryable, Identifiable, Insertable, AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(primary_key(discord_id), treat_none_as_null = true)]
pub struct User {
    pub discord_id: DiscordId,
    pub name: String,
    pub bio: Option<String>,
}

pub type NewUser = User; // same fields

/// A Partial User, in order to specify certain fields to update.
#[derive(AsChangeset, Debug, Default, Clone, PartialEq, Eq)]
#[diesel(table_name = users)]
pub struct PartialUser {
    pub discord_id: Option<DiscordId>,
    pub name: Option<String>,
    pub bio: Option<Option<String>>,
}

impl From<User> for PartialUser {
    /// Converts a [`User`] into a [`PartialUser`]
    /// by wrapping each User field into a 'Some'.
    fn from(user: User) -> Self {
        Self {
            discord_id: Some(user.discord_id),
            name: Some(user.name),
            bio: Some(user.bio),
        }
    }
}
