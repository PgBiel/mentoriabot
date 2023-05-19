//! Contains the [`Error`] enum.
use std::fmt::{Display, Formatter};

use diesel_async::pooled_connection::deadpool;
use minirustbot_forms as forms;
use poise::serenity_prelude as serenity;

/// An error in MiniRustBot.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// A [`FormError`] occurred while running an InteractionForm
    ///
    /// [`FormError`]: forms::FormError
    Form(forms::FormError),

    EnumParse(strum::ParseError),

    /// An [`Error`] occurred in Serenity
    ///
    /// [`Error`]: serenity::Error
    Serenity(serenity::Error),

    /// A Diesel operational [`Error`] occurred
    ///
    /// [`Error`]: diesel::result::Error
    Diesel(diesel::result::Error),

    /// A Diesel [`ConnectionError`] occurred.
    ///
    /// [`ConnectionError`]: diesel::result::ConnectionError
    DieselConnection(diesel::result::ConnectionError),

    /// A Deadpool [`BuildError`] occurred.
    ///
    /// [`BuildError`]: deadpool::BuildError
    DeadpoolBuild(deadpool::BuildError),

    /// A Deadpool [`PoolError`] occurred.
    ///
    /// [`PoolError`]: deadpool::PoolError
    DeadpoolPool(deadpool::PoolError),

    /// A Google Calendar [`Error`] occurred.
    ///
    /// [`Error`]: google_calendar3::Error
    Calendar(google_calendar3::Error),

    /// An [I/O `Error`] occurred.
    ///
    /// [I/O `Error`]: std::io::Error
    Io(std::io::Error),

    /// Indicates a [`HumanParseableDateTime`] failed to parse.
    ///
    /// [`HumanParseableDateTime`]: crate::util::HumanParseableDateTime
    DateTimeParse,

    /// If a command check failed (e.g. check
    /// if the user has a certain role).
    CommandCheck(&'static str),

    #[allow(dead_code)]
    Generic(Box<dyn std::error::Error + Send + Sync>),

    /// Some other error
    #[allow(dead_code)]
    Other(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! impl_from_error {
    ($err:ty => $variant:ident) => {
        impl From<$err> for Error {
            fn from(err: $err) -> Self {
                Self::$variant(err)
            }
        }
    };
}

impl_from_error!(forms::FormError => Form);
impl_from_error!(strum::ParseError => EnumParse);
impl_from_error!(serenity::Error => Serenity);
impl_from_error!(diesel::result::Error => Diesel);
impl_from_error!(diesel::result::ConnectionError => DieselConnection);
impl_from_error!(deadpool::BuildError => DeadpoolBuild);
impl_from_error!(deadpool::PoolError => DeadpoolPool);
impl_from_error!(google_calendar3::Error => Calendar);
impl_from_error!(std::io::Error => Io);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Form(inner) => Display::fmt(&inner, f),
            Self::EnumParse(inner) => Display::fmt(&inner, f),
            Self::Serenity(inner) => Display::fmt(&inner, f),
            Self::Diesel(inner) => Display::fmt(&inner, f),
            Self::DieselConnection(inner) => Display::fmt(&inner, f),
            Self::DeadpoolBuild(inner) => Display::fmt(&inner, f),
            Self::DeadpoolPool(inner) => Display::fmt(&inner, f),
            Self::Calendar(inner) => Display::fmt(&inner, f),
            Self::Io(inner) => Display::fmt(&inner, f),
            Self::DateTimeParse => write!(f, "Failed to parse the given date expression"),
            Self::CommandCheck(message) => write!(f, "{}", message),
            Self::Generic(inner) => Display::fmt(&inner, f),
            Self::Other(message) => write!(f, "{}", message),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Form(inner) => Some(inner),
            Self::EnumParse(inner) => Some(inner),
            Self::Serenity(inner) => Some(inner),
            Self::Diesel(inner) => Some(inner),
            Self::DieselConnection(inner) => Some(inner),
            Self::DeadpoolBuild(inner) => Some(inner),
            Self::DeadpoolPool(inner) => Some(inner),
            Self::Calendar(inner) => Some(inner),
            Self::Io(inner) => Some(inner),
            Self::Generic(inner) => Some(&**inner),
            _ => None,
        }
    }
}

// --- forms ---

impl From<Error> for forms::ContextualError<Error> {
    fn from(err: Error) -> Self {
        match err {
            Error::Form(err) => Self::Form(err),
            err => Self::Ctx(err),
        }
    }
}

impl From<forms::ContextualError<Error>> for Error {
    fn from(err: forms::ContextualError<Error>) -> Self {
        match err {
            forms::ContextualError::Form(err) => Self::Form(err),
            forms::ContextualError::Ctx(err) => err,
        }
    }
}
