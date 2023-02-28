use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MiniRustBotConfig {
    token: String
}

impl MiniRustBotConfig {
    pub fn get_token(&self) -> &String {
        &self.token
    }
}
