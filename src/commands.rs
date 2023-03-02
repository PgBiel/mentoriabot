use crate::common::{Data, ErrorBox};

mod ping;
mod userup;
mod modal;
mod testform;

pub use ping::ping;
pub use userup::userup;
pub use modal::modal;
pub use testform::testform;

pub fn get_commands() -> Vec<poise::Command<Data, ErrorBox>> {
    vec![ping(), userup(), modal(), testform()]
}
