use serde::{Deserialize, Serialize};

/// The bot's parsed config file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MiniRustBotConfig {
    token: String,
    guild_id: u64,
    database_url: String,
    admin_userids: Vec<u64>,
}

impl MiniRustBotConfig {
    /// Gets this bot's token.
    pub fn get_token(&self) -> &String {
        &self.token
    }

    /// Gets this bot's main Guild ID.
    pub fn get_guild_id(&self) -> u64 {
        self.guild_id
    }

    /// Gets this bot's database URL.
    pub fn get_database_url(&self) -> &String {
        &self.database_url
    }

    /// Gets the Discord User IDs of the administrators of this bot.
    pub fn get_admin_userids(&self) -> &Vec<u64> {
        &self.admin_userids
    }
}
