use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};

use super::DiscordId;
use crate::db::schema::*;

/// Represents a registered Teacher, which can create Lectures, and show their possible
/// [`Availabilities`].
///
/// [`Availabilities`]: super::Availability
#[derive(
    Queryable, Identifiable, Insertable, AsChangeset, Associations, Debug, Clone, PartialEq, Eq,
)]
#[diesel(primary_key(user_id))]
#[diesel(belongs_to(super::User, foreign_key = user_id), treat_none_as_null = true)]
pub struct Teacher {
    pub user_id: DiscordId,
    pub email: Option<String>,
    pub specialty: String,
    pub company: Option<String>,
    pub company_role: Option<String>,
}

pub type NewTeacher = Teacher; // same fields

/// A Partial Teacher, in order to specify certain fields to update.
#[derive(AsChangeset, Debug, Default, Clone, PartialEq, Eq)]
#[diesel(table_name = teachers)]
pub struct PartialTeacher {
    pub user_id: Option<DiscordId>,
    pub email: Option<Option<String>>,
    pub specialty: Option<String>,
    pub company: Option<Option<String>>,
    pub company_role: Option<Option<String>>,
}

impl From<Teacher> for PartialTeacher {
    /// Converts a [`Teacher`] into a [`PartialTeacher`]
    /// by wrapping each Teacher field into a 'Some'.
    fn from(user: Teacher) -> Self {
        Self {
            user_id: Some(user.user_id),
            email: Some(user.email),
            specialty: Some(user.specialty),
            company: Some(user.company),
            company_role: Some(user.company_role),
        }
    }
}
