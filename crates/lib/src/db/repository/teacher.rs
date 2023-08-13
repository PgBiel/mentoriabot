use std::sync::Arc;

use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use super::{
    super::schema::{self, teachers},
    repo_find_all, repo_get, repo_insert, repo_remove, repo_update, repo_upsert, Repository,
    UpdatableRepository,
};
use crate::{
    error::Result,
    model::{Availability, NewTeacher, PartialTeacher, Session, Teacher},
};

/// Manages Teacher instances.
#[derive(Clone)]
pub struct TeacherRepository {
    pool: Arc<Pool<AsyncPgConnection>>,
}

impl TeacherRepository {
    /// Creates a new TeacherRepository operating with the given
    /// connection pool.
    pub fn new(pool: &Arc<Pool<AsyncPgConnection>>) -> Self {
        Self {
            pool: Arc::clone(pool),
        }
    }

    /// Attempts to insert a Teacher; does nothing if such a Teacher (with the same e-mail
    /// or something) is already registered.
    /// Returns the inserted row count (1 if a new Teacher was inserted or 0 otherwise).
    pub async fn insert_if_not_exists(&self, teacher: &NewTeacher) -> Result<usize> {
        diesel::insert_into(teachers::table)
            .values(teacher)
            .on_conflict_do_nothing()
            .execute(&mut self.lock_connection().await?)
            .await
            .map_err(From::from)
    }

    /// Gets a Session's teacher.
    pub async fn find_by_session(&self, session: &Session) -> Result<Option<Teacher>> {
        self.get(session.teacher_id).await
    }

    /// Gets all teachers linked to certain availabilities.
    pub async fn find_by_availabilities(
        &self,
        availabilities: &[Availability],
    ) -> Result<Vec<(Teacher, Availability)>> {
        teachers::table
            .inner_join(schema::availability::table)
            .filter(
                schema::availability::id.eq_any(
                    availabilities
                        .iter()
                        .map(|avail| avail.id)
                        .collect::<Vec<_>>(),
                ),
            )
            .filter(
                teachers::id.eq_any(
                    availabilities
                        .iter()
                        .map(|avail| avail.teacher_id)
                        .collect::<Vec<_>>(),
                ),
            )
            .get_results(&mut self.lock_connection().await?)
            .await
            .map_err(From::from)
    }
}

#[async_trait]
impl Repository for TeacherRepository {
    type Table = teachers::table;

    type Entity = Teacher;

    type NewEntity = NewTeacher;

    type PrimaryKey = i64;

    const TABLE: Self::Table = teachers::table;

    fn get_connection_pool(&self) -> Arc<Pool<AsyncPgConnection>> {
        Arc::clone(&self.pool)
    }

    /// Gets a Teacher by their Discord ID.
    async fn get(&self, id: i64) -> Result<Option<Teacher>> {
        repo_get!(self, teachers::table; id)
    }

    async fn insert(&self, teacher: &NewTeacher) -> Result<Teacher> {
        repo_insert!(self, teachers::table; teacher)
    }

    async fn remove(&self, teacher: &Teacher) -> Result<usize> {
        repo_remove!(self; teacher)
    }

    async fn find_all(&self) -> Result<Vec<Teacher>> {
        repo_find_all!(self, teachers::table, teachers::table)
    }
}

#[async_trait]
impl UpdatableRepository for TeacherRepository {
    type PartialEntity = PartialTeacher;

    async fn upsert(&self, teacher: &NewTeacher) -> Result<Teacher> {
        repo_upsert!(self, teachers::table; /*conflict_columns=*/teachers::id; teacher)
    }

    async fn update(&self, old_teacher: &Teacher, new_teacher: PartialTeacher) -> Result<Teacher> {
        repo_update!(self; old_teacher => new_teacher)
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::super::tests::init_db;
    use crate::{
        db::{Repository, UpdatableRepository},
        error::Result,
        model::NewTeacher,
    };

    #[tokio::test]
    async fn test_teacher_get_insert_find_remove() -> Result<()> {
        let db = init_db();
        let teacher_repo = db.teacher_repository();

        let new_teacher = NewTeacher {
            name: "John Doe".to_string(),
            email: "aaa@bbb.com".to_string(),
            specialty: "Math".to_string(),
            applied_at: None,
            company: Some("Mozilla Inc.".to_string()),
            company_role: Some("CTO".to_string()),
            bio: Some("I am a teacher".to_string()),
            course_info: Some("Teaching at University".to_string()),
            whatsapp: Some("(10) 12345-6789".to_string()),
            linkedin: Some("https://linkedin.com/????????".to_string()),
            comment_general: None,
            comment_experience: Some("All the experience".to_string()),
        };

        let inserted_teacher = teacher_repo.insert(&new_teacher).await?;

        assert_eq!(new_teacher, inserted_teacher.clone().into());
        assert_eq!(
            Some(&inserted_teacher),
            teacher_repo.get(inserted_teacher.id).await?.as_ref()
        );
        assert_eq!(
            vec![&inserted_teacher],
            teacher_repo
                .find_all()
                .await
                .unwrap()
                .iter()
                .collect::<Vec<_>>()
        );
        assert_eq!(1, teacher_repo.remove(&inserted_teacher).await?);
        assert_eq!(None, teacher_repo.get(inserted_teacher.id).await?.as_ref());

        Ok(())
    }

    #[tokio::test]
    async fn test_teacher_update() -> Result<()> {
        let db = init_db();
        let teacher_repo = db.teacher_repository();

        let new_teacher = NewTeacher {
            name: "John Rust".to_string(),
            email: "rustacean@mozilla.org".to_string(),
            specialty: "Rust".to_string(),
            applied_at: Some(
                chrono::Utc
                    .with_ymd_and_hms(2000, 10, 1, 12, 13, 14)
                    .unwrap(),
            ),
            company: Some("Mozilla Inc.".to_string()),
            company_role: Some("Programmer".to_string()),
            bio: Some("I like Rust.".to_string()),
            course_info: Some("Computer Science at MIT".to_string()),
            whatsapp: Some("(13) 12345-6778".to_string()),
            linkedin: Some("https://linkedin.com/amongus".to_string()),
            comment_general: None,
            comment_experience: Some("No comments".to_string()),
        };
        let other_teacher = NewTeacher {
            company: None,
            email: "other@mozilla.org".to_string(),
            ..new_teacher.clone()
        };
        let third_teacher = NewTeacher {
            specialty: "Swift".to_string(),
            email: "third@hotmail.com".to_string(),
            ..other_teacher.clone()
        };

        let inserted_new_teacher = teacher_repo.upsert(&new_teacher).await?;
        let inserted_other_teacher = teacher_repo.upsert(&other_teacher).await?;

        assert_eq!(new_teacher, inserted_new_teacher.clone().into());
        assert_eq!(other_teacher, inserted_other_teacher.clone().into());
        assert_eq!(
            Some(&inserted_new_teacher),
            teacher_repo.get(inserted_new_teacher.id).await?.as_ref()
        );
        assert_eq!(
            third_teacher,
            teacher_repo
                .update(&inserted_new_teacher, third_teacher.clone().into())
                .await?
                .into()
        );
        assert_eq!(
            Some(third_teacher),
            teacher_repo
                .get(inserted_new_teacher.id)
                .await?
                .as_ref()
                .map(Clone::clone)
                .map(|teacher| teacher.into())
        );

        Ok(())
    }
}
