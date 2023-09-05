use super::modals::register::RegisterModal;
use crate::{
    common::ApplicationContext,
    lib::{
        db::{Repository, UpdatableRepository},
        error::Result,
        model::User,
        tr,
    },
};

/// Creates or updates your current information the bot has
#[poise::command(
    slash_command,
    ephemeral,
    name_localized("pt-BR", "cadastro"),
    description_localized("pt-BR", "Realiza ou altera seu cadastro de informações no bot.")
)]
pub async fn register(ctx: ApplicationContext<'_>) -> Result<()> {
    let author_id = ctx.author().id.into();
    if let Some(user) = ctx.data.db.user_repository().get(author_id).await? {
        let User {
            discord_id,
            name,
            email,
            bio,
        } = user.clone();

        // user already exists => present a modal with their existing data and let them change
        if let Some(modal) = RegisterModal::ask_with_defaults(ctx, name, email, bio).await? {
            let new_user = modal.generate_new_user(discord_id);
            ctx.data
                .db
                .user_repository()
                .update(&user, new_user.into())
                .await?;
            ctx.say(tr!("commands.register.updated_register_success", ctx = ctx))
                .await?;
        }
    } else if let Some(modal) = RegisterModal::ask(ctx).await? {
        // user doesn't exist so we asked them for brand new data
        let new_user = modal.generate_new_user(author_id);
        ctx.data.db.user_repository().insert(&new_user).await?;
        ctx.say(tr!("commands.register.new_register_success", ctx = ctx))
            .await?;
    }

    Ok(())
}
