use async_trait::async_trait;
use diesel::{query_builder::{InsertStatement, UpdateStatement, IntoUpdateTarget, AsQuery, DeleteStatement}};
use diesel_async::{RunQueryDsl, methods::LoadQuery, AsyncPgConnection};

use crate::error::Result;

mod user;

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
        new_entity: &Self::NewEntity
    ) -> Result<Self::Entity>
    where  // don't mind the witchery... just the first two lines matter - the last two are for async/threading reasons
        for<'a> &'a Self::NewEntity: diesel::Insertable<Self::Table>,
        Self::Table: for<'a> LoadQuery<'a, AsyncPgConnection, Self::NewEntity>,
        for<'a> InsertStatement<Self::Table, <&'a Self::NewEntity as diesel::Insertable<Self::Table>>::Values>: LoadQuery<'a, AsyncPgConnection, Self::Entity>,
        // ---
        // Async bounds:
        <Self::Table as diesel::QuerySource>::FromClause: Send + Sync,
        for<'a> <&'a Self::NewEntity as diesel::Insertable<Self::Table>>::Values: Send
    {
        let entity = diesel::insert_into(Self::TABLE)
            .values(new_entity)
            .get_result::<Self::Entity>(conn)
            .await?;

        Ok(entity)
    }

    async fn update(
        conn: &mut AsyncPgConnection,
        old_entity: &Self::Entity,
        new_entity: &Self::NewEntity
    ) -> Result<Self::Entity>
    where
        // Diesel bounds:
        for<'a> &'a Self::Entity: diesel::Identifiable<Table = Self::Table> + diesel::query_builder::IntoUpdateTarget,
        for<'a> &'a Self::NewEntity: diesel::AsChangeset<Target = Self::Table>,
        for<'a,'b,'c> UpdateStatement<Self::Table, <&'a Self::Entity as IntoUpdateTarget>::WhereClause, <&'b Self::NewEntity as diesel::AsChangeset>::Changeset>: AsQuery
            + LoadQuery<'c, AsyncPgConnection, Self::Entity>,
        // ---
        // Async bounds:
        <Self::Table as diesel::QuerySource>::FromClause: Send + Sync,
        for<'a> <&'a Self::Entity as IntoUpdateTarget>::WhereClause: Send + Sync,
        for<'a> <&'a Self::NewEntity as diesel::AsChangeset>::Changeset: Send + Sync,
    {
        let entity = diesel::update(old_entity)
            .set(new_entity)
            .get_result::<Self::Entity>(conn)
            .await?;

        Ok(entity)
    }

    async fn remove(
        conn: &mut AsyncPgConnection,
        entity: &Self::Entity
    ) -> Result<()>
    where
        // Diesel bounds:
        for<'a> &'a Self::Entity: diesel::Identifiable<Table = Self::Table> + diesel::query_builder::IntoUpdateTarget,
        for<'a> DeleteStatement<Self::Table, <&'a Self::Entity as IntoUpdateTarget>::WhereClause>: diesel_async::methods::ExecuteDsl<AsyncPgConnection>,
        // ---
        // Async bounds:
        <Self::Table as diesel::QuerySource>::FromClause: Send + Sync,
        for<'a> <&'a Self::Entity as IntoUpdateTarget>::WhereClause: Send + Sync,
    {
        diesel::delete(entity)
            .execute(conn)
            .await?;

        Ok(())
    }
}
