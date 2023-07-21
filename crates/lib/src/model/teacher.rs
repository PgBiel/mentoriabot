use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

use crate::db::schema::*;

/// Represents a registered Teacher, which can create Lectures, and show their possible
/// [`Availabilities`].
///
/// [`Availabilities`]: super::Availability
#[derive(Queryable, Identifiable, Insertable, AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(treat_none_as_null = true)]
pub struct Teacher {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub specialty: String,
    pub applied_at: Option<chrono::DateTime<chrono::Utc>>,
    pub bio: Option<String>,
    pub company: Option<String>,
    pub company_role: Option<String>,
    pub whatsapp: Option<String>,
    pub linkedin: Option<String>,
}

/// A New Teacher, to be inserted
#[derive(Insertable, AsChangeset, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = teachers, treat_none_as_null = true)]
pub struct NewTeacher {
    pub name: String,
    pub email: String,
    pub specialty: String,
    pub applied_at: Option<chrono::DateTime<chrono::Utc>>,
    pub bio: Option<String>,
    pub company: Option<String>,
    pub company_role: Option<String>,
    pub whatsapp: Option<String>,
    pub linkedin: Option<String>,
}

/// A Partial Teacher, in order to specify certain fields to update.
#[derive(AsChangeset, Debug, Default, Clone, PartialEq, Eq)]
#[diesel(table_name = teachers)]
pub struct PartialTeacher {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub specialty: Option<String>,
    pub applied_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub bio: Option<Option<String>>,
    pub company: Option<Option<String>>,
    pub company_role: Option<Option<String>>,
    pub whatsapp: Option<Option<String>>,
    pub linkedin: Option<Option<String>>,
}

impl From<Teacher> for NewTeacher {
    /// Converts a [`Teacher`] into a [`NewTeacher`]
    /// by ignoring the 'id' attribute.
    fn from(teacher: Teacher) -> Self {
        Self {
            name: teacher.name,
            email: teacher.email,
            specialty: teacher.specialty,
            applied_at: teacher.applied_at,
            company: teacher.company,
            company_role: teacher.company_role,
            bio: teacher.bio,
            whatsapp: teacher.whatsapp,
            linkedin: teacher.linkedin,
        }
    }
}

impl From<Teacher> for PartialTeacher {
    /// Converts a [`Teacher`] into a [`PartialTeacher`]
    /// by wrapping each Teacher field into a 'Some'.
    fn from(teacher: Teacher) -> Self {
        Self {
            id: Some(teacher.id),
            name: Some(teacher.name),
            email: Some(teacher.email),
            specialty: Some(teacher.specialty),
            applied_at: Some(teacher.applied_at),
            company: Some(teacher.company),
            company_role: Some(teacher.company_role),
            bio: Some(teacher.bio),
            whatsapp: Some(teacher.whatsapp),
            linkedin: Some(teacher.linkedin),
        }
    }
}

impl From<NewTeacher> for PartialTeacher {
    /// Converts a [`NewTeacher`] into a [`PartialTeacher`]
    /// by wrapping each Teacher field into a 'Some',
    /// except for 'id' (None).
    fn from(new_teacher: NewTeacher) -> Self {
        Self {
            id: None,
            name: Some(new_teacher.name),
            email: Some(new_teacher.email),
            specialty: Some(new_teacher.specialty),
            applied_at: Some(new_teacher.applied_at),
            company: Some(new_teacher.company),
            company_role: Some(new_teacher.company_role),
            bio: Some(new_teacher.bio),
            whatsapp: Some(new_teacher.whatsapp),
            linkedin: Some(new_teacher.linkedin),
        }
    }
}
