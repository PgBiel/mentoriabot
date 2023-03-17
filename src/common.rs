use crate::{connection::ConnectionManager, error::Error};

// User data, which is stored and accessible in all command invocations
pub struct Data {
    pub connection: ConnectionManager,
    pub admin_userids: Vec<u64>,
}

impl Data {
    pub fn new(connection: ConnectionManager, admin_userids: Vec<u64>) -> Self {
        Data {
            connection,
            admin_userids,
        }
    }
}

pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;
