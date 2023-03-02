use super::common::ApplicationContext;
use std::sync::Arc;
use std::time::Duration;
use poise::serenity_prelude as serenity;

pub mod form;

/// Wait for a given interaction's response.
pub async fn wait_for_message_interaction(ctx: ApplicationContext<'_>, custom_id: impl ToString)
        -> Result<Option<Arc<serenity::MessageComponentInteraction>>, serenity::Error> {
    let custom_id = custom_id.to_string();
    let response = serenity::CollectComponentInteraction::new(&ctx.serenity_context.shard)
            .filter(move |interaction| interaction.data.custom_id.to_string() == custom_id)
            .timeout(Duration::from_secs(15*60))
            .await;

    let response = match response {
        Some(value) => value,
        None => return Ok(None),
    };

    // send acknowledgement
    response
        .create_interaction_response(ctx.serenity_context, |f| {
            f.kind(serenity::InteractionResponseType::DeferredUpdateMessage)
        })
        .await?;

    Ok(Some(response))
}
