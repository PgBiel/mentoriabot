use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, Table};
use diesel_async::RunQueryDsl;

use super::{repo_get_by_id, repo_insert, repo_remove, repo_update, repo_upsert, Repository};
use crate::{
    error::Result,
    model::{DiscordId, NewUser, User},
    schema::users,
};

/// Manages User instances.
pub struct UserRepository;

impl UserRepository {
    pub async fn get(
        conn: &mut diesel_async::AsyncPgConnection,
        discord_id: DiscordId
    ) -> Result<Option<User>> {
        repo_get_by_id!(conn, users::table, /*id_column=*/users::discord_id; discord_id)
    }

    pub async fn find_all(conn: &mut diesel_async::AsyncPgConnection) -> Result<Vec<User>> {
        users::table
            .select(users::table::all_columns())
            .get_results(conn)
            .await
            .map_err(From::from)
    }
}

#[async_trait]
impl Repository for UserRepository {
    type Table = users::table;

    type Entity = User;

    type NewEntity = NewUser;

    const TABLE: Self::Table = users::table;

    async fn insert(conn: &mut diesel_async::AsyncPgConnection, user: NewUser) -> Result<User> {
        repo_insert!(conn, users::table; user)
    }

    async fn upsert(conn: &mut diesel_async::AsyncPgConnection, user: NewUser) -> Result<User> {
        repo_upsert!(conn, users::table; /*conflict_columns=*/users::discord_id; &user)
    }

    async fn update(
        conn: &mut diesel_async::AsyncPgConnection,
        old_user: User,
        new_user: NewUser,
    ) -> Result<User> {
        repo_update!(conn; &old_user => new_user)
    }

    async fn remove(conn: &mut diesel_async::AsyncPgConnection, user: User) -> Result<()> {
        repo_remove!(conn; &user)
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
