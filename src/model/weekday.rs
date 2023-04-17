use diesel::{
    backend::RawValue,
    deserialize::FromSql,
    serialize::{Output, ToSql},
    sql_types::SmallInt,
    AsExpression, FromSqlRow,
};

use crate::error::Error;

#[derive(
    FromSqlRow,
    AsExpression,
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[diesel(sql_type = SmallInt)]
pub enum Weekday {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

impl TryFrom<i16> for Weekday {
    type Error = Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Sunday),
            1 => Ok(Self::Monday),
            2 => Ok(Self::Tuesday),
            3 => Ok(Self::Wednesday),
            4 => Ok(Self::Thursday),
            5 => Ok(Self::Friday),
            6 => Ok(Self::Saturday),
            _ => Err(Error::Other("Failed to convert from i16 to Weekday")),
        }
    }
}

impl From<Weekday> for i16 {
    fn from(value: Weekday) -> Self {
        match value {
            Weekday::Sunday => 0,
            Weekday::Monday => 1,
            Weekday::Tuesday => 2,
            Weekday::Wednesday => 3,
            Weekday::Thursday => 4,
            Weekday::Friday => 5,
            Weekday::Saturday => 6,
        }
    }
}

impl From<&Weekday> for i16 {
    fn from(value: &Weekday) -> Self {
        Into::<i16>::into(*value)
    }
}

impl From<chrono::Weekday> for Weekday {
    fn from(value: chrono::Weekday) -> Self {
        match value {
            chrono::Weekday::Sun => Self::Sunday,
            chrono::Weekday::Mon => Self::Monday,
            chrono::Weekday::Tue => Self::Tuesday,
            chrono::Weekday::Wed => Self::Wednesday,
            chrono::Weekday::Thu => Self::Thursday,
            chrono::Weekday::Fri => Self::Friday,
            chrono::Weekday::Sat => Self::Saturday,
        }
    }
}

impl From<Weekday> for chrono::Weekday {
    fn from(value: Weekday) -> Self {
        match value {
            Weekday::Sunday => Self::Sun,
            Weekday::Monday => Self::Mon,
            Weekday::Tuesday => Self::Tue,
            Weekday::Wednesday => Self::Wed,
            Weekday::Thursday => Self::Thu,
            Weekday::Friday => Self::Fri,
            Weekday::Saturday => Self::Sat,
        }
    }
}

impl ToSql<SmallInt, diesel::pg::Pg> for Weekday
where
    i16: ToSql<SmallInt, diesel::pg::Pg>,
{
    /// Allows usage of Weekday with diesel, with SmallInt fields.
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
        let converted_self: i16 = self.into();
        <i16 as ToSql<SmallInt, diesel::pg::Pg>>::to_sql(&converted_self, &mut out.reborrow()) // see ToSql docs regarding temp values
    }
}

impl<DB> FromSql<SmallInt, DB> for Weekday
where
    DB: diesel::backend::Backend,
    i16: FromSql<SmallInt, DB>,
{
    /// Allows usage of Weekday with diesel, with SmallInt fields.
    fn from_sql(bytes: RawValue<'_, DB>) -> diesel::deserialize::Result<Self> {
        i16::from_sql(bytes).and_then(|v| Self::try_from(v).map_err(Into::into))
    }

    fn from_nullable_sql(bytes: Option<RawValue<'_, DB>>) -> diesel::deserialize::Result<Self> {
        i16::from_nullable_sql(bytes).and_then(|v| Self::try_from(v).map_err(Into::into))
    }
}
