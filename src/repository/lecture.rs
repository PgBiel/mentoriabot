use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::RunQueryDsl;

use super::{
    repo_find_all, repo_get_by_id, repo_insert, repo_remove, repo_update, repo_upsert, Repository,
};
use crate::{
    error::Result,
    model::{DiscordId, Lecture, NewLecture},
    schema::lectures,
};

/// Manages Lecture instances.
pub struct LectureRepository;

impl LectureRepository {
    /// Gets a Lecture by its ID.
    pub async fn get(
        conn: &mut diesel_async::AsyncPgConnection,
        id: i64,
    ) -> Result<Option<Lecture>> {
        repo_get_by_id!(conn, lectures::table, /*id_column=*/lectures::id; id)
    }

    /// Searches for Lectures by a particular teacher (with a particular Discord ID).
    pub async fn find_by_teacher(
        conn: &mut diesel_async::AsyncPgConnection,
        teacher_id: DiscordId,
    ) -> Result<Vec<Lecture>> {
        lectures::table
            .filter(lectures::teacher_id.eq(teacher_id))
            .get_results(conn)
            .await
            .map_err(From::from)
    }

    /// Searches for Lectures starting after a certain point in time.
    pub async fn find_starts_after<T: chrono::TimeZone>(
        conn: &mut diesel_async::AsyncPgConnection,
        starts_after: chrono::DateTime<T>,
    ) -> Result<Vec<Lecture>>
    where
        <T as chrono::TimeZone>::Offset: Send + Sync,
    {
        lectures::table
            .filter(lectures::start_at.ge(starts_after))
            .get_results(conn)
            .await
            .map_err(From::from)
    }
}

#[async_trait]
impl Repository for LectureRepository {
    type Table = lectures::table;

    type Entity = Lecture;

    type NewEntity = NewLecture;

    const TABLE: Self::Table = lectures::table;

    async fn insert(
        conn: &mut diesel_async::AsyncPgConnection,
        lecture: NewLecture,
    ) -> Result<Lecture> {
        repo_insert!(conn, lectures::table; lecture)
    }

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

    async fn remove(conn: &mut diesel_async::AsyncPgConnection, lecture: Lecture) -> Result<()> {
        repo_remove!(conn; &lecture)
    }

    async fn find_all(conn: &mut diesel_async::AsyncPgConnection) -> Result<Vec<Lecture>> {
        repo_find_all!(conn, lectures::table, lectures::table)
    }
}
