use std::sync::Arc;

use diesel_async::{
    AsyncConnection, AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool}
};

use crate::error::Result;
use crate::repository::{LectureRepository, LectureStudentRepository, UserRepository};

/// Manages database Connection and Repository objects, using a
/// connection [`Pool`].
pub struct DatabaseManager {
    pool: Arc<Pool<AsyncPgConnection>>,
    user_repository: UserRepository,
    lecture_repository: LectureRepository,
    lecture_student_repository: LectureStudentRepository,
}

/// General function for creating a connection pool to the database.
pub fn create_connection_pool(database_url: &str) -> Result<Pool<AsyncPgConnection>> {
    let manager = AsyncDieselConnectionManager::new(database_url);

    Pool::builder(manager)
        .build()
        .map_err(From::from)
}

impl DatabaseManager {
    /// Creates a Database Manager by initializing a connection
    /// to the given Database URL.
    pub fn new(database_url: &str) -> Result<Self> {
        let pool = create_connection_pool(database_url)?;
        let pool = Arc::new(pool);

        let user_repository = UserRepository::new(&pool);
        let lecture_repository = LectureRepository::new(&pool);
        let lecture_student_repository = LectureStudentRepository::new(&pool);

        Ok(Self {
            pool,
            user_repository,
            lecture_repository,
            lecture_student_repository
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

    /// Returns a [`LectureRepository`] object using the current connection pool.
    pub fn lecture_repository(&self) -> &LectureRepository {
        &self.lecture_repository
    }

    /// Returns a [`LectureStudentRepository`] object using the current connection pool.
    pub fn lecture_student_repository(&self) -> &LectureStudentRepository {
        &self.lecture_student_repository
    }
}
