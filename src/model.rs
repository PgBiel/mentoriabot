mod user;

pub use user::{User, NewUser};

/// For deserializing Discord IDs from database
pub struct DiscordIdField(u64);

impl Into<u64> for DiscordIdField {
    fn into(self) -> u64 {
        self.0
    }
}

impl<DB> diesel::Queryable<diesel::sql_types::VarChar, DB> for DiscordIdField
where
    DB: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<diesel::sql_types::VarChar, DB>,
{
    type Row = String;

    fn build(s: String) -> diesel::deserialize::Result<Self> {  // convert received String to u64, if possible
        s.parse::<u64>().map(DiscordIdField).map_err(From::from)
    }
}
