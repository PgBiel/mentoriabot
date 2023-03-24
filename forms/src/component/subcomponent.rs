//! A module for helpers for specific types of Form components.

use std::sync::Arc;

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


/// Subcomponents hold common logic for predefined interaction types,
/// such as buttons and select menus.
///
/// They must be able to be generated from a received [`MessageComponentInteraction`] response,
/// and must generate the Buildables required to build the interactions to be sent.
///
/// [`MessageComponentInteraction`]: serenity::MessageComponentInteraction
#[async_trait]
pub trait Subcomponent<ContextData, ContextError, FormData = ()> {
    type BuilderType: Send + Sync;
    type ReturnedBuildable: BuildableWithId<Self::BuilderType> + Send + Sync;

    /// Generates the necessary buildables for this subcomponent.
    /// E.g., for a button, this will return a vector with just one element,
    /// which is a buildable for the button (such as a [`ButtonSpec`]).
    /// For a row of buttons, this will (possibly) return a vector of [`ButtonSpec`].
    ///
    /// They are buildables with IDs because the IDs are necessary to await
    /// each of those components (each button in a row of buttons, for example),
    /// and to figure out which one was clicked or otherwise interacted with.
    async fn generate_buildables(
        context: ApplicationContext<'_, ContextData, ContextError>,
        data: &mut FormData,
    ) -> Result<Vec<Self::ReturnedBuildable>>;

    /// Constructs this subcomponent from the received interaction, by e.g.
    /// comparing the custom IDs or the received select option value.
    async fn build_from_interaction(
        context: ApplicationContext<'_, ContextData, ContextError>,
        interaction: Arc<serenity::MessageComponentInteraction>,
        data: &mut FormData,
    ) -> Result<Box<Self>>;
}

/// Represents a Select Option, that is,
/// an object (usually enum) that represents
/// a user's specific choice in a selection menu.
#[async_trait]
pub trait SelectOption<ContextData, ContextError, FormData = ()> {
    /// Generates the specs of all possible options, based on the context
    /// and data.
    async fn generate_options(
        context: ApplicationContext<'_, ContextData, ContextError>,
        data: &mut FormData,
    ) -> Result<Vec<SelectMenuOptionSpec>>;

    /// Builds Self based on the selected value, using context and data as necessary.
    async fn build_from_selected_value(
        value: SelectValue,
        context: ApplicationContext<'_, ContextData, ContextError>,
        data: &mut FormData,
    ) -> Result<Box<Self>>;
}
