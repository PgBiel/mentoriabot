use poise::serenity_prelude as serenity;
use serenity::GuildId;

mod minirustbot;
mod config;

use minirustbot::{common, events};
use common::{Context, Data, Error};
use config::MiniRustBotConfig as Config;

const CONFIG_FILE: &str = "config.json";

/// Displays your or another user's account creation date
#[poise::command(slash_command)]
async fn userup(
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

#[tokio::main]
async fn main() {
    let config_contents = std::fs::read_to_string(CONFIG_FILE).unwrap();
    let parsed_config = serde_json::from_str::<Config>(&config_contents).unwrap();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![userup()],
            event_handler: |ctx, event, framework_context, data| {
                data.get_event_handler().handle(ctx, event, framework_context, data)
            },
            ..Default::default()
        })
        .token(parsed_config.get_token())
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx,
                                                   &framework.options().commands,
                                                   GuildId(parsed_config.get_guild_id())).await?;
                Ok(Data::new())
            })
        });

    framework.run().await.unwrap();
}
