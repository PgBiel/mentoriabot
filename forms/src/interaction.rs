mod custom_id;
mod selectvalue;
use std::{sync::Arc, time::Duration};

pub use custom_id::{CustomId, HasCustomId};
use poise::{serenity_prelude as serenity, ApplicationContext};
pub use selectvalue::SelectValue;

/// Wait for a given interaction's response.
/// This works by waiting until the given Custom ID is received as an interaction,
/// or until 15 minutes have passed (timeout).
///
/// This may return `None` if no response was received (e.g. due to timeout),
/// or `Some(Arc<MessageComponentInteraction>)` if an interaction was received with the response.
pub async fn wait_for_message_interaction<ContextData, ContextError>(
    ctx: ApplicationContext<'_, ContextData, ContextError>,
    custom_id: impl ToString,
) -> Result<Option<Arc<serenity::MessageComponentInteraction>>, serenity::Error> {
    wait_for_message_interactions(ctx, vec![custom_id]).await
}

/// Wait for the first response in a set of expected interaction responses.
/// With the given expected Custom IDs, the first interaction response with a
/// matching Custom ID will be returned. Otherwise, waits until 15 minutes
/// (interaction token expiration time) have passed.
///
/// This may return `None` if no response was received (e.g. due to timeout, or
/// if no custom IDs were given), or return `Some(Arc<MessageComponentInteraction>)`
/// if an interaction was received with the response.
pub async fn wait_for_message_interactions<ContextData, ContextError>(
    ctx: ApplicationContext<'_, ContextData, ContextError>,
    custom_ids: Vec<impl ToString>,
) -> Result<Option<Arc<serenity::MessageComponentInteraction>>, serenity::Error> {
    if custom_ids.is_empty() {
        return Ok(None);
    }

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
