use crate::common::ApplicationContext;
use crate::error::{FormError, Result};
use crate::form::InteractionForm;
use crate::interaction;
use crate::util::generate_custom_id;
use async_trait::async_trait;
use poise::serenity_prelude as serenity;
use std::sync::Arc;

/// A Form component for either a Button or a Select Menu;
/// that is, one that can be displayed on a message.
#[async_trait]
pub trait MessageFormComponent: Sync {
    type Form: InteractionForm + Send + Sync;

    async fn send_component(
        &self,
        context: ApplicationContext<'_>,
        custom_id: &str,
        form_data: Self::Form,
    ) -> Result<Self::Form>;

    async fn on_response(
        &self,
        context: ApplicationContext<'_>,
        interaction: Arc<serenity::MessageComponentInteraction>,
        form_data: Self::Form,
    ) -> Result<Self::Form>;

    async fn run(
        &self,
        context: ApplicationContext<'_>,
        form_data: Self::Form,
    ) -> Result<Self::Form> {
        let custom_id = generate_custom_id();

        let form_data = self.send_component(context, &custom_id, form_data).await?;

        let response = interaction::wait_for_message_interaction(context, custom_id).await?;

        match response {
            Some(interaction) => {
                // strip original components (avoid repeated interactions)
                interaction
                    .edit_original_interaction_response(context.serenity_context, |f| {
                        f.components(|f| f)
                    })
                    .await?;

                self.on_response(context, interaction, form_data).await
            }
            None => Err(FormError::NoResponse.into()),
        }
    }
}
