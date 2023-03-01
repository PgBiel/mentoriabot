use super::common::{Data, Error};

mod ping;
mod userup;
mod modal;

pub use ping::ping;
pub use userup::userup;
pub use modal::modal;

pub fn get_commands() -> Vec<poise::Command<Data, Error>> {
    vec![ping(), userup(), modal()]
}
