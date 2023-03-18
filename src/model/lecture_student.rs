use diesel::{Identifiable, Insertable, Queryable};

use super::DiscordId;
use crate::schema::*;

/// Represents a relation between [`User`] and [`Lecture`], indicating
/// the User is taking that Lecture.
///
/// [`User`]: super::User
/// [`Lecture`]: super::Lecture
#[derive(Queryable, Identifiable, Insertable, Debug, Copy, Clone, PartialEq, Eq)]
#[diesel(primary_key(user_id, lecture_id))]
pub struct LectureStudent {
    pub lecture_id: i64,
    pub user_id: DiscordId,
}

pub type NewLectureStudent = LectureStudent; // same fields
