use crate::{common::Context, error::Error};

/// Displays your or another user's account creation date
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|reply| reply.content("Pong!").ephemeral(true))
        .await?;
    Ok(())
}
