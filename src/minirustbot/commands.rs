use super::common::{Data, ErrorBox};

mod ping;
mod userup;
mod modal;

pub use ping::ping;
pub use userup::userup;
pub use modal::modal;

pub fn get_commands() -> Vec<poise::Command<Data, ErrorBox>> {
    vec![ping(), userup(), modal()]
}
