use poise::serenity_prelude as serenity;

use crate::{
    common::ApplicationContext,
    error::Result,
    interaction::{CustomId, SelectValue},
};

pub mod button;
pub mod reply;
pub mod selectmenu;
pub use button::ButtonSpec;
pub use reply::ReplySpec;
pub use selectmenu::SelectMenuSpec;

use self::selectmenu::SelectMenuOptionSpec;

/// Represents a single SelectMenu.
pub trait SelectComponent<Data = ()> {
    fn on_build<'a>(
        builder: &'a mut serenity::CreateSelectMenu,
        context: ApplicationContext<'_>,
        data: &Data,
    ) -> (&'a mut serenity::CreateSelectMenu, CustomId);

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
pub trait SelectOption<Data = ()>: From<SelectValue> {
    fn get_spec() -> SelectMenuOptionSpec<Data>;
}

/// Represents a single Button.
pub trait ButtonComponent<Data = ()> {
    fn on_build<'a>(
        builder: &'a mut serenity::CreateButton,
        context: ApplicationContext<'_>,
        data: &Data,
    ) -> (&'a mut serenity::CreateButton, Option<CustomId>);
    // use Option<CustomId> as link buttons do not have a Custom ID (and thus cannot be awaited)

    /// Function to run when an interaction response is received.
    /// Creates an instance of this component holding the received
    /// interaction response.
    fn create_with_interaction(
        interaction: serenity::MessageComponentInteraction,
    ) -> Result<Box<Self>>;
}

pub trait ButtonsComponent<Data = ()> {
    const BUTTON_SPECS: Vec<ButtonSpec<Data>>;

    /// Returns the generated custom IDs
    fn on_build<'a>(
        context: ApplicationContext<'_>,
        mut builder: &'a mut serenity::CreateActionRow,
        data: &Data,
    ) -> (&'a mut serenity::CreateActionRow, Vec<CustomId>) {
        let mut custom_ids = Vec::new();

        for button in Self::BUTTON_SPECS {
            builder = builder.create_button(|b| {
                let (b, custom_id) = button.on_build(b, context, data);

                if let Some(custom_id) = custom_id {
                    custom_ids.push(custom_id);
                }

                b
            });
        }

        (builder, custom_ids)
    }

    /// Function to run when an interaction response is received.
    /// Creates an instance of this component holding the received
    /// interaction response.
    fn create_with_interaction(
        interaction: serenity::MessageComponentInteraction,
    ) -> Result<Box<Self>>;
}
