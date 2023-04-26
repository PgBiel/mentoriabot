use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};

use super::DiscordId;
use crate::schema::*;

/// Represents a session between a teacher and its student(s).
#[derive(
    Queryable, Identifiable, Insertable, AsChangeset, Associations, Debug, Clone, PartialEq, Eq,
)]
#[diesel(belongs_to(super::User, foreign_key = teacher_id))]
pub struct Session {
    pub id: i64,
    pub teacher_id: DiscordId,
    pub student_id: DiscordId,
    pub name: String,
    pub description: String,
    pub notified: bool,
    pub availability_id: Option<i64>,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
}

/// Auxiliary struct for inserting a Session.
#[derive(Insertable, AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub teacher_id: DiscordId,
    pub student_id: DiscordId,
    pub name: String,
    pub description: String,
    pub notified: bool,
    pub availability_id: Option<i64>,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
}

#[derive(AsChangeset, Debug, Default, Clone, PartialEq, Eq)]
#[diesel(table_name = sessions)]
pub struct PartialSession {
    pub id: Option<i64>,
    pub teacher_id: Option<DiscordId>,
    pub student_id: Option<DiscordId>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub notified: Option<bool>,
    pub availability_id: Option<Option<i64>>,
    pub start_at: Option<chrono::DateTime<chrono::Utc>>,
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<Session> for PartialSession {
    /// Converts a [`Session`] into a [`PartialSession`]
    /// by wrapping each Session field into a 'Some'.
    fn from(session: Session) -> Self {
        Self {
            id: Some(session.id),
            teacher_id: Some(session.teacher_id),
            student_id: Some(session.student_id),
            name: Some(session.name),
            description: Some(session.description),
            notified: Some(session.notified),
            availability_id: Some(session.availability_id),
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
            name,
            description,
            notified,
            availability_id,
            start_at,
            end_at,
            ..
        } = session;

        Self {
            teacher_id,
            student_id,
            name,
            description,
            notified,
            availability_id,
            start_at,
            end_at,
        }
    }
}
