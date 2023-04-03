use std::sync::Arc;

use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use super::{repo_find_all, repo_find_by, repo_get, repo_insert, repo_remove, Repository};
use crate::{
    error::Result,
    model::{DiscordId, NewSessionStudent, Session, SessionStudent, User},
    schema::session_students,
};

/// Manages SessionStudent instances, which are basically associations
/// that determine that a given User is a student in a given Session.
#[derive(Clone)]
pub struct SessionStudentRepository {
    pool: Arc<Pool<AsyncPgConnection>>,
}

impl SessionStudentRepository {
    /// Creates a new SessionStudentRepository operating with the given
    /// connection pool.
    pub fn new(pool: &Arc<Pool<AsyncPgConnection>>) -> Self {
        Self {
            pool: Arc::clone(pool),
        }
    }

    /// Inserts a SessionStudent for the given User and Session,
    /// thus marking that User as a student of that Session.
    pub async fn insert_for_user_and_session(
        &self,
        user: &User,
        session: &Session,
    ) -> Result<SessionStudent> {
        Self::insert(
            self,
            &NewSessionStudent {
                session_id: session.id,
                user_id: user.discord_id,
            },
        )
        .await
    }

    /// Finds a SessionStudent instance related to a User and a Session.
    pub async fn find_by_user_and_session(
        &self,
        user: &User,
        session: &Session,
    ) -> Result<Option<SessionStudent>> {
        Self::get(self, (user.discord_id, session.id)).await
    }

    /// Gets all SessionStudents belonging to a certain Session.
    pub async fn find_by_session(&self, session_id: i64) -> Result<Vec<SessionStudent>> {
        repo_find_by!(self, session_students::table; session_students::session_id.eq(session_id))
    }

    /// Searches for all instances of SessionStudent for a certain User.
    pub async fn find_by_user(&self, user_id: DiscordId) -> Result<Vec<SessionStudent>> {
        session_students::table
            .filter(session_students::user_id.eq(user_id))
            .get_results(&mut self.lock_connection().await?)
            .await
            .map_err(From::from)
    }
}

#[async_trait]
impl Repository for SessionStudentRepository {
    type Table = session_students::table;

    type Entity = SessionStudent;

    type NewEntity = NewSessionStudent;

    type PrimaryKey = (DiscordId, i64);

    const TABLE: Self::Table = session_students::table;

    fn get_connection_pool(&self) -> Arc<Pool<AsyncPgConnection>> {
        Arc::clone(&self.pool)
    }

    async fn get(&self, pk: Self::PrimaryKey) -> Result<Option<SessionStudent>> {
        repo_get!(self, session_students::table; pk)
    }

    async fn insert(&self, session: &NewSessionStudent) -> Result<SessionStudent> {
        repo_insert!(self, session_students::table; session)
    }

    async fn remove(&self, session: &SessionStudent) -> Result<usize> {
        repo_remove!(self; session)
    }

    async fn find_all(&self) -> Result<Vec<SessionStudent>> {
        repo_find_all!(self, session_students::table, session_students::table)
    }
}
