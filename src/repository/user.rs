use async_trait::async_trait;
use diesel::{QueryDsl, ExpressionMethods, OptionalExtension};
use diesel_async::RunQueryDsl;

use crate::{model::{User, NewUser}, schema::users, error::Result};

use super::{repo_insert, Repository, repo_update, repo_remove, repo_get_by_id};

/// Manages User instances.
pub struct UserRepository;

impl UserRepository {
    async fn get(
        conn: &mut diesel_async::AsyncPgConnection,
        id: i32
    ) -> Result<Option<User>> {
        repo_get_by_id!(conn, users::table, /*id_column=*/users::id; id)
    }
}

#[async_trait]
impl Repository for UserRepository {
    type Table = users::table;

    type Entity = User;

    type NewEntity = NewUser;

    const TABLE: Self::Table = users::table;

    async fn insert(conn: &mut diesel_async::AsyncPgConnection, user: &NewUser) -> Result<User> {
        repo_insert!(conn, users::table; user)
    }

    async fn update(conn: &mut diesel_async::AsyncPgConnection, old_user: &User, new_user: &NewUser) -> Result<User> {
        repo_update!(conn; old_user => new_user)
    }

    async fn remove(conn: &mut diesel_async::AsyncPgConnection, user: &User) -> Result<()> {
        repo_remove!(conn; user)
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
