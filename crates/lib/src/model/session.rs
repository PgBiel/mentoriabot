use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};

use super::DiscordId;
use crate::db::schema::*;

/// Represents a session between a teacher and its student(s).
#[derive(
    Queryable, Identifiable, Insertable, AsChangeset, Associations, Debug, Clone, PartialEq, Eq,
)]
#[diesel(belongs_to(super::User, foreign_key = student_id))]
pub struct Session {
    pub id: i64,
    pub teacher_id: i64,
    pub student_id: DiscordId,
    pub availability_id: i64,
    pub summary: Option<String>,
    pub notified: bool,
    pub meet_id: Option<String>,
    pub calendar_event_id: Option<String>,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
}

/// Auxiliary struct for inserting a Session.
#[derive(Insertable, AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = sessions, treat_none_as_null = true)]
pub struct NewSession {
    pub teacher_id: i64,
    pub student_id: DiscordId,
    pub availability_id: i64,
    pub summary: Option<String>,
    pub notified: bool,
    pub meet_id: Option<String>,
    pub calendar_event_id: Option<String>,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
}

#[derive(AsChangeset, Debug, Default, Clone, PartialEq, Eq)]
#[diesel(table_name = sessions)]
pub struct PartialSession {
    pub id: Option<i64>,
    pub teacher_id: Option<i64>,
    pub student_id: Option<DiscordId>,
    pub availability_id: Option<i64>,
    pub summary: Option<Option<String>>,
    pub notified: Option<bool>,
    pub meet_id: Option<Option<String>>,
    pub calendar_event_id: Option<Option<String>>,
    pub start_at: Option<chrono::DateTime<chrono::Utc>>,
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Session {
    /// Given the session's start time and the amount of duration units, returns the appropriate
    /// 'end_at' date. Currently, each duration unit corresponds to 40 minutes.
    pub fn generate_end_at_from_duration(
        start_at: chrono::DateTime<chrono::Utc>,
        duration_units: i64,
    ) -> chrono::DateTime<chrono::Utc> {
        const MINUTES_PER_DURATION_UNIT: i64 = 40;

        start_at + chrono::Duration::minutes(duration_units * MINUTES_PER_DURATION_UNIT)
    }
}

impl From<Session> for PartialSession {
    /// Converts a [`Session`] into a [`PartialSession`]
    /// by wrapping each Session field into a 'Some'.
    fn from(session: Session) -> Self {
        Self {
            id: Some(session.id),
            teacher_id: Some(session.teacher_id),
            student_id: Some(session.student_id),
            availability_id: Some(session.availability_id),
            summary: Some(session.summary),
            notified: Some(session.notified),
            meet_id: Some(session.meet_id),
            calendar_event_id: Some(session.calendar_event_id),
            start_at: Some(session.start_at),
            end_at: Some(session.end_at),
        }
    }
}

impl From<Session> for NewSession {
    /// Takes all of the [`Session`]'s fields, except
    /// for `id`.
    fn from(session: Session) -> Self {
        let Session {
            teacher_id,
            student_id,
            availability_id,
            summary,
            notified,
            meet_id,
            calendar_event_id,
            start_at,
            end_at,
            ..
        } = session;

        Self {
            teacher_id,
            student_id,
            availability_id,
            summary,
            notified,
            meet_id,
            calendar_event_id,
            start_at,
            end_at,
        }
    }
}
