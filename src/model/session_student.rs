use diesel::{Identifiable, Insertable, Queryable};

use super::DiscordId;
use crate::schema::*;

/// Represents a relation between [`User`] and [`Session`], indicating
/// the User is attending that Session.
///
/// [`User`]: super::User
/// [`Lecture`]: super::Session
#[derive(Queryable, Identifiable, Insertable, Debug, Copy, Clone, PartialEq, Eq)]
#[diesel(primary_key(user_id, session_id))]
pub struct SessionStudent {
    pub session_id: i64,
    pub user_id: DiscordId,
}

pub type NewSessionStudent = SessionStudent; // same fields
