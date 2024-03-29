//! Contains the [`Error`] enum.
use std::fmt::{Display, Formatter};

use diesel_async::pooled_connection::deadpool;
use mentoriabot_forms as forms;
use poise::serenity_prelude as serenity;

/// An error in Mentoria Bot.
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

    /// A Google API [`Error`] occurred.
    ///
    /// [`Error`]: google_calendar3::client::Error
    GoogleApi(google_calendar3::client::Error),

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

    /// Indicates authentication failed.
    /// See [`Error`].
    ///
    /// [`Error`]: google_apis_common::oauth2::Error
    Auth(google_apis_common::oauth2::Error),

    /// Indicates [lettre] failed to parse an [`Address`].
    /// Holds an [`AddressError`].
    ///
    /// [`Address`]: lettre::address::Address
    /// [`AddressError`]: lettre::address::AddressError
    LettreAddress(lettre::address::AddressError),

    /// Holds a [lettre `Error`].
    ///
    /// [lettre `Error`]: lettre::error::Error
    Lettre(lettre::error::Error),

    /// Holds a [regex `Error`].
    ///
    /// [regex `Error`]: regex::Error
    Regex(regex::Error),

    /// Holds a [csv `Error`].
    ///
    /// [csv `Error`]: csv::Error
    Csv(csv::Error),

    /// Holds one or more validation errors.
    Validations(validator::ValidationErrors),

    #[allow(dead_code)]
    Generic(Box<dyn std::error::Error + Send + Sync>),

    /// Some other error
    #[allow(dead_code)]
    Other(&'static str),

    /// Some other error (string)
    #[allow(dead_code)]
    String(String),
}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! impl_from_error_private {
    ($err:ty => $variant:ident) => {
        impl From<$err> for Error {
            fn from(err: $err) -> Self {
                Self::$variant(err)
            }
        }
    };
}

macro_rules! impl_from_error {
    ($($err:ty => $variant:ident;)*) => {
        $(impl_from_error_private!($err => $variant);)*
    }
}

impl_from_error!(
    forms::FormError => Form;
    strum::ParseError => EnumParse;
    serenity::Error => Serenity;
    diesel::result::Error => Diesel;
    diesel::result::ConnectionError => DieselConnection;
    deadpool::BuildError => DeadpoolBuild;
    deadpool::PoolError => DeadpoolPool;
    google_calendar3::client::Error => GoogleApi;
    std::io::Error => Io;
    google_apis_common::oauth2::Error => Auth;
    lettre::address::AddressError => LettreAddress;
    lettre::error::Error => Lettre;
    regex::Error => Regex;
    csv::Error => Csv;
    validator::ValidationErrors => Validations;
    String => String;
);

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
            Self::GoogleApi(inner) => Display::fmt(&inner, f),
            Self::Io(inner) => Display::fmt(&inner, f),
            Self::Auth(inner) => Display::fmt(&inner, f),
            Self::LettreAddress(inner) => Display::fmt(&inner, f),
            Self::Lettre(inner) => Display::fmt(&inner, f),
            Self::Regex(inner) => Display::fmt(&inner, f),
            Self::Csv(inner) => Display::fmt(&inner, f),
            Self::Validations(inner) => Display::fmt(&inner, f),
            Self::DateTimeParse => write!(f, "Failed to parse the given date expression"),
            Self::CommandCheck(message) => write!(f, "{}", message),
            Self::Generic(inner) => Display::fmt(&inner, f),
            Self::Other(message) => write!(f, "{}", message),
            Self::String(message) => write!(f, "{}", message),
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
            Self::GoogleApi(inner) => Some(inner),
            Self::Regex(inner) => Some(inner),
            Self::Csv(inner) => Some(inner),
            Self::Validations(inner) => Some(inner),
            Self::Io(inner) => Some(inner),
            Self::Auth(inner) => Some(inner),
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
