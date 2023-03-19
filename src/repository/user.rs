use std::sync::Arc;

use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use super::{
    repo_find_all, repo_find_by_first, repo_get, repo_insert, repo_remove, repo_update,
    repo_upsert, Repository, UpdatableRepository,
};
use crate::{
    error::Result,
    model::{DiscordId, Lecture, LectureStudent, NewUser, User, PartialUser},
    schema::{lecture_students, lectures, users},
};

/// Manages User instances.
#[derive(Clone)]
pub struct UserRepository {
    pool: Arc<Pool<AsyncPgConnection>>,
}

impl UserRepository {
    /// Creates a new UserRepository operating with the given
    /// connection pool.
    pub fn new(pool: &Arc<Pool<AsyncPgConnection>>) -> Self {
        Self {
            pool: Arc::clone(pool),
        }
    }

    /// Searches for all Users that are Students of
    /// a particular Lecture.
    pub async fn find_by_lecture(&self, lecture: &Lecture) -> Result<Vec<User>> {
        users::table
            .inner_join(lecture_students::table)
            .filter(lecture_students::lecture_id.eq(lecture.id))
            .get_results(&mut self.lock_connection().await?)
            .await
            .map(|v: Vec<(User, LectureStudent)>| {
                // get just the User (we don't need the LectureStudent).
                v.into_iter().map(|x| x.0).collect()
            })
            .map_err(From::from)
    }
}

#[async_trait]
impl Repository for UserRepository {
    type Table = users::table;

    type Entity = User;

    type NewEntity = NewUser;

    type PrimaryKey = DiscordId;

    const TABLE: Self::Table = users::table;

    fn get_connection_pool(&self) -> Arc<Pool<AsyncPgConnection>> {
        Arc::clone(&self.pool)
    }

    /// Gets a User by their Discord ID.
    async fn get(&self, discord_id: DiscordId) -> Result<Option<User>> {
        repo_get!(self, users::table; discord_id)
    }

    async fn insert(&self, user: NewUser) -> Result<User> {
        repo_insert!(self, users::table; user)
    }

    async fn remove(&self, user: &User) -> Result<usize> {
        repo_remove!(self; user)
    }

    async fn find_all(&self) -> Result<Vec<User>> {
        repo_find_all!(self, users::table, users::table)
    }
}

#[async_trait]
impl UpdatableRepository for UserRepository {
    type PartialEntity = PartialUser;

    async fn upsert(&self, user: NewUser) -> Result<User> {
        repo_upsert!(self, users::table; /*conflict_columns=*/users::discord_id; &user)
    }

    async fn update(
        &self, old_user: &User, new_user: PartialUser
    ) -> Result<User> {
        repo_update!(self; old_user => new_user)
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
