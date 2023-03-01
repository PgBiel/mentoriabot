use poise::serenity_prelude as serenity;
use super::super::common::{ApplicationContext, Error};

/// Displays your or another user's account creation date
#[poise::command(slash_command)]
pub async fn userup(
        ctx: ApplicationContext<'_>,
        #[description = "Selected user"] user: Option<serenity::User>,
        ) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    let id = ctx.interaction.id();

    let rx = ctx.data.interaction_handler()
        .wait_for_interaction(id.to_string(), serenity::InteractionType::MessageComponent).await;

    ctx.send(|create_reply| create_reply
                .content(response)
                .ephemeral(true)
                .components(|create_components|
                    create_components.create_action_row(|create_row|
                        create_row.create_button(|create_button|
                            create_button
                                .custom_id(id)
                                .label("OK")
                                .style(serenity::ButtonStyle::Primary))))).await?;

    if let Some(mut rx) = rx {
        if let Ok(interaction) = rx.changed().await.map(|_| rx.borrow()) {
            if let Some(serenity::Interaction::MessageComponent(component)) = &*interaction {
                let response = format!("Wow, nice {}", component.data.custom_id);
                println!("RESPONSE: {}", response);
            }
        }
    }
    //    ctx.say(response).await?;
    Ok(())
}
