use crate::{common::Context, lib::error::Error};

/// Checks if the bot is alive
#[poise::command(
    slash_command,
    ephemeral,
    description_localized("pt-BR", "Verifica se o bot está funcionando.")
)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|reply| reply.content("Pong!").ephemeral(true))
        .await?;
    Ok(())
}
