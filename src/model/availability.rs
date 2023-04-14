use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};

use super::DiscordId;
use crate::{model::Weekday, schema::*};

/// Represents a certain time of the week when a Teacher can initiate a Lecture (or any kind
/// of session) with students, which may "claim" one of them for the current week.
#[derive(
    Queryable, Identifiable, Insertable, AsChangeset, Associations, Debug, Clone, PartialEq, Eq,
)]
#[diesel(belongs_to(super::Teacher, foreign_key = teacher_id))]
#[diesel(table_name = availability)]
pub struct Availability {
    pub id: i64,
    pub teacher_id: DiscordId,
    pub weekday: Weekday,
    pub time_start: chrono::NaiveTime,
    pub time_end: chrono::NaiveTime,
}

#[derive(Insertable, AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = availability)]
pub struct NewAvailability {
    pub teacher_id: DiscordId,
    pub weekday: Weekday,
    pub time_start: chrono::NaiveTime,
    pub time_end: chrono::NaiveTime,
}

/// A Partial Availability, in order to specify certain fields to update.
#[derive(AsChangeset, Debug, Default, Clone, PartialEq, Eq)]
#[diesel(table_name = availability)]
pub struct PartialAvailability {
    pub id: Option<i64>,
    pub teacher_id: Option<DiscordId>,
    pub weekday: Option<Weekday>,
    pub time_start: Option<chrono::NaiveTime>,
    pub time_end: Option<chrono::NaiveTime>,
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
            time_end: Some(other.time_end),
        }
    }
}
