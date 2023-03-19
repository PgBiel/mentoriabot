use std::sync::Arc;

use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use super::{repo_find_all, repo_find_by, repo_get, repo_insert, repo_remove, Repository};
use crate::{
    error::Result,
    model::{DiscordId, Lecture, LectureStudent, NewLectureStudent, User},
    schema::lecture_students,
};

/// Manages LectureStudent instances, which are basically associations
/// that determine that a given User is a student in a given Lecture.
#[derive(Clone)]
pub struct LectureStudentRepository {
    pool: Arc<Pool<AsyncPgConnection>>,
}

impl LectureStudentRepository {
    /// Creates a new LectureStudentRepository operating with the given
    /// connection pool.
    pub fn new(pool: &Arc<Pool<AsyncPgConnection>>) -> Self {
        Self {
            pool: Arc::clone(pool),
        }
    }

    /// Inserts a LectureStudent for the given User and Lecture,
    /// thus marking that User as a student of that Lecture.
    pub async fn insert_for_user_and_lecture(
        &self,
        user: &User,
        lecture: &Lecture,
    ) -> Result<LectureStudent> {
        Self::insert(
            self,
            NewLectureStudent {
                lecture_id: lecture.id,
                user_id: user.discord_id,
            },
        )
        .await
    }

    /// Finds a LectureStudent instance related to a User and a Lecture.
    pub async fn find_by_user_and_lecture(
        &self,
        user: &User,
        lecture: &Lecture,
    ) -> Result<Option<LectureStudent>> {
        Self::get(self, (user.discord_id, lecture.id)).await
    }

    /// Gets all LectureStudents belonging to a certain Lecture.
    pub async fn find_by_lecture(&self, lecture_id: i64) -> Result<Vec<LectureStudent>> {
        repo_find_by!(self, lecture_students::table; lecture_students::lecture_id.eq(lecture_id))
    }

    /// Searches for all instances of LectureStudent for a certain User.
    pub async fn find_by_user(&self, user_id: DiscordId) -> Result<Vec<LectureStudent>> {
        lecture_students::table
            .filter(lecture_students::user_id.eq(user_id))
            .get_results(&mut self.lock_connection().await?)
            .await
            .map_err(From::from)
    }
}

#[async_trait]
impl Repository for LectureStudentRepository {
    type Table = lecture_students::table;

    type Entity = LectureStudent;

    type NewEntity = NewLectureStudent;

    type PrimaryKey = (DiscordId, i64);

    const TABLE: Self::Table = lecture_students::table;

    fn get_connection_pool(&self) -> Arc<Pool<AsyncPgConnection>> {
        Arc::clone(&self.pool)
    }

    async fn get(&self, pk: Self::PrimaryKey) -> Result<Option<LectureStudent>> {
        repo_get!(self, lecture_students::table; pk)
    }

    async fn insert(&self, lecture: NewLectureStudent) -> Result<LectureStudent> {
        repo_insert!(self, lecture_students::table; lecture)
    }

    async fn remove(&self, lecture: &LectureStudent) -> Result<usize> {
        repo_remove!(self; lecture)
    }

    async fn find_all(&self) -> Result<Vec<LectureStudent>> {
        repo_find_all!(self, lecture_students::table, lecture_students::table)
    }
}
