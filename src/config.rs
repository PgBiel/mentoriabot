use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// The bot's parsed config file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct MiniRustBotConfig {
    /// This bot's token.
    pub(crate) token: String,

    /// This bot's Guild ID.
    pub(crate) guild_id: u64,

    /// This bot's database URL.
    pub(crate) database_url: String,

    /// The Discord User IDs of this bot's administrators.
    #[serde(default)]
    pub(crate) admin_userids: Vec<u64>,

    /// The default logging level for the application
    /// (e.g. "info").
    #[serde(default = "info_variant")]
    pub(crate) default_logging_level: LoggingLevels,
}

/// Possible logging levels.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum LoggingLevels {
    #[serde(alias = "Trace", alias = "TRACE")]
    Trace,

    #[serde(alias = "Debug", alias = "DEBUG")]
    Debug,

    #[serde(alias = "Info", alias = "INFO")]
    Info,

    #[serde(alias = "Warn", alias = "WARN")]
    Warn,

    #[serde(alias = "Error", alias = "ERROR")]
    Error,
}

impl Into<tracing::Level> for LoggingLevels {
    fn into(self) -> tracing::Level {
        match self {
            Self::Trace => tracing::Level::TRACE,
            Self::Debug => tracing::Level::DEBUG,
            Self::Info => tracing::Level::INFO,
            Self::Warn => tracing::Level::WARN,
            Self::Error => tracing::Level::ERROR,
        }
    }
}

fn info_variant() -> LoggingLevels {
    LoggingLevels::Info
}
