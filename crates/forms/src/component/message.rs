use std::sync::Arc;

use async_trait::async_trait;
use poise::{serenity_prelude as serenity, ApplicationContext};

use crate::{
    error::{ContextualResult, FormError},
    interaction::{self, CustomId},
    FormState,
};

/// A Form component for either a Button or a Select Menu;
/// that is, one that can be displayed on a message.
#[async_trait]
pub trait MessageFormComponent<ContextData, ContextError, FormData = ()>: Send + Sync
where
    ContextData: Send + Sync,
    ContextError: Send + Sync,
    FormData: Send + Sync,
{
    /// Method to send the component's interaction(s) to Discord.
    /// This should return all Custom IDs for which a User response is expected.
    async fn send_component(
        context: ApplicationContext<'_, ContextData, ContextError>,
        data: &mut FormState<FormData>,
    ) -> ContextualResult<Vec<CustomId>, ContextError>;

    /// Method that waits for the user's response to the interaction.
    async fn wait_for_response(
        context: ApplicationContext<'_, ContextData, ContextError>,
        _data: &mut FormState<FormData>,
        custom_ids: &Vec<CustomId>,
    ) -> ContextualResult<Option<Arc<serenity::MessageComponentInteraction>>, ContextError> {
        interaction::wait_for_message_interactions(context, custom_ids)
            .await
            .map_err(From::from)
    }

    /// Method to react to the user's response to the interaction.
    ///
    /// Return None to not proceed with the form, and keep waiting instead.
    async fn on_response(
        context: ApplicationContext<'_, ContextData, ContextError>,
        interaction: Arc<serenity::MessageComponentInteraction>,
        data: &mut FormState<FormData>,
    ) -> ContextualResult<Option<Box<Self>>, ContextError>;

    /// The main method, causes the component to be sent to Discord
    /// and its response awaited by invoking the other methods.
    async fn run(
        context: ApplicationContext<'_, ContextData, ContextError>,
        data: &mut FormState<FormData>,
    ) -> ContextualResult<Box<Self>, ContextError> {
        let ids = Self::send_component(context, data).await?;

        // keep waiting for a response until 'on_response' returns something
        loop {
            let response = Self::wait_for_response(context, data, &ids).await?;

            match response {
                Some(interaction) => {
                    // strip original components (avoid repeated interactions)
                    let ret = Self::on_response(context, Arc::clone(&interaction), data).await?;

                    if let Some(new_self) = ret {
                        interaction
                            .edit_original_interaction_response(context.serenity_context, |f| {
                                f.components(|f| f)
                            })
                            .await?;

                        return Ok(new_self);
                    }
                    // otherwise keep waiting for a response
                }
                None => return Err(FormError::NoResponse.into()),
            }
        }
    }
}
