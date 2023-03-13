//! A module for helpers for specific types of Form components.

use async_trait::async_trait;
use poise::{serenity_prelude as serenity, ApplicationContext};

use crate::{
    error::{FormError, Result},
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
pub trait Subcomponent<ContextData, ContextError, FormData = ()>: TryFrom<serenity::MessageComponentInteraction, Error = FormError> {
    type BuilderType: Send + Sync;
    type ReturnedBuildable: BuildableWithId<Self::BuilderType> + Send + Sync;

    async fn generate_buildables(
        context: ApplicationContext<'_, ContextData, ContextError>,
        data: &FormData,
    ) -> Result<Vec<Self::ReturnedBuildable>>;
}


/// Represents a Select Option, that is,
/// an object (usually enum) that represents
/// a user's specific choice in a selection menu.
pub trait SelectOption: TryFrom<SelectValue> {
    /// Returns the specs of all possible options.
    fn get_specs() -> Vec<SelectMenuOptionSpec>;
}
