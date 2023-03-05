use crate::common::ApplicationContext;
use crate::error::{FormError, Result};
use crate::interaction;
use crate::util::generate_custom_id;
use async_trait::async_trait;
use poise::serenity_prelude as serenity;
use std::sync::Arc;

/// A Form component for either a Button or a Select Menu;
/// that is, one that can be displayed on a message.
#[async_trait]
pub trait MessageFormComponent<Data: Send + Sync = ()>: Send + Sync {
    type Form: Send + Sync;

    async fn send_component(
        context: ApplicationContext<'_>,
        custom_id: &str,
        data: &mut Data,
    ) -> Result<()>;

    async fn on_response(
        context: ApplicationContext<'_>,
        interaction: Arc<serenity::MessageComponentInteraction>,
        data: &mut Data,
    ) -> Result<Box<Self>>;

    async fn run(context: ApplicationContext<'_>, data: &mut Data) -> Result<Box<Self>> {
        let custom_id = generate_custom_id();

        Self::send_component(context, &custom_id, data).await?;

        let response = interaction::wait_for_message_interaction(context, custom_id).await?;

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
