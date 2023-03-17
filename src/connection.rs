use std::sync::Arc;

use diesel_async::AsyncConnection;
use tokio::sync as tokio;

pub struct ConnectionManager {
    connection: Arc<tokio::Mutex<diesel_async::AsyncPgConnection>>,
}

pub async fn create_connection(database_url: &str) -> diesel_async::AsyncPgConnection {
    diesel_async::AsyncPgConnection::establish(database_url)
        .await
        .expect("Failed to connect to database.")
}

impl ConnectionManager {
    pub async fn create(database_url: &str) -> Self {
        ConnectionManager {
            connection: Arc::new(tokio::Mutex::new(create_connection(database_url).await)),
        }
    }

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
