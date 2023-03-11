use std::sync::Arc;

use async_trait::async_trait;
use poise::{serenity_prelude as serenity, ApplicationContext};

use super::GenerateReply;
use crate::{
    error::{FormError, Result},
    interaction::{self, CustomId},
};

/// A Form component for either a Button or a Select Menu;
/// that is, one that can be displayed on a message.
#[async_trait]
pub trait MessageFormComponent<ContextData, ContextError, Data = ()>:
    GenerateReply<ContextData, ContextError, Data> + Send + Sync
where
    ContextData: Send + Sync,
    ContextError: Send + Sync,
    Data: Send + Sync,
{
    /// Method to send the component's interaction to Discord.
    async fn send_component(
        context: ApplicationContext<'_, ContextData, ContextError>,
        data: &mut Data,
    ) -> Result<Vec<CustomId>>;

    /// Method that waits for the user's response to the interaction.
    async fn wait_for_response(
        context: ApplicationContext<'_, ContextData, ContextError>,
        data: &mut Data,
        custom_ids: Vec<CustomId>,
    ) -> Result<Option<Arc<serenity::MessageComponentInteraction>>> {
        interaction::wait_for_message_interactions(context, custom_ids)
            .await
            .map_err(FormError::Serenity)
    }

    /// Method to react to the user's response to the interaction.
    async fn on_response(
        context: ApplicationContext<'_, ContextData, ContextError>,
        interaction: Arc<serenity::MessageComponentInteraction>,
        data: &mut Data,
    ) -> Result<Box<Self>>;

    async fn run(context: ApplicationContext<'_, ContextData, ContextError>, data: &mut Data) -> Result<Box<Self>> {
        let ids = Self::send_component(context, data).await?;
        let response = Self::wait_for_response(context, data, ids).await?;

        match response {
            Some(interaction) => {
                // strip original components (avoid repeated interactions)
                interaction
                    .edit_original_interaction_response(context.serenity_context, |f| {
                        f.components(|f| f)
                    })
                    .await?;

                Self::on_response(context, interaction, data).await
            }
            None => Err(FormError::NoResponse.into()),
        }
    }
}
