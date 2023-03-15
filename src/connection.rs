use diesel_async::AsyncConnection;

pub async fn create_connection(database_url: &str) -> diesel_async::AsyncPgConnection {
    diesel_async::AsyncPgConnection::establish(database_url)
        .await
        .expect("Failed to connect to database.")
}
