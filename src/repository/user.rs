use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::RunQueryDsl;

use super::{
    repo_find_all, repo_find_by_first, repo_get, repo_insert, repo_remove, repo_update,
    repo_upsert, BasicRepository, Repository,
};
use crate::{
    error::Result,
    model::{DiscordId, Lecture, LectureStudent, NewUser, User},
    schema::{lecture_students, lectures, users},
};

/// Manages User instances.
pub struct UserRepository;

impl UserRepository {
    /// Searches for all Users that are Students of
    /// a particular Lecture.
    async fn find_by_lecture(
        conn: &mut diesel_async::AsyncPgConnection,
        lecture: &Lecture,
    ) -> Result<Vec<User>> {
        users::table
            .inner_join(lecture_students::table)
            .filter(lecture_students::lecture_id.eq(lecture.id))
            .get_results(conn)
            .await
            .map(|v: Vec<(User, LectureStudent)>| {
                // get just the User (we don't need the LectureStudent).
                v.into_iter().map(|x| x.0).collect()
            })
            .map_err(From::from)
    }
}

#[async_trait]
impl BasicRepository for UserRepository {
    type Table = users::table;

    type Entity = User;

    type NewEntity = NewUser;

    type PrimaryKey = DiscordId;

    const TABLE: Self::Table = users::table;

    async fn get(
        conn: &mut diesel_async::AsyncPgConnection,
        discord_id: DiscordId,
    ) -> Result<Option<User>> {
        repo_get!(conn, users::table; discord_id)
    }

    async fn insert(conn: &mut diesel_async::AsyncPgConnection, user: NewUser) -> Result<User> {
        repo_insert!(conn, users::table; user)
    }

    async fn remove(conn: &mut diesel_async::AsyncPgConnection, user: User) -> Result<()> {
        repo_remove!(conn; &user)
    }

    async fn find_all(conn: &mut diesel_async::AsyncPgConnection) -> Result<Vec<User>> {
        repo_find_all!(conn, users::table, users::table)
    }
}

#[async_trait]
impl Repository for UserRepository {
    async fn upsert(conn: &mut diesel_async::AsyncPgConnection, user: NewUser) -> Result<User> {
        repo_upsert!(conn, users::table; /*conflict_columns=*/users::discord_id; &user)
    }

    async fn update(
        conn: &mut diesel_async::AsyncPgConnection,
        old_user: User,
        new_user: NewUser,
    ) -> Result<User> {
        repo_update!(conn; &old_user => new_user)
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::connection::create_connection;

//     #[test]
//     fn test_basic_operations() {
//         let db_url = std::env::var("DATABASE_URL").expect("Testing database requires DATABASE_URL
// env var.");         let conn = create_connection(&db_url);

//     }
// }
