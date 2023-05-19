use crate::{common::Data, lib::error::Error};

mod forms;
mod modal;
mod modals;
mod ping;
mod schedule;
mod sessions;
mod testform;
mod userman;
mod userup;

pub use modal::modal;
pub use ping::ping;
pub use schedule::schedule;
pub use sessions::sessions;
pub use testform::testform;
pub use userman::userman;
pub use userup::userup;

pub fn get_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        ping(),
        userup(),
        modal(),
        testform(),
        userman(),
        sessions(),
        schedule(),
    ]
}
