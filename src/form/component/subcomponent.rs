use async_trait::async_trait;
use poise::serenity_prelude as serenity;

use super::{GenerateReply, MessageFormComponent};
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

// --- impls ---

#[async_trait]
impl<B, D> MessageFormComponent<D> for B
where
    D: Send + Sync,
    B: ButtonComponent<D> + GenerateReply,
{
    async fn send_component_and_wait(
        context: ApplicationContext<'_>,
        data: &mut Data,
    ) -> Result<Option<Arc<serenity::MessageComponentInteraction>>> {
        context
            .send(|f| {
                Self::create_reply(&mut f)
                    .components(|f| f.create_action_row(|f| f.create_button(|f| Self::create(f))))
            })
            .await?;
        // TODO...
    }

    async fn on_response(
        context: ApplicationContext<'_>,
        interaction: Arc<serenity::MessageComponentInteraction>,
        data: &mut Data,
    ) -> Result<Box<Self>>;
}
