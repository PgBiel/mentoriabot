use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, QueryableByName};

use crate::{db::schema::*, model::Weekday};

/// Represents a certain time of the week when a Teacher can initiate a Session
/// with students, which may "claim" one of them for the current week.
#[derive(
    Queryable,
    Identifiable,
    Insertable,
    AsChangeset,
    Associations,
    QueryableByName,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[diesel(belongs_to(super::Teacher, foreign_key = teacher_id))]
#[diesel(table_name = availability)]
pub struct Availability {
    pub id: i64,
    pub teacher_id: i64,
    pub weekday: Weekday,
    pub time_start: chrono::NaiveTime,
    pub expired: bool,
    pub duration: i16,
}

#[derive(Insertable, AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = availability, treat_none_as_null = true)]
pub struct NewAvailability {
    pub teacher_id: i64,
    pub weekday: Weekday,
    pub time_start: chrono::NaiveTime,
    pub expired: bool,
    pub duration: i16,
}

/// A Partial Availability, in order to specify certain fields to update.
#[derive(AsChangeset, Debug, Default, Clone, PartialEq, Eq)]
#[diesel(table_name = availability)]
pub struct PartialAvailability {
    pub id: Option<i64>,
    pub teacher_id: Option<i64>,
    pub weekday: Option<Weekday>,
    pub time_start: Option<chrono::NaiveTime>,
    pub expired: Option<bool>,
    pub duration: Option<i16>,
}

impl Availability {
    /// Returns the first date this availability could correspond to
    /// after the given date (first matching weekday).
    pub fn first_possible_date_after(
        &self,
        initial_date: &chrono::DateTime<chrono::FixedOffset>,
    ) -> chrono::DateTime<chrono::FixedOffset> {
        self.weekday.next_day_with_this_weekday(initial_date)
    }
}

impl From<Availability> for PartialAvailability {
    /// Converts a [`Availability`] into a [`PartialAvailability`]
    /// by wrapping each Availability field into a 'Some'.
    fn from(other: Availability) -> Self {
        Self {
            id: Some(other.id),
            teacher_id: Some(other.teacher_id),
            weekday: Some(other.weekday),
            time_start: Some(other.time_start),
            expired: Some(other.expired),
            duration: Some(other.duration),
        }
    }
}

impl From<Availability> for NewAvailability {
    fn from(other: Availability) -> Self {
        let Availability {
            teacher_id,
            weekday,
            time_start,
            expired,
            duration,
            ..
        } = other;

        Self {
            teacher_id,
            weekday,
            time_start,
            expired,
            duration,
        }
    }
}
