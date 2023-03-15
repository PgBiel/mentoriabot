use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MiniRustBotConfig {
    token: String,
    guild_id: u64,
    database_url: String,
    admin_userids: Vec<u64>
}

impl MiniRustBotConfig {
    pub fn get_token(&self) -> &String {
        &self.token
    }

    pub fn get_guild_id(&self) -> u64 {
        self.guild_id
    }

    pub fn get_database_url(&self) -> &String {
        &self.database_url
    }

    pub fn get_admin_userids(&self) -> &Vec<u64> {
        &self.admin_userids
    }
}
