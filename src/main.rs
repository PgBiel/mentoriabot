use poise::serenity_prelude as serenity;
use serenity::GuildId;
use tracing::{error, info, warn};

mod commands;
mod common;
mod config;
mod events;

use common::Data;
use config::MiniRustBotConfig as Config;
use minirustbot_forms::FormError;

use crate::{
    common::FrameworkError,
    lib::{
        db,
        error::Error,
        notification,
        util::{self, tr},
    },
};

pub mod forms {
    pub use minirustbot_forms::*;
}
pub mod lib {
    pub use minirustbot_lib::*;
}

const CONFIG_FILE: &str = "config.json";

rust_i18n::i18n!("locales");

fn on_error(framework_error: FrameworkError<'_>) -> poise::BoxFuture<'_, ()> {
    Box::pin(async move {
        let locale = framework_error
            .ctx()
            .map(util::locale::get_defaulted_locale)
            .unwrap_or("pt-BR");

        let response = match &framework_error {
            FrameworkError::CommandCheckFailed { error: None, .. } => {
                tr!("main_on_error.command_check.default", locale = locale)
            }
            FrameworkError::CommandCheckFailed {
                error: Some(Error::CommandCheck(message)),
                ..
            } => message.to_string(),
            FrameworkError::ArgumentParse { input, .. } => {
                if let Some(invalid_format) = input.as_ref() {
                    tr!(
                        "main_on_error.argument_parse.with_message",
                        locale = locale,
                        "message" => invalid_format
                    )
                } else {
                    tr!("main_on_error.argument_parse.default", locale = locale)
                }
            }
            FrameworkError::MissingBotPermissions {
                missing_permissions,
                ..
            } => {
                if missing_permissions.get_permission_names().len() != 1 {
                    tr!(
                        "main_on_error.missing_bot_permissions.plural",
                        locale = locale,
                        "permissions" => {
                            let permissions = missing_permissions.to_string();
                            if locale == "pt-BR" {
                                permissions.replace("and ", "e ")
                            } else {
                                permissions
                            }
                        }
                    )
                } else {
                    tr!(
                        "main_on_error.missing_bot_permissions.singular",
                        locale = locale
                    )
                }
            }
            FrameworkError::MissingUserPermissions {
                missing_permissions,
                ..
            } => match missing_permissions {
                None => tr!(
                    "main_on_error.missing_user_permissions.default",
                    locale = locale
                ),
                Some(missing_permissions) => {
                    if missing_permissions.get_permission_names().len() != 1 {
                        tr!(
                            "main_on_error.missing_user_permissions.plural",
                            locale = locale,
                            "permissions" => {
                                let permissions = missing_permissions.to_string();
                                if locale == "pt-BR" {
                                    permissions.replace("and ", "e ")
                                } else {
                                    permissions
                                }
                            }
                        )
                    } else {
                        tr!(
                            "main_on_error.missing_user_permissions.singular",
                            locale = locale
                        )
                    }
                }
            },
            FrameworkError::DmOnly { .. } => {
                tr!("main_on_error.dm_only.default", locale = locale)
            }
            FrameworkError::GuildOnly { .. } => {
                tr!("main_on_error.guild_only.default", locale = locale)
            }
            FrameworkError::NsfwOnly { .. } => {
                tr!("main_on_error.nsfw_only.default", locale = locale)
            }
            FrameworkError::NotAnOwner { .. } => {
                tr!("main_on_error.owners_only.default", locale = locale)
            }
            FrameworkError::CooldownHit {
                remaining_cooldown, ..
            } => {
                if remaining_cooldown.as_secs() < 1 {
                    tr!("main_on_error.cooldown_hit.default", locale = locale)
                } else {
                    tr!(
                        "main_on_error.cooldown_hit.with_duration",
                        locale = locale,
                        "duration" => {
                            if locale == "pt-BR" {
                                util::locale::convert_duration_to_brazilian_string(*remaining_cooldown)
                            } else {
                                util::locale::convert_duration_to_string(*remaining_cooldown)
                            }
                        }
                    )
                }
            }
            FrameworkError::UnknownInteraction { .. } => {
                tr!("main_on_error.unknown_interaction.default", locale = locale)
            }
            FrameworkError::Setup { error, .. } => {
                error!("Failed to run bot setup: {error}");
                tr!("main_on_error.setup.default", locale = locale)
            }
            FrameworkError::Command { error, .. } => match error {
                Error::Diesel(error) => {
                    error!("Diesel database error: {error}");
                    tr!("main_on_error.database.default", locale = locale)
                }
                Error::DieselConnection(_) | Error::DeadpoolPool(_) => {
                    error!("Database connection error: {error}");
                    tr!("main_on_error.database_connection.default", locale = locale)
                }
                Error::Form(FormError::Cancelled) => "".to_string(),
                _ => {
                    error!("Unexpected command error: {}: {}", framework_error, error);
                    tr!("main_on_error.unexpected.default", locale = locale)
                }
            },
            _ => {
                error!("Unexpected bot error: {}", framework_error);
                tr!("main_on_error.unexpected.default", locale = locale)
            }
        };

        if !response.is_empty() {
            if let Some(ctx) = framework_error.ctx() {
                ctx.send(|b| {
                    b.content(tr!(
                        "main_on_error.error_message",
                        locale = locale,
                        "message" => response
                    ))
                })
                .await
                .err()
                .map(|err| warn!("Failed to reply with error message: {err}"))
                .unwrap_or_default()
            }
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
        guild_ids,
        admin_userids,
        default_logging_level,
        google_calendar_oauth2_secret,
        ..
    } = parsed_config;

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Into::<tracing::Level>::into(default_logging_level))
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default logging level failed.");

    let db =
        db::DatabaseManager::new(&database_url).expect("Failed to connect to the bot's database.");

    // FIXME: Google Calendar ID
    let google = notification::GoogleApiManager::connect(google_calendar_oauth2_secret, "", "me")
        .await
        .expect("Failed to connect to the Google API.");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get_commands(),
            event_handler: events::handle,
            on_error,
            owners: admin_userids
                .iter()
                .map(|id| serenity::UserId(*id))
                .collect(),
            ..Default::default()
        })
        .token(&token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                // register commands in the guild IDs.
                for guild_id in guild_ids {
                    info!("Registering for {}...", guild_id);
                    poise::builtins::register_in_guild(
                        ctx,
                        &framework.options().commands,
                        GuildId(guild_id),
                    )
                    .await?;
                }
                info!("Registered");
                Ok(Data::new(db, admin_userids, google))
            })
        });

    framework.run().await.unwrap();
}
