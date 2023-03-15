use crate::{common::Data, error::Error};

mod modal;
mod ping;
mod testform;
mod userup;
mod userman;

pub use modal::modal;
pub use ping::ping;
pub use testform::testform;
pub use userup::userup;
pub use userman::userman;

pub fn get_commands() -> Vec<poise::Command<Data, Error>> {
    vec![ping(), userup(), modal(), testform(), userman()]
}
