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
    SUNDAY = 0,
    MONDAY = 1,
    TUESDAY = 2,
    WEDNESDAY = 3,
    THURSDAY = 4,
    FRIDAY = 5,
    SATURDAY = 6,
}

impl TryFrom<i16> for Weekday {
    type Error = Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SUNDAY),
            1 => Ok(Self::MONDAY),
            2 => Ok(Self::TUESDAY),
            3 => Ok(Self::WEDNESDAY),
            4 => Ok(Self::THURSDAY),
            5 => Ok(Self::FRIDAY),
            6 => Ok(Self::SATURDAY),
            _ => Err(Error::Other("Failed to convert from i16 to Weekday")),
        }
    }
}

impl From<Weekday> for i16 {
    fn from(value: Weekday) -> Self {
        match value {
            Weekday::SUNDAY => 0,
            Weekday::MONDAY => 1,
            Weekday::TUESDAY => 2,
            Weekday::WEDNESDAY => 3,
            Weekday::THURSDAY => 4,
            Weekday::FRIDAY => 5,
            Weekday::SATURDAY => 6,
        }
    }
}

impl From<&Weekday> for i16 {
    fn from(value: &Weekday) -> Self {
        Into::<i16>::into(value.clone())
    }
}

impl From<chrono::Weekday> for Weekday {
    fn from(value: chrono::Weekday) -> Self {
        match value {
            chrono::Weekday::Sun => Self::SUNDAY,
            chrono::Weekday::Mon => Self::MONDAY,
            chrono::Weekday::Tue => Self::TUESDAY,
            chrono::Weekday::Wed => Self::WEDNESDAY,
            chrono::Weekday::Thu => Self::THURSDAY,
            chrono::Weekday::Fri => Self::FRIDAY,
            chrono::Weekday::Sat => Self::SATURDAY,
        }
    }
}

impl From<Weekday> for chrono::Weekday {
    fn from(value: Weekday) -> Self {
        match value {
            Weekday::SUNDAY => Self::Sun,
            Weekday::MONDAY => Self::Mon,
            Weekday::TUESDAY => Self::Tue,
            Weekday::WEDNESDAY => Self::Wed,
            Weekday::THURSDAY => Self::Thu,
            Weekday::FRIDAY => Self::Fri,
            Weekday::SATURDAY => Self::Sat,
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
