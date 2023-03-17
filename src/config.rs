use serde::{Deserialize, Serialize};

/// The bot's parsed config file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct MiniRustBotConfig {
    /// This bot's token.
    pub(crate) token: String,

    /// This bot's Guild ID.
    pub(crate) guild_id: u64,

    /// This bot's database URL.
    pub(crate) database_url: String,

    /// The Discord User IDs of this bot's administrators.
    pub(crate) admin_userids: Vec<u64>,
}
