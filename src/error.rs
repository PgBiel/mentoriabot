use std::fmt::{Display, Formatter};

use poise::serenity_prelude as serenity;

use crate::forms;

/// An error in MiniRustBot.
#[derive(Debug)]
pub enum Error {
    /// An error occurred while running an InteractionForm
    Form(forms::FormError),

    EnumParse(strum::ParseError),

    /// An error occurred in Serenity
    Serenity(serenity::Error),

    /// Some other error
    Other(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<FormError> for Error {
    fn from(err: FormError) -> Self {
        Self::Form(err)
    }
}

impl From<strum::ParseError> for Error {
    fn from(err: strum::ParseError) -> Self {
        Self::EnumParse(err)
    }
}

impl From<serenity::Error> for Error {
    fn from(err: serenity::Error) -> Self {
        Self::Serenity(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Form(inner) => Display::fmt(&inner, f),
            Self::EnumParse(inner) => Display::fmt(&inner, f),
            Self::Serenity(inner) => Display::fmt(&inner, f),
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
            _ => None,
        }
    }
}
