use poise::serenity_prelude as serenity;
use serenity::GuildId;

mod commands;
mod common;
mod config;
mod error;
mod events;
mod schema;
mod model;
mod repository;
mod connection;

use common::Data;
use config::MiniRustBotConfig as Config;

pub mod macros {
    pub use minirustbot_macros::*;
}

pub mod forms {
    pub use minirustbot_forms::*;
}

const CONFIG_FILE: &str = "config.json";

#[tokio::main]
async fn main() {
    let config_contents = std::fs::read_to_string(CONFIG_FILE).unwrap();
    let parsed_config = serde_json::from_str::<Config>(&config_contents).unwrap();

    let database_url = parsed_config.get_database_url().clone();
    let conn = connection::create_connection(&database_url);

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
                Ok(Data::new())
            })
        });

    framework.run().await.unwrap();
}
