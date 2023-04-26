use std::sync::Arc;

use async_trait::async_trait;
use diesel::{OptionalExtension, QueryDsl};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use super::{
    repo_find_all, repo_get, repo_insert, repo_remove, repo_update, repo_upsert, Repository,
    UpdatableRepository,
};
use crate::{
    error::{Error, Result},
    model::{DiscordId, NewUser, PartialUser, Session, Teacher, User},
    schema::users,
};

/// Manages User instances.
#[derive(Clone)]
pub struct UserRepository {
    pool: Arc<Pool<AsyncPgConnection>>,
}

impl UserRepository {
    /// Creates a new UserRepository operating with the given
    /// connection pool.
    pub fn new(pool: &Arc<Pool<AsyncPgConnection>>) -> Self {
        Self {
            pool: Arc::clone(pool),
        }
    }

    /// Gets a User from the database by their Discord ID,
    /// or inserts them instead.
    pub async fn get_or_insert(&self, user: &NewUser) -> Result<User> {
        if let Some(found_user) = self.get(user.discord_id).await? {
            Ok(found_user)
        } else {
            self.insert(user).await
        }
    }

    /// Attempts to insert a User; does nothing if such a User is already registered.
    /// Returns the inserted row count (1 if a new User was inserted or 0 otherwise).
    pub async fn insert_if_not_exists(&self, user: &NewUser) -> Result<usize> {
        diesel::insert_into(users::table)
            .values(user)
            .on_conflict_do_nothing()
            .execute(&mut self.lock_connection().await?)
            .await
            .map_err(From::from)
    }

    /// Searches for a Teacher's User instance.
    pub async fn find_by_teacher(&self, teacher: &Teacher) -> Result<User> {
        self.get(teacher.user_id)
            .await?
            .ok_or_else(|| Error::Other("Could not find User for a certain teacher!"))
    }

    /// Gets a Session's student User.
    pub async fn find_student_of_session(&self, session: &Session) -> Result<User> {
        self.get(session.student_id)
            .await?
            .ok_or_else(|| Error::Other("Could not find User that is student of a session!"))
    }
}

#[async_trait]
impl Repository for UserRepository {
    type Table = users::table;

    type Entity = User;

    type NewEntity = NewUser;

    type PrimaryKey = DiscordId;

    const TABLE: Self::Table = users::table;

    fn get_connection_pool(&self) -> Arc<Pool<AsyncPgConnection>> {
        Arc::clone(&self.pool)
    }

    /// Gets a User by their Discord ID.
    async fn get(&self, discord_id: DiscordId) -> Result<Option<User>> {
        repo_get!(self, users::table; discord_id)
    }

    async fn insert(&self, user: &NewUser) -> Result<User> {
        repo_insert!(self, users::table; user)
    }

    async fn remove(&self, user: &User) -> Result<usize> {
        repo_remove!(self; user)
    }

    async fn find_all(&self) -> Result<Vec<User>> {
        repo_find_all!(self, users::table, users::table)
    }
}

#[async_trait]
impl UpdatableRepository for UserRepository {
    type PartialEntity = PartialUser;

    async fn upsert(&self, user: &NewUser) -> Result<User> {
        repo_upsert!(self, users::table; /*conflict_columns=*/users::discord_id; user)
    }

    async fn update(&self, old_user: &User, new_user: PartialUser) -> Result<User> {
        repo_update!(self; old_user => new_user)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::init_db;
    use crate::{
        model::{DiscordId, NewUser},
        repository::{Repository, UpdatableRepository},
    };

    #[tokio::test]
    async fn test_user_get_insert_find_remove() {
        let db = init_db();
        let repo = db.user_repository();

        let id = DiscordId(1);
        let new_user = NewUser {
            discord_id: id,
            name: "Joseph".to_string(),
            bio: Some("I am myself".to_string()),
        };

        assert_eq!(None, repo.get(id).await.unwrap());
        assert_eq!(new_user, repo.insert(&new_user).await.unwrap());
        assert_eq!(Some(&new_user), repo.get(id).await.unwrap().as_ref());
        assert_eq!(
            vec![&new_user],
            repo.find_all().await.unwrap().iter().collect::<Vec<_>>()
        );
        assert_eq!(1, repo.remove(&new_user).await.unwrap());
        assert_eq!(None, repo.get(id).await.unwrap().as_ref());
    }

    #[tokio::test]
    async fn test_user_upsert_update() {
        let db = init_db();
        let repo = db.user_repository();

        let id = DiscordId(2);
        let new_user = NewUser {
            discord_id: id,
            name: "Joseph".to_string(),
            bio: Some("I am myself".to_string()),
        };
        let other_user = NewUser {
            bio: None,
            ..new_user.clone()
        };
        let third_user = NewUser {
            name: "Andrew".to_string(),
            ..other_user.clone()
        };

        assert_eq!(new_user, repo.upsert(&new_user).await.unwrap());
        assert_eq!(other_user, repo.upsert(&other_user).await.unwrap());
        assert_eq!(
            Some(&other_user),
            repo.get(new_user.discord_id).await.unwrap().as_ref()
        );
        assert_eq!(
            third_user,
            repo.update(&other_user, third_user.clone().into())
                .await
                .unwrap()
        );
        assert_eq!(
            Some(&third_user),
            repo.get(new_user.discord_id).await.unwrap().as_ref()
        );
    }
}
