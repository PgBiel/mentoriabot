use crate::{connection::ConnectionManager, error::Error};

/// Global command data, which is stored and accessible in all command invocations
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

/// The bot's custom general Command Context type
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// The bot's custom Application Context type
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;
