use poise::{Modal, serenity_prelude as serenity};

use crate::{common::ApplicationContext, error::Result, repository::{UserRepository, Repository}, model::NewUser};

#[derive(Default, Modal)]
#[name = "Adding a User"]
struct UserModal {
    #[name = "Name"]
    #[min_length = 3]
    #[max_length = 150]
    name: String,

    #[name = "Bio"]
    #[max_length = 1000]
    #[paragraph]
    bio: Option<String>,
}

/// Manages users in the database.
#[poise::command(slash_command,
    subcommands("add", "get", "remove", "all"),
    ephemeral,
)]
pub async fn userman(
    ctx: ApplicationContext<'_>
) -> Result<()> {
    ctx.send(|b| b.content("Use the 'add' command.").ephemeral(true)).await?;
    Ok(())
}

/// Adds a User to the database.
#[poise::command(slash_command, ephemeral)]
pub async fn add(
    ctx: ApplicationContext<'_>,
    #[description = "Whom to add as User"] user: serenity::User
) -> Result<()> {
    if !ctx.data.admin_userids.contains(ctx.author().id.as_u64()) {
        ctx.send(|b| b.content("Error: Not an admin user.").ephemeral(true)).await?;
    } else {
        let data = UserModal::execute(ctx).await?;
        if let Some(modal_data) = data {
            let new_user = NewUser {
                name: modal_data.name,
                discord_userid: user.id.as_u64().to_string(),
                bio: modal_data.bio,
            };
            let inserted_user;
            {
                let conn_mutex = ctx.data.connection.get_connection();
                let mut conn = conn_mutex.lock().await;
                inserted_user = UserRepository::upsert(&mut conn, new_user).await?;
            }
            let response = format!(
                "Successfully added Mr. {}. to the database (with{} a bio).",
                inserted_user.name,
                if inserted_user.bio.is_none() { "out" } else { "" }
            );
            ctx.send(|f| f.content(response).ephemeral(true)).await?;
        }
        //    ctx.say(response).await?;
    }


    Ok(())
}

/// Gets a User from the database
#[poise::command(slash_command, ephemeral)]
pub async fn get(
    ctx: ApplicationContext<'_>,
    #[description = "Whose User info to get"] user: serenity::User
) -> Result<()> {
    if !ctx.data.admin_userids.contains(ctx.author().id.as_u64()) {
        ctx.send(|b| b.content("Error: Not an admin user.").ephemeral(true)).await?;
    } else {
        let conn_mutex = ctx.data.connection.get_connection();
        let mut conn = conn_mutex.lock().await;
        let found_user = UserRepository::find_by_discordid(&mut conn, user.id.into()).await?;
        if let Some(found_user) = found_user {
            let bio = found_user.bio;
            let response = format!(
                "We have Mr. {} with {}.",
                found_user.name,
                if bio.is_some() { "the bio below" } else { "no bio" }
            );

            if let Some(bio) = bio {
                ctx.send(|f| f
                    .content(response)
                    .ephemeral(true)
                    .embed(|f| f
                        .title("Their bio")
                        .description(bio)
                        .color(serenity::Colour::BLITZ_BLUE))).await?;
            } else {
                ctx.send(|f| f
                    .content(response)
                    .ephemeral(true)).await?;

            }
        } else {
            ctx.send(|b| b.content("User not registered.").ephemeral(true)).await?;
        }
    }

    Ok(())
}

/// Removes a User from the database
#[poise::command(slash_command, ephemeral)]
pub async fn remove(
    ctx: ApplicationContext<'_>,
    #[description = "Whose User info to remove"] user: serenity::User
) -> Result<()> {
    if !ctx.data.admin_userids.contains(ctx.author().id.as_u64()) {
        ctx.send(|b| b.content("Error: Not an admin user.").ephemeral(true)).await?;
    } else {
        let conn_mutex = ctx.data.connection.get_connection();
        let mut conn = conn_mutex.lock().await;
        let found_user = UserRepository::find_by_discordid(&mut conn, user.id.into()).await?;
        if let Some(found_user) = found_user {
            let removed_msg = format!("Successfully removed {} from the database.", found_user.name);
            UserRepository::remove(&mut conn, found_user).await?;

            ctx.send(|b| b.content(removed_msg).ephemeral(true)).await?;
        } else {
            ctx.send(|b| b.content("User not registered, so cannot be removed.").ephemeral(true)).await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command, ephemeral)]
pub async fn all(
    ctx: ApplicationContext<'_>
) -> Result<()> {
    if !ctx.data.admin_userids.contains(ctx.author().id.as_u64()) {
        ctx.send(|b| b.content("Error: Not an admin user.").ephemeral(true)).await?;
    } else {
        let conn_mutex = ctx.data.connection.get_connection();
        let mut conn = conn_mutex.lock().await;
        let users = UserRepository::find_all(&mut conn).await?;
        let mut text = String::from("Users:");
        for user in users {
            text.push_str(&format!("\n- {:?}", user));
        }
        ctx.send(|b| b.content(text).ephemeral(true)).await?;
    }

    Ok(())
}
