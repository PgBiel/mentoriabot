use poise::serenity_prelude as serenity;
use super::super::common::{Context, Error};

/// Displays your or another user's account creation date
#[poise::command(slash_command)]
pub async fn userup(
        ctx: Context<'_>,
        #[description = "Selected user"] user: Option<serenity::User>,
        ) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());

    ctx.send(|create_reply| create_reply
                .content(response)
                .ephemeral(true)
                .components(|create_components|
                    create_components.create_action_row(|create_row|
                        create_row.create_button(|create_button|
                            create_button
                                .custom_id("cool_button")
                                .label("OK")
                                .style(serenity::ButtonStyle::Primary))))).await?;
    //    ctx.say(response).await?;
    Ok(())
}
