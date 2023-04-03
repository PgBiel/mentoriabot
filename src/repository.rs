use std::sync::Arc;

use async_trait::async_trait;
use diesel::query_builder::AsQuery;
use diesel_async::{pooled_connection::deadpool, AsyncPgConnection};

use crate::error::Result;

mod user;
pub use user::UserRepository;

mod session;
pub use session::SessionRepository;

mod session_student;
pub use session_student::SessionStudentRepository;

pub mod macros;

pub(crate) use macros::{
    repo_find_all, repo_find_by, repo_find_by_first, repo_get, repo_insert, repo_remove,
    repo_update, repo_upsert,
};

/// Trait for a generic entity Repository with insert/remove.
#[async_trait]
pub trait Repository {
    /// The table with the data this repository this will use
    type Table: diesel::Table + diesel::query_dsl::methods::FindDsl<Self::PrimaryKey> + Send + Sync;

    /// The entity type to be queried from the table
    type Entity: diesel::Queryable<<Self::Table as AsQuery>::SqlType, diesel::pg::Pg> + Send + Sync;

    /// The struct used to insert a new entity
    /// (Usually lacks a primary key, as it hasn't been inserted yet)
    type NewEntity: diesel::Insertable<Self::Table> + Send + Sync;

    /// The Primary Key type of the table,
    /// in the models (e.g. (i64, i64); i64; etc.)
    type PrimaryKey;

    const TABLE: Self::Table;

    /// Returns the active connection pool.
    fn get_connection_pool(&self) -> Arc<deadpool::Pool<AsyncPgConnection>>;

    /// Locks the connection for own usage.
    async fn lock_connection(&self) -> Result<deadpool::Object<AsyncPgConnection>> {
        self.get_connection_pool().get().await.map_err(From::from)
    }

    /// Gets an entity by their Primary Key.
    async fn get(&self, pk: Self::PrimaryKey) -> Result<Option<Self::Entity>>;

    /// Insert a new Entity to the database.
    async fn insert(&self, new_entity: &Self::NewEntity) -> Result<Self::Entity>;

    /// Remove an Entity from the database.
    async fn remove(&self, entity: &Self::Entity) -> Result<usize>;

    /// Find all entities stored in the database.
    async fn find_all(&self) -> Result<Vec<Self::Entity>>;
}

/// Trait for a full-fledged repository which can also update.
#[async_trait]
pub trait UpdatableRepository: Repository {
    /// A type that represents the data that will be changed in an existing entity.
    /// It is usually composed exclusively of optional (Option) fields, such that
    /// certain fields (specified as None) will remain unchanged after the update.
    type PartialEntity: diesel::AsChangeset<Target = Self::Table> + Send + Sync;

    /// Insert a new Entity to the database, or update if it already exists.
    async fn upsert(&self, new_entity: &Self::NewEntity) -> Result<Self::Entity>;

    /// Update an existing Entity with new data.
    async fn update(
        &self,
        old_entity: &Self::Entity,
        new_entity: Self::PartialEntity,
    ) -> Result<Self::Entity>;
}
