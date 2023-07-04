use poise::serenity_prelude as serenity;

use crate::{
    commands::modals::register::RegisterModal,
    common::ApplicationContext,
    lib::{
        db::{Repository, UpdatableRepository},
        error::Result,
    },
};

/// Manages users in the database.
#[poise::command(slash_command, subcommands("add", "get", "remove", "all"), ephemeral)]
pub async fn userman(ctx: ApplicationContext<'_>) -> Result<()> {
    ctx.send(|b| b.content("Use the 'add' command.").ephemeral(true))
        .await?;
    Ok(())
}

/// Adds a User to the database.
#[poise::command(slash_command, ephemeral, owners_only)]
pub async fn add(
    ctx: ApplicationContext<'_>,
    #[description = "Whom to add as User"] user: serenity::User,
) -> Result<()> {
    let data = RegisterModal::ask(ctx).await?;
    if let Some(modal_data) = data {
        let new_user = modal_data.generate_new_user(user.id.into());
        let inserted_user = ctx.data.db.user_repository().upsert(&new_user).await?;
        let response = format!(
            "Successfully added Mr. {} ({}) to the database (with{} a bio).",
            inserted_user.name,
            inserted_user.email,
            if inserted_user.bio.is_none() {
                "out"
            } else {
                ""
            }
        );
        ctx.send(|f| f.content(response).ephemeral(true)).await?;
    }
    //    ctx.say(response).await?;

    Ok(())
}

/// Gets a User from the database
#[poise::command(slash_command, ephemeral, owners_only)]
pub async fn get(
    ctx: ApplicationContext<'_>,
    #[description = "Whose User info to get"] user: serenity::User,
) -> Result<()> {
    if !ctx.data.admin_userids.contains(ctx.author().id.as_u64()) {
        ctx.send(|b| b.content("Error: Not an admin user.").ephemeral(true))
            .await?;
    } else {
        let found_user = ctx.data.db.user_repository().get(user.id.into()).await?;
        if let Some(crate::lib::model::User {
            bio, name, email, ..
        }) = found_user
        {
            let response = format!(
                "We have Mr. {name} ({email}) with {}.",
                if bio.is_some() {
                    "the bio below"
                } else {
                    "no bio"
                }
            );

            if let Some(bio) = bio {
                ctx.send(|f| {
                    f.content(response).ephemeral(true).embed(|f| {
                        f.title("Their bio")
                            .description(bio)
                            .color(serenity::Colour::BLITZ_BLUE)
                    })
                })
                .await?;
            } else {
                ctx.send(|f| f.content(response).ephemeral(true)).await?;
            }
        } else {
            ctx.send(|b| b.content("User not registered.").ephemeral(true))
                .await?;
        }
    }

    Ok(())
}

/// Removes a User from the database
#[poise::command(slash_command, ephemeral, owners_only)]
pub async fn remove(
    ctx: ApplicationContext<'_>,
    #[description = "Whose User info to remove"] user: serenity::User,
) -> Result<()> {
    let found_user = ctx.data().db.user_repository().get(user.id.into()).await?;
    if let Some(found_user) = found_user {
        let removed_msg = format!(
            "Successfully removed {} from the database.",
            found_user.name
        );
        ctx.data().db.user_repository().remove(&found_user).await?;

        ctx.send(|b| b.content(removed_msg).ephemeral(true)).await?;
    } else {
        ctx.send(|b| {
            b.content("User not registered, so cannot be removed.")
                .ephemeral(true)
        })
        .await?;
    }

    Ok(())
}

#[poise::command(slash_command, ephemeral, owners_only)]
pub async fn all(ctx: ApplicationContext<'_>) -> Result<()> {
    let users = ctx.data().db.user_repository().find_all().await?;
    let mut text = String::from("Users:");
    for user in users {
        text.push_str(&format!("\n- {:?}", user));
    }
    ctx.send(|b| b.content(text).ephemeral(true)).await?;

    Ok(())
}
