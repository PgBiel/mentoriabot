use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::RunQueryDsl;

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
pub struct LectureRepository;

impl LectureRepository {
    /// Searches for Lectures by a particular teacher (with a particular Discord ID),
    /// in ascending 'start_at' order (starting earlier first).
    pub async fn find_by_teacher(
        conn: &mut diesel_async::AsyncPgConnection,
        teacher_id: DiscordId,
    ) -> Result<Vec<Lecture>> {
        repo_find_by!(
            conn, lectures::table;
            lectures::teacher_id.eq(teacher_id);
            @order_by: lectures::start_at.asc()
        )
    }

    /// Searches for Lectures starting after a certain point in time,
    /// in ascending order (starting earlier first).
    pub async fn find_starts_after<T: chrono::TimeZone>(
        conn: &mut diesel_async::AsyncPgConnection,
        starts_after: chrono::DateTime<T>,
    ) -> Result<Vec<Lecture>>
    where
        <T as chrono::TimeZone>::Offset: Send + Sync,
    {
        repo_find_by!(
            conn, lectures::table;
            lectures::start_at.ge(starts_after);
            @order_by: lectures::start_at.asc()
        )
    }

    /// Searches for all Lectures starting after the current point in time,
    /// in ascending order (starting earlier first).
    pub async fn find_will_start(
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Vec<Lecture>> {
        Self::find_starts_after(conn, chrono::Utc::now()).await
    }

    /// Searches for all Lectures starting before a certain point in time,
    /// in ascending order (starting earlier first).
    pub async fn find_starts_before<T: chrono::TimeZone>(
        conn: &mut diesel_async::AsyncPgConnection,
        starts_before: chrono::DateTime<T>,
    ) -> Result<Vec<Lecture>>
    where
        <T as chrono::TimeZone>::Offset: Send + Sync,
    {
        repo_find_by!(
            conn, lectures::table;
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

    /// Gets a Lecture by its ID.
    async fn get(conn: &mut diesel_async::AsyncPgConnection, id: i64) -> Result<Option<Lecture>> {
        repo_get!(conn, lectures::table; id)
    }

    async fn insert(
        conn: &mut diesel_async::AsyncPgConnection,
        lecture: NewLecture,
    ) -> Result<Lecture> {
        repo_insert!(conn, lectures::table; lecture)
    }

    async fn remove(conn: &mut diesel_async::AsyncPgConnection, lecture: Lecture) -> Result<()> {
        repo_remove!(conn; &lecture)
    }

    async fn find_all(conn: &mut diesel_async::AsyncPgConnection) -> Result<Vec<Lecture>> {
        repo_find_all!(conn, lectures::table, lectures::table; @order_by: lectures::start_at.asc())
    }
}

#[async_trait]
impl Repository for LectureRepository {
    async fn upsert(
        conn: &mut diesel_async::AsyncPgConnection,
        lecture: NewLecture,
    ) -> Result<Lecture> {
        repo_upsert!(conn, lectures::table; /*conflict_columns=*/lectures::id; &lecture)
    }

    async fn update(
        conn: &mut diesel_async::AsyncPgConnection,
        old_lecture: Lecture,
        new_lecture: NewLecture,
    ) -> Result<Lecture> {
        repo_update!(conn; &old_lecture => new_lecture)
    }
}
