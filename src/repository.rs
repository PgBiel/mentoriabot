use async_trait::async_trait;

use crate::error::Result;

mod user;

#[async_trait]
pub trait Repository {

    /// The type for SQL operations using this repository's table
    /// (A tuple with each of the columns' types)
    type SqlType;

    /// The table with the data this repository this will use
    type Table: diesel::Table;

    /// The entity type to be queried from the table
    type Entity: diesel::Queryable<Self::SqlType, diesel::pg::Pg>;

    /// The struct used to insert a new entity
    /// (Usually lacks a primary key, as it hasn't been inserted yet)
    type NewEntity: diesel::Insertable<Self::Table>;
    type Column: diesel::Column<Table = Self::Table, SqlType = Self::SqlType> + diesel::expression::ValidGrouping<()>;

    const TABLE: Self::Table;
    const ID_COLUMN: Self::Column;

    async fn insert(
        conn: &mut diesel_async::AsyncPgConnection,
        user: &Self::NewEntity
    ) -> Result<Self::Entity> {
        let inserted_id: i32 = diesel::insert_into(Self::TABLE)
            .values(user)
            .returning(Self::ID_COLUMN)
            .get_result(conn)
            .await?;

        let user: Self::Entity = Self::TABLE.filter(Self::ID_COLUMN.eq(inserted_id)).first(conn).await?;

        Ok(user)
    }
}
