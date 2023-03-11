use std::fmt::{Display, Formatter};

use poise::serenity_prelude as serenity;

/// Represents an error within an InteractionForm object.
#[derive(Debug)]
pub enum FormError {
    NoResponse,
    InvalidUserResponse,
    CannotAwaitComponent,

    Serenity(serenity::Error),
}

pub type Result<T> = std::result::Result<T, FormError>;

impl Display for FormError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoResponse => write!(f, "No interaction response received"),
            Self::InvalidUserResponse => write!(f, "Invalid user interaction response given"),
            Self::CannotAwaitComponent => {
                write!(f, "This form component is not awaitable (e.g. link button)")
            }
            Self::Serenity(cause) => Display::fmt(cause, f),
        }
    }
}

impl std::error::Error for FormError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serenity(cause) => Some(cause),
            _ => None,
        }
    }
}

impl From<serenity::Error> for FormError {
    fn from(value: serenity::Error) -> Self {
        Self::Serenity(value)
    }
}
