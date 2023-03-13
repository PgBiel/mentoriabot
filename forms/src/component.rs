//! A module for Form components.

mod buildable;
mod generatereply;
mod message;
mod modal;
mod subcomponent;

pub use buildable::{Buildable, BuildableWithId};
pub use generatereply::GenerateReply;
pub use message::MessageFormComponent;
pub use modal::ModalFormComponent;
pub use subcomponent::{
    ButtonSpec, ReplySpec, Subcomponent,
    SelectMenuOptionSpec, SelectMenuSpec, SelectOption,
};
