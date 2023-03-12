//! A module for helpers for specific types of Form components.

use async_trait::async_trait;
use poise::{serenity_prelude as serenity, ApplicationContext};

use crate::{
    error::Result,
    interaction::SelectValue,
};

pub mod button;
pub mod reply;
pub mod selectmenu;

pub use button::ButtonSpec;
pub use reply::ReplySpec;
pub use selectmenu::{SelectMenuOptionSpec, SelectMenuSpec};

use super::BuildableWithId;

/// Represents a single SelectMenu.
#[async_trait]
pub trait SelectComponent<ContextData, ContextError, Data = ()> {
    /// The Select Menu buildable returned by `generate_menu`.
    type SelectBuilder: BuildableWithId<serenity::CreateSelectMenu>;

    /// Generates this menu's Spec, allowing it to be built
    /// and sent to Discord.
    async fn generate_menu(
        context: ApplicationContext<'_, ContextData, ContextError>,
        data: &Data,
    ) -> Result<Self::SelectBuilder>;

    /// Function to run when an interaction response is received.
    /// Creates an instance of this component holding the received
    /// interaction response.
    fn create_with_interaction(
        interaction: serenity::MessageComponentInteraction,
    ) -> Result<Box<Self>>;
}

/// Represents a Select Option, that is,
/// an object (usually enum) that represents
/// a user's specific choice in a selection menu.
pub trait SelectOption: TryFrom<SelectValue> {
    /// Returns the specs of all possible options.
    fn get_specs() -> Vec<SelectMenuOptionSpec>;
}

/// Represents a single Button.
#[async_trait]
pub trait ButtonComponent<ContextData, ContextError, Data = ()> {
    /// The Select Menu buildable returned by `generate_button`.
    type ButtonBuilder: BuildableWithId<serenity::CreateButton>;

    /// Generates this button's Spec, allowing it to be built
    /// and sent to Discord. You'll generally want to use
    /// [`ButtonSpec`], but you're free to implement your own
    /// [`BuildableWithId`]-implementing types for this.
    async fn generate_button(
        context: ApplicationContext<'_, ContextData, ContextError>,
        data: &Data,
    ) -> Result<Self::ButtonBuilder>;

    /// Function to run when an interaction response is received.
    /// Creates an instance of this component holding the received
    /// interaction response.
    fn create_with_interaction(
        interaction: serenity::MessageComponentInteraction,
    ) -> Result<Box<Self>>;
}

/// Represents a row of buttons in a component.
#[async_trait]
pub trait ButtonsComponent<ContextData, ContextError, Data = ()> {
    /// The Select Menu buildable returned by `generate_buttons`.
    type ButtonBuilder: BuildableWithId<serenity::CreateButton>;

    /// Generates the specs of this Button row. Usually you'll wish
    /// to use [`ButtonSpec`], but you have to freedom to return your own
    /// types for this.
    async fn generate_buttons<'a>(
        context: ApplicationContext<'_, ContextData, ContextError>,
        data: &Data,
    ) -> Result<Vec<Self::ButtonBuilder>>;

    /// Function to run when an interaction response is received.
    /// Creates an instance of this component holding the received
    /// interaction response.
    fn create_with_interaction(
        interaction: serenity::MessageComponentInteraction,
    ) -> Result<Box<Self>>;
}
