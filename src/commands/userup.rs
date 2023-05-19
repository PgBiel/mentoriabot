use poise::serenity_prelude as serenity;

use crate::{common::ApplicationContext, forms::interaction, lib::error::Error};

/// Displays your or another user's account creation date
#[poise::command(slash_command, owners_only)]
pub async fn userup(
    ctx: ApplicationContext<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    let id = ctx.interaction.id();

    //    let rx = ctx.data.interaction_handler()
    //        .wait_for_interaction(id.to_string(),
    // serenity::InteractionType::MessageComponent).await;

    ctx.send(|create_reply| {
        create_reply
            .content(response)
            .ephemeral(true)
            .components(|create_components| {
                create_components.create_action_row(|create_row| {
                    create_row.create_button(|create_button| {
                        create_button
                            .custom_id(id)
                            .label("OK")
                            .style(serenity::ButtonStyle::Primary)
                    })
                })
            })
    })
    .await?;

    let response = interaction::wait_for_message_interaction(ctx, id.to_string()).await?;
    if let Some(response) = response {
        let content = format!(
            "Wow, nice job! You clicked {:?} {}",
            response.data.component_type, response.data.custom_id
        );
        response
            .create_followup_message(ctx.serenity_context, move |f| {
                f.content(content).ephemeral(true)
            })
            .await?;
    }
    //    ctx.say(response).await?;
    Ok(())
}
