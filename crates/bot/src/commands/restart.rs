use crate::{
    common::ApplicationContext,
    lib::{error::Result, tr},
};

/// Restarts the bot.
#[poise::command(
    slash_command,
    ephemeral,
    owners_only,
    name_localized("pt-BR", "reiniciar"),
    description_localized("pt-BR", "Reinicia o bot.")
)]
pub async fn restart(ctx: ApplicationContext<'_>) -> Result<()> {
    // if sending the message fails, ignore
    ctx.say(tr!("commands.restart.trying", ctx = ctx))
        .await
        .ok();

    ctx.framework
        .shard_manager
        .lock()
        .await
        .shutdown_all()
        .await;
    Ok(())
}
