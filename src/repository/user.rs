use diesel::{QueryDsl, ExpressionMethods};
use diesel_async::RunQueryDsl;

use crate::{model::{User, NewUser}, schema::users, error::Result};

/// Manages User instances.
pub struct UserRepository;

impl UserRepository {

    async fn insert(conn: &mut diesel_async::AsyncPgConnection, user: &NewUser) -> Result<User> {
        let inserted_id: i32 = diesel::insert_into(users::table)
            .values(user)
            .returning(users::id)
            .get_result(conn)
            .await?;

        let user: User = users::table.filter(users::id.eq(inserted_id)).first(conn).await?;

        Ok(user)
    }
}
