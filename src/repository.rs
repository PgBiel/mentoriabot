use async_trait::async_trait;
use diesel::{query_builder::{InsertStatement, UpdateStatement, IntoUpdateTarget, AsQuery, DeleteStatement}};
use diesel_async::{RunQueryDsl, methods::LoadQuery, AsyncPgConnection};

use crate::error::Result;

mod user;
pub use user::UserRepository;

pub mod macros;
pub(crate) use macros::{repo_insert, repo_update, repo_remove, repo_get_by_id};

#[async_trait]
pub trait Repository {

    /// The table with the data this repository this will use
    type Table: diesel::Table + Send + Sync;

    /// The entity type to be queried from the table
    type Entity: diesel::Queryable<<Self::Table as diesel::query_builder::AsQuery>::SqlType, diesel::pg::Pg>
        + Send + Sync;

    /// The struct used to insert a new entity
    /// (Usually lacks a primary key, as it hasn't been inserted yet)
    type NewEntity: diesel::Insertable<Self::Table> + Send + Sync;

    const TABLE: Self::Table;

    async fn insert(
        conn: &mut AsyncPgConnection,
        new_entity: Self::NewEntity
    ) -> Result<Self::Entity>;

    async fn update(
        conn: &mut AsyncPgConnection,
        old_entity: Self::Entity,
        new_entity: Self::NewEntity
    ) -> Result<Self::Entity>;

    async fn remove(
        conn: &mut AsyncPgConnection,
        entity: Self::Entity
    ) -> Result<()>;
}
