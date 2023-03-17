use std::sync::Arc;

use diesel_async::AsyncConnection;
use tokio::sync as tokio;
use crate::error::Result;

/// Holds a database Connection object, with a [`tokio::sync::Mutex`]
/// to allow asynchronous locking of access to it.
///
/// [`tokio::sync::Mutex`]: tokio::Mutex
pub struct ConnectionManager {
    connection: Arc<tokio::Mutex<diesel_async::AsyncPgConnection>>,
}

/// General function for creating a connection to the database.
pub async fn create_connection(database_url: &str) -> Result<diesel_async::AsyncPgConnection> {
    diesel_async::AsyncPgConnection::establish(database_url)
        .await
        .map_err(From::from)
}

impl ConnectionManager {
    /// Creates a Connection Manager by initializing a connection
    /// to the given Database URL, if possible
    pub async fn create(database_url: &str) -> Result<Self> {
        let connection = create_connection(database_url).await?;
        Ok(ConnectionManager {
            connection: Arc::new(tokio::Mutex::new(connection)),
        })
    }

    /// Gets the connection held by this ConnectionManager, wrapped in a Mutex
    /// to allow for concurrent access.
    pub fn get_connection(&self) -> Arc<tokio::Mutex<diesel_async::AsyncPgConnection>> {
        Arc::clone(&self.connection)
    }

    // pub async fn run_with_connection<T, F, Fut>(
    //     &self,
    //     f: F
    // ) -> Result<T>
    // where
    //     F: FnOnce(Arc<tokio::Mutex<diesel_async::AsyncPgConnection>>) -> Fut,
    //     Fut: Future<Output = T>
    // {
    //     // let mut conn = self.connection.try_lock().map_err(|_| Error::Other("Failed to lock
    // connection"))?;     Ok(f(Arc::clone(&self.connection)).await)
    // }

    // pub async fn run_with_connection_boxed<T, F>(
    //     &self,
    //     f: F
    // ) -> Result<T>
    // where
    //     F: FnOnce(&mut diesel_async::AsyncPgConnection) -> BoxFuture<T>
    // {
    //     let mut conn = self.connection.try_lock().map_err(|_| Error::Other("Failed to lock
    // connection"))?;     Ok(f(&mut conn).await)
    // }
}
