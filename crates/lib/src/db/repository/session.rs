use std::sync::Arc;

use async_trait::async_trait;
use diesel::{dsl::sql, ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use super::{
    super::schema::{sessions, teachers, users},
    repo_find_all, repo_find_by, repo_get, repo_insert, repo_remove, repo_update, repo_upsert,
    Repository, UpdatableRepository,
};
use crate::{
    error::Result,
    model::{DiscordId, NewSession, PartialSession, Session, Teacher, User},
};

/// Manages Session instances.
#[derive(Clone)]
pub struct SessionRepository {
    pool: Arc<Pool<AsyncPgConnection>>,
}

impl SessionRepository {
    /// Creates a new SessionRepository operating with the given
    /// connection pool.
    pub fn new(pool: &Arc<Pool<AsyncPgConnection>>) -> Self {
        Self {
            pool: Arc::clone(pool),
        }
    }

    /// Finds a Session and retrieves the associated teacher's Teacher object.
    pub async fn get_with_teacher(&self, session_id: i64) -> Result<Option<(Session, Teacher)>> {
        sessions::table
            .filter(sessions::id.eq(session_id))
            .inner_join(teachers::table)
            .get_results(&mut self.lock_connection().await?)
            .await
            .map(|v| v.into_iter().next())
            .map_err(From::from)
    }

    /// Finds a Session and retrieves both the associated teacher's Teacher object
    /// and the student's User object.
    pub async fn get_with_participants(
        &self,
        session_id: i64,
    ) -> Result<Option<(Session, Teacher, User)>> {
        sessions::table
            .inner_join(teachers::table)
            .inner_join(users::table)
            .filter(sessions::id.eq(session_id))
            .get_results(&mut self.lock_connection().await?)
            .await
            .map(|v| v.into_iter().next())
            .map_err(From::from)
    }

    /// Searches for Sessions by a particular teacher (with a particular Discord ID),
    /// in ascending 'start_at' order (starting earlier first).
    pub async fn find_by_teacher(&self, teacher_id: i64) -> Result<Vec<Session>> {
        repo_find_by!(
            self, sessions::table;
            sessions::teacher_id.eq(teacher_id);
            @order_by: sessions::start_at.asc()
        )
    }

    /// Searches for Sessions by a particular student (with a particular Discord ID),
    /// in ascending 'start_at' order (starting earlier first).
    pub async fn find_by_student(&self, student_id: DiscordId) -> Result<Vec<Session>> {
        repo_find_by!(
            self, sessions::table;
            sessions::student_id.eq(student_id);
            @order_by: sessions::start_at.asc()
        )
    }

    /// Searches for Sessions by a particular student (with a particular Discord ID),
    /// in ascending 'start_at' order (starting earlier first), based on a partial
    /// session ID input.
    /// If 'only_active' is given, only active sessions (which haven't already started)
    /// are displayed.
    pub async fn find_student_autocomplete(
        &self,
        student_id: DiscordId,
        partial_id: &str,
        only_active: bool,
    ) -> Result<Vec<Session>> {
        if !partial_id.is_empty() && partial_id.parse::<u32>().is_err() {
            // invalid integer => no results
            return Ok(Vec::new());
        }

        repo_find_by!(
            self, sessions::table;
            // belongs to this student
            sessions::student_id.eq(student_id)

            // starts with the given id string
            sql::<diesel::sql_types::Bool>("starts_with(CAST(id AS TEXT), ")
                .bind::<diesel::sql_types::Text, _>(partial_id)
                .sql(")"),

            // if only_active was given, only display sessions which haven't yet begun
            @filter_if (only_active) => sessions::start_at.ge(chrono::Utc::now());

            @order_by: sessions::start_at.asc()
        )
    }

    /// Searches for Sessions starting after a certain point in time,
    /// in ascending order (starting earlier first).
    pub async fn find_starts_after<T: chrono::TimeZone>(
        &self,
        starts_after: chrono::DateTime<T>,
    ) -> Result<Vec<Session>>
    where
        <T as chrono::TimeZone>::Offset: Send + Sync,
    {
        repo_find_by!(
            self, sessions::table;
            sessions::start_at.ge(starts_after);
            @order_by: sessions::start_at.asc()
        )
    }

    /// Searches for all Sessions starting after the current point in time,
    /// in ascending order (starting earlier first).
    pub async fn find_will_start(&self) -> Result<Vec<Session>> {
        self.find_starts_after(chrono::Utc::now()).await
    }

    /// Searches for all Sessions starting before a certain point in time,
    /// in ascending order (starting earlier first).
    pub async fn find_starts_before<T: chrono::TimeZone>(
        &self,
        starts_before: chrono::DateTime<T>,
    ) -> Result<Vec<Session>>
    where
        <T as chrono::TimeZone>::Offset: Send + Sync,
    {
        repo_find_by!(
            self, sessions::table;
            sessions::start_at.le(starts_before);
            @order_by: sessions::start_at.asc()
        )
    }
}

#[async_trait]
impl Repository for SessionRepository {
    type Table = sessions::table;

    type Entity = Session;

    type NewEntity = NewSession;

    type PrimaryKey = i64;

    const TABLE: Self::Table = sessions::table;

    fn get_connection_pool(&self) -> Arc<Pool<AsyncPgConnection>> {
        Arc::clone(&self.pool)
    }

    /// Gets a Session by its ID.
    async fn get(&self, id: i64) -> Result<Option<Session>> {
        repo_get!(self, sessions::table; id)
    }

    async fn insert(&self, session: &NewSession) -> Result<Session> {
        repo_insert!(self, sessions::table; session)
    }

    async fn remove(&self, session: &Session) -> Result<usize> {
        repo_remove!(self; session)
    }

    async fn find_all(&self) -> Result<Vec<Session>> {
        repo_find_all!(self, sessions::table, sessions::table; @order_by: sessions::start_at.asc())
    }
}

#[async_trait]
impl UpdatableRepository for SessionRepository {
    type PartialEntity = PartialSession;

    async fn upsert(&self, session: &NewSession) -> Result<Session> {
        repo_upsert!(self, sessions::table; /*conflict_columns=*/sessions::id; session)
    }

    async fn update(&self, old_session: &Session, new_session: PartialSession) -> Result<Session> {
        repo_update!(self; old_session => new_session)
    }
}
