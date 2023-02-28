use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MiniRustBotConfig {
    token: String,
    guild_id: u64
}

impl MiniRustBotConfig {
    pub fn get_token(&self) -> &String {
        &self.token
    }

    pub fn get_guild_id(&self) -> u64 {
        self.guild_id
    }
}
