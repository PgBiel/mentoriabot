use poise::serenity_prelude as serenity;

use crate::interaction::CustomId;

pub mod button;
pub use button::Button;

/// Represents a single SelectMenu.
pub trait SelectComponent<Data = ()> {
    fn on_build<'a>(
        &self,
        builder: &'a mut serenity::CreateSelectMenu,
        data: &Data,
    ) -> (&'a mut serenity::CreateSelectMenu, CustomId);

    /// Function to run when an interaction response is received.
    fn set_interaction(&mut self, interaction: serenity::MessageComponentInteraction);
}

/// Represents a single Button.
pub trait ButtonComponent<Data = ()> {
    fn on_build<'a>(
        &self,
        builder: &'a mut serenity::CreateButton,
        data: &Data,
    ) -> (&'a mut serenity::CreateButton, CustomId);

    /// Function to run when an interaction response is received.
    fn set_interaction(&mut self, interaction: serenity::MessageComponentInteraction);
}

pub trait ButtonsComponent<Data = ()> {
    /// Returns the generated custom IDs
    fn on_build<'a>(
        &self,
        builder: &'a mut serenity::CreateActionRow,
        data: &Data,
    ) -> (&'a mut serenity::CreateActionRow, Vec<CustomId>);

    /// Function to run when an interaction response is received.
    fn set_interaction(&mut self, interaction: serenity::MessageComponentInteraction);
}
