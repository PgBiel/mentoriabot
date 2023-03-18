use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use super::{repo_find_all, repo_find_by, repo_get, repo_insert, repo_remove, BasicRepository};
use crate::{
    error::Result,
    model::{DiscordId, Lecture, LectureStudent, NewLectureStudent, User},
    schema::lecture_students,
};

/// Manages LectureStudent instances, which are basically associations
/// that determine that a given User is a student in a given Lecture.
pub struct LectureStudentRepository;

impl LectureStudentRepository {
    /// Inserts a LectureStudent for the given User and Lecture,
    /// thus marking that User as a student of that Lecture.
    pub async fn insert_for_user_and_lecture(
        conn: &mut AsyncPgConnection,
        user: &User,
        lecture: &Lecture,
    ) -> Result<LectureStudent> {
        Self::insert(
            conn,
            NewLectureStudent {
                lecture_id: lecture.id,
                user_id: user.discord_id,
            },
        )
        .await
    }

    /// Finds a LectureStudent instance related to a User and a Lecture.
    pub async fn find_by_user_and_lecture(
        conn: &mut AsyncPgConnection,
        user: &User,
        lecture: &Lecture,
    ) -> Result<Option<LectureStudent>> {
        Self::get(conn, (user.discord_id, lecture.id)).await
    }

    /// Gets all LectureStudents belonging to a certain Lecture.
    pub async fn find_by_lecture(
        conn: &mut AsyncPgConnection,
        lecture_id: i64,
    ) -> Result<Vec<LectureStudent>> {
        repo_find_by!(conn, lecture_students::table; lecture_students::lecture_id.eq(lecture_id))
    }

    /// Searches for all instances of LectureStudent for a certain User.
    pub async fn find_by_user(
        conn: &mut AsyncPgConnection,
        user_id: DiscordId,
    ) -> Result<Vec<LectureStudent>> {
        lecture_students::table
            .filter(lecture_students::user_id.eq(user_id))
            .get_results(conn)
            .await
            .map_err(From::from)
    }
}

#[async_trait]
impl BasicRepository for LectureStudentRepository {
    type Table = lecture_students::table;

    type Entity = LectureStudent;

    type NewEntity = NewLectureStudent;

    type PrimaryKey = (DiscordId, i64);

    const TABLE: Self::Table = lecture_students::table;

    async fn get(
        conn: &mut AsyncPgConnection,
        pk: Self::PrimaryKey,
    ) -> Result<Option<LectureStudent>> {
        repo_get!(conn, lecture_students::table; pk)
    }

    async fn insert(
        conn: &mut AsyncPgConnection,
        lecture: NewLectureStudent,
    ) -> Result<LectureStudent> {
        repo_insert!(conn, lecture_students::table; lecture)
    }

    async fn remove(conn: &mut AsyncPgConnection, lecture: LectureStudent) -> Result<()> {
        repo_remove!(conn; &lecture)
    }

    async fn find_all(conn: &mut AsyncPgConnection) -> Result<Vec<LectureStudent>> {
        repo_find_all!(conn, lecture_students::table, lecture_students::table)
    }
}
