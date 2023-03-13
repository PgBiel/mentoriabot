use diesel::Connection;

pub fn create_connection(database_url: &String) -> diesel::PgConnection {
    diesel::PgConnection::establish(database_url)
        .expect("Failed to connect to database.")
}
