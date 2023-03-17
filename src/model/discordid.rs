use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use diesel::{
    backend::RawValue,
    deserialize::FromSql,
    serialize,
    serialize::{Output, ToSql},
    sql_types::VarChar,
    AsExpression, FromSqlRow,
};
use poise::serenity_prelude as serenity;

macro_rules! impl_from_u64_id {
    ($from_type:ty) => {
        impl From<$from_type> for DiscordId {
            fn from(value: $from_type) -> Self {
                Into::<u64>::into(value).into()
            }
        }
    };
}

/// For deserializing Discord IDs from database
#[derive(FromSqlRow, AsExpression, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[diesel(sql_type = VarChar)]
pub struct DiscordId(u64);

impl Into<u64> for DiscordId {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<u64> for DiscordId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl FromStr for DiscordId {
    type Err = <u64 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <u64 as FromStr>::from_str(s).map(Self)
    }
}

impl Display for DiscordId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0.to_string(), f)
    }
}

impl_from_u64_id!(serenity::UserId);
impl_from_u64_id!(serenity::RoleId);
impl_from_u64_id!(serenity::EmojiId);
impl_from_u64_id!(serenity::GuildId);
impl_from_u64_id!(serenity::ChannelId);
impl_from_u64_id!(serenity::GenericId);

impl ToSql<VarChar, diesel::pg::Pg> for DiscordId
where
    String: ToSql<VarChar, diesel::pg::Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::pg::Pg>) -> serialize::Result {
        let converted_self = self.to_string();
        <String as ToSql<VarChar, diesel::pg::Pg>>::to_sql(&converted_self, &mut out.reborrow()) // see ToSql docs regarding temp values
    }
}

impl<DB> FromSql<VarChar, DB> for DiscordId
where
    DB: diesel::backend::Backend,
    String: FromSql<VarChar, DB>,
{
    fn from_sql(bytes: RawValue<'_, DB>) -> diesel::deserialize::Result<Self> {
        String::from_sql(bytes).and_then(|s| s.parse().map_err(Into::into))
    }

    fn from_nullable_sql(bytes: Option<RawValue<'_, DB>>) -> diesel::deserialize::Result<Self> {
        String::from_nullable_sql(bytes).and_then(|s| s.parse().map_err(Into::into))
    }
}
