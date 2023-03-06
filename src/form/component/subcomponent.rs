use poise::serenity_prelude as serenity;

use crate::interaction::CustomId;

/// Represents a single SelectMenu.
pub trait SelectComponent<Data> {
    fn create<'a>(
        builder: &'a mut serenity::CreateSelectMenu,
        data: &Data,
    ) -> (&'a mut serenity::CreateSelectMenu, CustomId);

    /// Function to run when an interaction response is received.
    fn create_with_interaction(interaction: serenity::MessageComponentInteraction) -> Self;
}

/// Represents a single Button.
pub trait ButtonComponent<Data> {
    fn create<'a>(
        builder: &'a mut serenity::CreateButton,
        data: &Data,
    ) -> (&'a mut serenity::CreateButton, CustomId);

    /// Function to run when an interaction response is received.
    fn create_with_interaction(interaction: serenity::MessageComponentInteraction) -> Self;
}

pub trait ButtonsComponent<Data> {
    /// Returns the generated custom IDs
    fn create<'a>(
        builder: &'a mut serenity::CreateActionRow,
        data: &Data,
    ) -> (&'a mut serenity::CreateActionRow, Vec<CustomId>);

    /// Function to run when an interaction response is received.
    fn create_with_interaction(interaction: serenity::MessageComponentInteraction) -> Self;
}
