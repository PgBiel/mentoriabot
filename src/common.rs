use crate::{
    forms,
    lib::{db::DatabaseManager, error::Error, notification::GoogleApiManager},
};

/// Global command data, which is stored and accessible in all command invocations
#[derive(Clone)]
pub struct Data {
    pub db: DatabaseManager,
    pub admin_userids: Vec<u64>,
    pub google: GoogleApiManager,
}

impl Data {
    pub fn new(db: DatabaseManager, admin_userids: Vec<u64>, google: GoogleApiManager) -> Self {
        Self {
            db,
            admin_userids,
            google,
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

/// Results in Forms use our Error.
pub type ContextualResult<T> = forms::ContextualResult<T, Error>;
