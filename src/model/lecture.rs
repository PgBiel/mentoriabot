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
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
}
