use serde::{Deserialize, Serialize};

/// The bot's parsed config file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct MentoriaBotConfig {
    /// This bot's token.
    pub(crate) token: String,

    /// The Guild IDs where the bot is functional.
    pub(crate) guild_ids: Vec<u64>,

    /// This bot's database URL.
    pub(crate) database_url: String,

    /// The Discord User IDs of this bot's administrators.
    #[serde(default)]
    pub(crate) admin_userids: Vec<u64>,

    /// ID of the calendar to use for Google Calendar operations.
    pub(crate) google_calendar_id: String,

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

impl From<LoggingLevels> for tracing::Level {
    fn from(value: LoggingLevels) -> Self {
        match value {
            LoggingLevels::Trace => Self::TRACE,
            LoggingLevels::Debug => Self::DEBUG,
            LoggingLevels::Info => Self::INFO,
            LoggingLevels::Warn => Self::WARN,
            LoggingLevels::Error => Self::ERROR,
        }
    }
}

fn info_variant() -> LoggingLevels {
    LoggingLevels::Info
}
