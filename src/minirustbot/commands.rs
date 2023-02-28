use super::common::{Data, Error};

mod ping;
mod userup;

pub use ping::ping;
pub use userup::userup;

pub fn get_commands() -> Vec<poise::Command<Data, Error>> {
    vec![ping(), userup()]
}
