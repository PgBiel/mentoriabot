use poise::serenity_prelude as serenity;
use serenity::GuildId;
use tracing::{error, info, warn};

mod commands;
mod common;
mod config;
mod connection;
mod error;
mod events;
mod model;
mod repository;
mod schema;
mod util;

use common::Data;
use config::MiniRustBotConfig as Config;

use crate::{common::FrameworkError, error::Error};

pub mod forms {
    pub use minirustbot_forms::*;
}

const CONFIG_FILE: &str = "config.json";

fn on_error(framework_error: FrameworkError<'_>) -> poise::BoxFuture<'_, ()> {
    Box::pin(async move {
        let response = match &framework_error {
            FrameworkError::CommandCheckFailed { error: None, .. } => {
                "You cannot use this command for unknown reasons.".to_string()
            }
            FrameworkError::CommandCheckFailed {
                error: Some(Error::CommandCheck(message)),
                ..
            } => message.to_string(),
            FrameworkError::ArgumentParse { input, .. } => {
                format!(
                    "One of the given parameters had an invalid format{}.",
                    input
                        .as_ref()
                        .map(|s| format!(": '{s}'"))
                        .unwrap_or_default()
                )
            }
            FrameworkError::MissingBotPermissions {
                missing_permissions,
                ..
            } => {
                format!(
                    "The bot is missing the following permission{}: {}",
                    if missing_permissions.get_permission_names().len() > 1 {
                        "s"
                    } else {
                        ""
                    },
                    missing_permissions
                )
            }
            FrameworkError::MissingUserPermissions {
                missing_permissions,
                ..
            } => match missing_permissions {
                None => "You are missing certain permissions to run this command.".to_string(),
                Some(missing_permissions) => format!(
                    "You are missing the following permission{}: {}",
                    if missing_permissions.get_permission_names().len() > 1 {
                        "s"
                    } else {
                        ""
                    },
                    missing_permissions
                ),
            },
            FrameworkError::DmOnly { .. } => {
                "This command is exclusive to Direct Messages (DMs).".to_string()
            }
            FrameworkError::GuildOnly { .. } => "This command is exclusive to guilds.".to_string(),
            FrameworkError::NsfwOnly { .. } => {
                "This command is exclusive to NSFW channels.".to_string()
            }
            FrameworkError::NotAnOwner { .. } => "You're not a bot administrator.".to_string(),
            FrameworkError::CooldownHit {
                remaining_cooldown, ..
            } => {
                if remaining_cooldown.as_secs() < 1 {
                    "Please try running this command again.".to_string()
                } else {
                    format!(
                        "Please wait {} before using this command again.",
                        util::convert_duration_to_string(remaining_cooldown.clone())
                    )
                }
            }
            FrameworkError::UnknownInteraction { .. } => {
                "I do not recognize that command you're trying to run.".to_string()
            }
            FrameworkError::Setup { error, .. } => {
                error!("Failed to run bot setup: {error}");
                "Bot setup failed.".to_string()
            }
            FrameworkError::Command {
                error: Error::Diesel(error),
                ..
            } => {
                error!("Diesel database error: {error}");
                "An internal database error occurred. Sorry!".to_string()
            }
            FrameworkError::Command {
                error: error @ (Error::DieselConnection(_) | Error::DeadpoolPool(_)),
                ..
            } => {
                error!("Database connection error: {error}");
                "An internal database connection error occurred. Sorry!".to_string()
            }
            _ => {
                error!("Unexpected bot error: {}", framework_error);
                "Unexpected error occurred.".to_string()
            }
        };

        if let Some(ctx) = framework_error.ctx() {
            ctx.send(|b| b.content(format!("**Error:** {response}")))
                .await
                .err()
                .map(|err| warn!("Failed to reply with error message: {err}"))
                .unwrap_or_default()
        }
    })
}

#[tokio::main]
async fn main() {
    let config_contents =
        std::fs::read_to_string(CONFIG_FILE).expect("Could not read the bot's config.json.");

    let parsed_config = serde_json::from_str::<Config>(&config_contents).expect(
        "Failed to parse the bot's config.json. Ensure it follows the \
config.example.json structure.",
    );

    let Config {
        database_url,
        token,
        guild_id,
        admin_userids,
        default_logging_level,
    } = parsed_config;

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Into::<tracing::Level>::into(default_logging_level))
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default logging level failed.");

    let db = connection::DatabaseManager::new(&database_url)
        .expect("Failed to connect to the bot's database.");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get_commands(),
            event_handler: events::handle,
            on_error,
            owners: admin_userids
                .iter()
                .map(|id| serenity::UserId(id.clone()))
                .collect(),
            ..Default::default()
        })
        .token(&token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    GuildId(guild_id),
                )
                .await?;
                info!("Registered");
                Ok(Data::new(db, admin_userids))
            })
        });

    framework.run().await.unwrap();
}
