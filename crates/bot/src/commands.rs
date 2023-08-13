use crate::{common::Data, lib::error::Error};

mod forms;
mod loadmentors;
mod modal;
mod modals;
mod ping;
mod schedule;
mod sessionman;
mod testform;
mod userman;
mod userup;

pub use loadmentors::loadmentors;
pub use modal::modal;
pub use ping::ping;
pub use schedule::schedule;
pub use sessionman::sessionman;
pub use testform::testform;
pub use userman::userman;
pub use userup::userup;

pub fn get_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        loadmentors(),
        ping(),
        userup(),
        modal(),
        testform(),
        userman(),
        sessionman(),
        schedule(),
    ]
}
