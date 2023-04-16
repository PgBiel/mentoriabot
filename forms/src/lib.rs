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
pub mod util;

#[cfg(test)]
mod tests;

pub use component::{
    Buildable, BuildableWithId, ButtonSpec, GenerateReply, MessageFormComponent,
    ModalFormComponent, ReplySpec, SelectMenuOptionSpec, SelectMenuSpec, SelectOption,
    Subcomponent,
};
pub use error::{CtxError, CtxResult, FormError, Result};
pub use form::InteractionForm;
pub use interaction::{CustomId, HasCustomId, SelectValue};

pub mod macros {
    pub use minirustbot_macros::*;
}

pub use macros::{ButtonComponent, GenerateReply, InteractionForm, SelectOption};
