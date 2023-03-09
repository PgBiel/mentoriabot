mod custom_id;
mod selectvalue;
use std::{sync::Arc, time::Duration};

pub use custom_id::CustomId;
use poise::serenity_prelude as serenity;
pub use selectvalue::SelectValue;

use crate::common::ApplicationContext;

/// Wait for a given interaction's response.
pub async fn wait_for_message_interaction(
    ctx: ApplicationContext<'_>,
    custom_id: impl ToString,
) -> Result<Option<Arc<serenity::MessageComponentInteraction>>, serenity::Error> {
    wait_for_message_interactions(ctx, vec![custom_id]).await
}

/// Wait for the first response in a set of expected interaction responses.
pub async fn wait_for_message_interactions(
    ctx: ApplicationContext<'_>,
    custom_ids: Vec<impl ToString>,
) -> Result<Option<Arc<serenity::MessageComponentInteraction>>, serenity::Error> {
    let custom_ids = custom_ids
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    let response = serenity::CollectComponentInteraction::new(&ctx.serenity_context.shard)
        .filter(move |interaction| custom_ids.contains(&interaction.data.custom_id.to_string()))
        .timeout(Duration::from_secs(15 * 60))
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
