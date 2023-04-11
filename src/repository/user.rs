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
    model::{DiscordId, NewUser, PartialUser, Session, SessionStudent, Teacher, User},
    schema::{session_students, teachers, users},
};
use crate::error::Error;

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

    /// Gets a User from the database by their Discord ID,
    /// or inserts them instead.
    pub async fn get_or_insert(&self, user: &NewUser) -> Result<User> {
        if let Some(found_user) = self.get(user.discord_id).await? {
            Ok(found_user)
        } else {
            self.insert(user).await
        }
    }

    /// Attempts to insert a User; does nothing if such a User is already registered.
    /// Returns the inserted row count (1 if a new User was inserted or 0 otherwise).
    pub async fn insert_if_not_exists(&self, user: &NewUser) -> Result<usize> {
        diesel::insert_into(users::table)
            .values(user)
            .on_conflict_do_nothing()
            .execute(&mut self.lock_connection().await?)
            .await
            .map_err(From::from)
    }

    /// Searches for a Teacher's User instance.
    pub async fn find_by_teacher(&self, teacher: &Teacher) -> Result<User> {
        self.get(teacher.user_id)
            .await?
            .ok_or_else(|| Error::Other("Could not find User for a certain teacher!"))
    }

    /// Searches for all Users that are Students of
    /// a particular Session.
    pub async fn find_by_session(&self, session: &Session) -> Result<Vec<User>> {
        users::table
            .inner_join(session_students::table)
            .filter(session_students::session_id.eq(session.id))
            .get_results(&mut self.lock_connection().await?)
            .await
            .map(|v: Vec<(User, SessionStudent)>| {
                // get just the User (we don't need the SessionStudent).
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

    async fn insert(&self, user: &NewUser) -> Result<User> {
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

    async fn upsert(&self, user: &NewUser) -> Result<User> {
        repo_upsert!(self, users::table; /*conflict_columns=*/users::discord_id; user)
    }

    async fn update(&self, old_user: &User, new_user: PartialUser) -> Result<User> {
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
