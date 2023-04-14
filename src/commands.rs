use crate::{common::Data, error::Error};

mod modal;
mod modals;
mod ping;
mod sessions;
mod testform;
mod userman;
mod userup;
mod schedule;

pub use modal::modal;
pub use ping::ping;
pub use sessions::sessions;
pub use testform::testform;
pub use userman::userman;
pub use userup::userup;

pub fn get_commands() -> Vec<poise::Command<Data, Error>> {
    vec![ping(), userup(), modal(), testform(), userman(), sessions()]
}
