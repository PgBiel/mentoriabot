//! Module for easy creation of Discord forms.
//! Forms are simply a sequence of Discord interactions asking for user input,
//! may it be a Modal, may it be a Message Component (Button, Select Menu).
//!
//! Forms are structs with the trait [`InteractionForm`]. This trait
//! is derivable with a macro - see its documentation for more info.
//!
//! Use the [`MessageFormComponent`] trait to derive components for your form.
//! Modals have a blanket implementation for [`ModalFormComponent`], which can be overriden.
//!
//! There are several helper traits and derive-macros for specific component types, but you may wish
//! to ignore those depending on the complexity of your needs.
//!
//! [`InteractionForm`]: InteractionForm
//! [`MessageFormComponent`]: component::MessageFormComponent
//! [`MessageFormComponent`]: component::ModalFormComponent

pub mod component;
pub mod error;
mod form;
pub mod interaction;
mod state;
pub mod util;

pub use component::{
    Buildable, BuildableWithId, ButtonSpec, GenerateReply, MessageFormComponent,
    ModalFormComponent, ReplySpec, SelectMenuOptionSpec, SelectMenuSpec, SelectOption,
    Subcomponent,
};
pub use error::{ContextualError, ContextualResult, FormError, Result};
pub use form::InteractionForm;
pub use interaction::{CustomId, HasCustomId, SelectValue};
pub use state::FormState;

pub mod macros {
    pub use mentoriabot_macros::*;
}

pub use macros::{
    ButtonComponent, GenerateReply, InteractionForm, MessageFormComponent, SelectOption,
};
