use crate::{connection::DatabaseManager, error::Error};

/// Global command data, which is stored and accessible in all command invocations
pub struct Data {
    pub db: DatabaseManager,
    pub admin_userids: Vec<u64>,
}

impl Data {
    pub fn new(db: DatabaseManager, admin_userids: Vec<u64>) -> Self {
        Self {
            db,
            admin_userids,
        }
    }

    pub fn user_is_admin(&self, user_id: u64) -> bool {
        self.admin_userids.contains(&user_id)
    }
}

/// The bot's custom general Command Context type
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// Any error which occurred while using poise
pub type FrameworkError<'a> = poise::FrameworkError<'a, Data, Error>;

/// The bot's custom Application Context type
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;
