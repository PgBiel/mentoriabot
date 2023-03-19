use std::sync::Arc;

use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use super::{
    repo_find_all, repo_find_by, repo_get, repo_insert, repo_remove, repo_update, repo_upsert,
    BasicRepository, Repository,
};
use crate::{
    error::Result,
    model::{DiscordId, Lecture, NewLecture},
    schema::lectures,
};

/// Manages Lecture instances.
#[derive(Clone)]
pub struct LectureRepository {
    pool: Arc<Pool<AsyncPgConnection>>,
}

impl LectureRepository {
    /// Creates a new LectureRepository operating with the given
    /// connection pool.
    pub fn new(pool: &Arc<Pool<AsyncPgConnection>>) -> Self {
        Self {
            pool: Arc::clone(pool),
        }
    }

    /// Searches for Lectures by a particular teacher (with a particular Discord ID),
    /// in ascending 'start_at' order (starting earlier first).
    pub async fn find_by_teacher(&self, teacher_id: DiscordId) -> Result<Vec<Lecture>> {
        repo_find_by!(
            self, lectures::table;
            lectures::teacher_id.eq(teacher_id);
            @order_by: lectures::start_at.asc()
        )
    }

    /// Searches for Lectures starting after a certain point in time,
    /// in ascending order (starting earlier first).
    pub async fn find_starts_after<T: chrono::TimeZone>(
        &self,
        starts_after: chrono::DateTime<T>,
    ) -> Result<Vec<Lecture>>
    where
        <T as chrono::TimeZone>::Offset: Send + Sync,
    {
        repo_find_by!(
            self, lectures::table;
            lectures::start_at.ge(starts_after);
            @order_by: lectures::start_at.asc()
        )
    }

    /// Searches for all Lectures starting after the current point in time,
    /// in ascending order (starting earlier first).
    pub async fn find_will_start(&self) -> Result<Vec<Lecture>> {
        self.find_starts_after(chrono::Utc::now()).await
    }

    /// Searches for all Lectures starting before a certain point in time,
    /// in ascending order (starting earlier first).
    pub async fn find_starts_before<T: chrono::TimeZone>(
        &self,
        starts_before: chrono::DateTime<T>,
    ) -> Result<Vec<Lecture>>
    where
        <T as chrono::TimeZone>::Offset: Send + Sync,
    {
        repo_find_by!(
            self, lectures::table;
            lectures::start_at.le(starts_before);
            @order_by: lectures::start_at.asc()
        )
    }
}

#[async_trait]
impl BasicRepository for LectureRepository {
    type Table = lectures::table;

    type Entity = Lecture;

    type NewEntity = NewLecture;

    type PrimaryKey = i64;

    const TABLE: Self::Table = lectures::table;

    fn get_connection_pool(&self) -> Arc<Pool<AsyncPgConnection>> {
        Arc::clone(&self.pool)
    }

    /// Gets a Lecture by its ID.
    async fn get(&self, id: i64) -> Result<Option<Lecture>> {
        repo_get!(self, lectures::table; id)
    }

    async fn insert(&self, lecture: NewLecture) -> Result<Lecture> {
        repo_insert!(self, lectures::table; lecture)
    }

    async fn remove(&self, lecture: &Lecture) -> Result<usize> {
        repo_remove!(self; lecture)
    }

    async fn find_all(&self) -> Result<Vec<Lecture>> {
        repo_find_all!(self, lectures::table, lectures::table; @order_by: lectures::start_at.asc())
    }
}

#[async_trait]
impl Repository for LectureRepository {
    async fn upsert(&self, lecture: NewLecture) -> Result<Lecture> {
        repo_upsert!(self, lectures::table; /*conflict_columns=*/lectures::id; &lecture)
    }

    async fn update(&self, old_lecture: &Lecture, new_lecture: NewLecture) -> Result<Lecture> {
        repo_update!(self; old_lecture => new_lecture)
    }
}
