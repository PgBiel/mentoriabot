use std::sync::Arc;

use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use super::{
    repo_find_all, repo_get, repo_insert, repo_remove, repo_update, repo_upsert, Repository,
    UpdatableRepository,
};
use crate::{
    error::Result,
    model::{Availability, DiscordId, NewTeacher, PartialTeacher, Session, Teacher, User},
    schema,
    schema::teachers,
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

    /// Gets a Teacher from the database by their Discord ID,
    /// or inserts them instead.
    pub async fn get_or_insert(&self, teacher: &NewTeacher) -> Result<Teacher> {
        if let Some(found_teacher) = self.get(teacher.user_id).await? {
            Ok(found_teacher)
        } else {
            self.insert(teacher).await
        }
    }

    /// Attempts to insert a Teacher; does nothing if such a Teacher is already registered.
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

    /// Gets a User's associated Teacher instance.
    pub async fn find_by_user(&self, user: &User) -> Result<Option<Teacher>> {
        self.get(user.discord_id).await
    }

    /// Gets all teachers linked to certain availabilities.
    pub async fn find_by_availabilities(
        &self,
        availabilities: &[Availability],
    ) -> Result<Vec<(Teacher, User, Availability)>> {
        teachers::table
            .inner_join(schema::users::table)
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
                teachers::user_id.eq_any(
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

    type PrimaryKey = DiscordId;

    const TABLE: Self::Table = teachers::table;

    fn get_connection_pool(&self) -> Arc<Pool<AsyncPgConnection>> {
        Arc::clone(&self.pool)
    }

    /// Gets a Teacher by their Discord ID.
    async fn get(&self, user_id: DiscordId) -> Result<Option<Teacher>> {
        repo_get!(self, teachers::table; user_id)
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
        repo_upsert!(self, teachers::table; /*conflict_columns=*/teachers::user_id; teacher)
    }

    async fn update(&self, old_teacher: &Teacher, new_teacher: PartialTeacher) -> Result<Teacher> {
        repo_update!(self; old_teacher => new_teacher)
    }
}
