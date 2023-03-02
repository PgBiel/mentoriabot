use poise::serenity_prelude as serenity;
use std::fmt::{Display, Formatter};

/// Represents an error within an InteractionForm object.
#[derive(Debug)]
pub enum FormError {
    NoResponse,
    InvalidUserResponse
}

impl Display for FormError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoResponse => write!(f, "No interaction response received"),
            Self::InvalidUserResponse => write!(f, "Invalid user interaction response given"),
        }
    }
}

impl std::error::Error for FormError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
