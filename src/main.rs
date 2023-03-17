use poise::serenity_prelude as serenity;
use serenity::GuildId;

mod commands;
mod common;
mod config;
mod connection;
mod error;
mod events;
mod model;
mod repository;
mod schema;

use common::Data;
use config::MiniRustBotConfig as Config;

pub mod forms {
    pub use minirustbot_forms::*;
}

const CONFIG_FILE: &str = "config.json";

#[tokio::main]
async fn main() {
    let config_contents = std::fs::read_to_string(CONFIG_FILE)
        .expect("Could not read the bot's config.json.");

    let parsed_config = serde_json::from_str::<Config>(&config_contents)
        .expect("Failed to parse the bot's config.json. Ensure it follows the \
config.example.json structure.");

    let database_url = parsed_config.get_database_url();
    let conn = connection::ConnectionManager::create(database_url)
        .await
        .expect("Failed to connect to the bot's database.");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get_commands(),
            event_handler: events::handle,
            ..Default::default()
        })
        .token(parsed_config.get_token())
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    GuildId(parsed_config.get_guild_id()),
                )
                .await?;
                Ok(Data::new(conn, parsed_config.get_admin_userids().clone()))
            })
        });

    framework.run().await.unwrap();
}
