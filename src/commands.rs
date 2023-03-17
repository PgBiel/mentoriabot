use crate::{common::Data, error::Error};

mod modal;
mod ping;
mod testform;
mod userman;
mod userup;

pub use modal::modal;
pub use ping::ping;
pub use testform::testform;
pub use userman::userman;
pub use userup::userup;

pub fn get_commands() -> Vec<poise::Command<Data, Error>> {
    vec![ping(), userup(), modal(), testform(), userman()]
}
