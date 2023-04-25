use std::sync::Arc;

use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};

use crate::{
    error::Result,
    repository::{AvailabilityRepository, SessionRepository, UserRepository},
};
use crate::repository::TeacherRepository;

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

impl DatabaseManager {
    /// Creates a Database Manager by initializing a connection
    /// to the given Database URL.
    pub fn new(database_url: &str) -> Result<Self> {
        let pool = create_connection_pool(database_url)?;
        let pool = Arc::new(pool);

        let user_repository = UserRepository::new(&pool);
        let session_repository = SessionRepository::new(&pool);
        let teacher_repository = TeacherRepository::new(&pool);
        let availability_repository = AvailabilityRepository::new(&pool);

        Ok(Self {
            pool,
            user_repository,
            session_repository,
            teacher_repository,
            availability_repository,
        })
    }

    /// Gets the connection held by this DatabaseManager.
    pub fn get_connection_pool(&self) -> Arc<Pool<AsyncPgConnection>> {
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
