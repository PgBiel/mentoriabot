use async_trait::async_trait;
use diesel::{QueryDsl, ExpressionMethods, OptionalExtension, Table};
use diesel_async::RunQueryDsl;

use crate::{model::{User, NewUser}, schema::users, error::{Result, Error}};

use super::{repo_insert, Repository, repo_update, repo_remove, repo_get_by_id};

/// Manages User instances.
pub struct UserRepository;

impl UserRepository {
    pub async fn get(
        conn: &mut diesel_async::AsyncPgConnection,
        id: i32
    ) -> Result<Option<User>> {
        repo_get_by_id!(conn, users::table, /*id_column=*/users::id; id)
    }

    pub async fn find_by_discordid(
        conn: &mut diesel_async::AsyncPgConnection,
        discord_id: u64
    ) -> Result<Option<User>> {
        users::table.filter(users::discord_userid.eq(discord_id.to_string()))
            .first(conn)
            .await
            .optional()
            .map_err(From::from)
    }

    pub async fn find_all(
        conn: &mut diesel_async::AsyncPgConnection
    ) -> Result<Vec<User>> {
        users::table.select(users::table::all_columns())
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
        user.discord_userid.parse::<u64>()
            .map_err(|_| Error::Other("Provided invalid ID for user"))?;

        repo_insert!(conn, users::table; user)
    }

    async fn update(conn: &mut diesel_async::AsyncPgConnection, old_user: User, new_user: NewUser) -> Result<User> {
        new_user.discord_userid.parse::<u64>()
            .map_err(|_| Error::Other("Provided invalid ID for user"))?;

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
//         let db_url = std::env::var("DATABASE_URL").expect("Testing database requires DATABASE_URL env var.");
//         let conn = create_connection(&db_url);
        
//     }
// }
