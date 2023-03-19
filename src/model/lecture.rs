use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};

use super::DiscordId;
use crate::schema::*;

/// Represents a certain lecture.
#[derive(
    Queryable, Identifiable, Insertable, AsChangeset, Associations, Debug, Clone, PartialEq, Eq,
)]
#[diesel(belongs_to(super::User, foreign_key = teacher_id))]
pub struct Lecture {
    pub id: i64,
    pub teacher_id: DiscordId,
    pub name: String,
    pub description: String,
    pub student_limit: i32,
    pub notified: bool,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
}

/// Auxiliary struct for inserting a Lecture.
#[derive(Insertable, AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = lectures)]
pub struct NewLecture {
    pub teacher_id: DiscordId,
    pub name: String,
    pub description: String,
    pub student_limit: i32,
    pub notified: bool,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
}

#[derive(AsChangeset, Debug, Default, Clone, PartialEq, Eq)]
#[diesel(table_name = lectures)]
pub struct PartialLecture {
    pub teacher_id: Option<DiscordId>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub student_limit: Option<i32>,
    pub notified: Option<bool>,
    pub start_at: Option<chrono::DateTime<chrono::Utc>>,
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<NewLecture> for PartialLecture {
    /// Converts a [`NewLecture`] into a [`PartialLecture`]
    /// by wrapping each Lecture field into a 'Some'.
    fn from(lecture: NewLecture) -> Self {
        Self {
            teacher_id: Some(lecture.teacher_id),
            name: Some(lecture.name),
            description: Some(lecture.description),
            student_limit: Some(lecture.student_limit),
            notified: Some(lecture.notified),
            start_at: Some(lecture.start_at),
            end_at: Some(lecture.end_at),
        }
    }
}
