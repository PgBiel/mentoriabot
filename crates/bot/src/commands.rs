use crate::{common::Data, lib::error::Error};

mod autocomplete;
mod embeds;
mod forms;
mod loadmentors;
mod modal;
mod modals;
mod ping;
mod register;
mod restart;
mod schedule;
mod sessionman;
mod sessions;
mod testform;
mod unschedule;
mod userman;
mod userup;

pub use loadmentors::loadmentors;
pub use modal::modal;
pub use ping::ping;
pub use register::register;
pub use restart::restart;
pub use schedule::schedule;
pub use sessionman::sessionman;
pub use sessions::sessions;
pub use testform::testform;
pub use unschedule::unschedule;
pub use userman::userman;
pub use userup::userup;

pub fn get_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        loadmentors(),
        ping(),
        register(),
        restart(),
        userup(),
        modal(),
        testform(),
        unschedule(),
        userman(),
        sessions(),
        sessionman(),
        schedule(),
    ]
}
