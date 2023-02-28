use poise::serenity_prelude as serenity;
use serenity::GuildId;

mod minirustbot;
mod config;

use minirustbot::{common, commands};
use common::Data;
use config::MiniRustBotConfig as Config;

const CONFIG_FILE: &str = "config.json";

#[tokio::main]
async fn main() {
    let config_contents = std::fs::read_to_string(CONFIG_FILE).unwrap();
    let parsed_config = serde_json::from_str::<Config>(&config_contents).unwrap();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get_commands(),
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
