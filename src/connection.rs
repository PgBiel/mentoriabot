use std::sync::Arc;

use diesel_async::{
    pooled_connection::{
        deadpool::{Hook, HookError, HookErrorCause, Pool},
        AsyncDieselConnectionManager, PoolError,
    },
    AsyncConnection, AsyncPgConnection,
};

use crate::{
    error::Result,
    repository::{AvailabilityRepository, SessionRepository, TeacherRepository, UserRepository},
};

/// Manages database Connection and Repository objects, using a
/// connection [`Pool`].
#[derive(Clone)]
pub struct DatabaseManager {
    pool: Arc<Pool<AsyncPgConnection>>,
    user_repository: UserRepository,
    session_repository: SessionRepository,
    teacher_repository: TeacherRepository,
    availability_repository: AvailabilityRepository,
}

/// General function for creating a connection pool to the database.
pub fn create_connection_pool(database_url: &str) -> Result<Pool<AsyncPgConnection>> {
    let manager = AsyncDieselConnectionManager::new(database_url);

    Pool::builder(manager).build().map_err(From::from)
}

/// Creates a test connection pool (which is rolled back when the connection is dropped).
pub fn create_test_connection_pool(database_url: &str) -> Result<Pool<AsyncPgConnection>> {
    let manager = AsyncDieselConnectionManager::new(database_url);

    Pool::builder(manager)
        .post_create(Hook::async_fn(|pool: &mut AsyncPgConnection, _| {
            Box::pin(async move {
                pool.begin_test_transaction()
                    .await
                    .map_err(PoolError::QueryError)
                    .map_err(HookErrorCause::Backend)
                    .map_err(HookError::Abort)
            })
        }))
        .build()
        .map_err(From::from)
}

impl DatabaseManager {
    /// Creates a Database Manager by initializing a connection
    /// to the given Database URL.
    pub fn new(database_url: &str) -> Result<Self> {
        let pool = create_connection_pool(database_url)?;

        Ok(Self::with_pool(Arc::new(pool)))
    }

    /// Creates a Database Manager with a test connection
    /// (all changes to it are rolled back at the end).
    #[cfg(test)]
    pub fn test(database_url: &str) -> Result<Self> {
        let pool = create_test_connection_pool(database_url)?;

        Ok(Self::with_pool(Arc::new(pool)))
    }

    /// Creates a new Database Manager operating on the given connection pool.
    fn with_pool(pool: Arc<Pool<AsyncPgConnection>>) -> Self {
        let user_repository = UserRepository::new(&pool);
        let session_repository = SessionRepository::new(&pool);
        let teacher_repository = TeacherRepository::new(&pool);
        let availability_repository = AvailabilityRepository::new(&pool);

        Self {
            pool,
            user_repository,
            session_repository,
            teacher_repository,
            availability_repository,
        }
    }

    /// Returns this Database Manager's connection pool.
    pub fn pool(&self) -> Arc<Pool<AsyncPgConnection>> {
        Arc::clone(&self.pool)
    }

    /// Returns a [`UserRepository`] object using the current connection pool.
    pub fn user_repository(&self) -> &UserRepository {
        &self.user_repository
    }

    /// Returns a [`SessionRepository`] object using the current connection pool.
    pub fn session_repository(&self) -> &SessionRepository {
        &self.session_repository
    }

    /// Returns a [`TeacherRepository`] object using the current connection pool.
    pub fn teacher_repository(&self) -> &TeacherRepository {
        &self.teacher_repository
    }

    /// Returns an [`AvailabilityRepository`] object using the current connection pool.
    pub fn availability_repository(&self) -> &AvailabilityRepository {
        &self.availability_repository
    }
}
