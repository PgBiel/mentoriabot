use std::fmt::{Display, Formatter};

use poise::serenity_prelude as serenity;

/// Represents an error within an [`InteractionForm`] or related objects.
///
/// [`InteractionForm`]: super::InteractionForm
#[derive(Debug)]
pub enum FormError {
    /// Indicates no User response was received in a certain amount of time (usually 15 minutes).
    NoResponse,

    /// Indicates the response received was invalid, or could not be converted to a certain type.
    InvalidUserResponse,

    /// Indicates this component cannot be awaited; that is, it is impossible to know
    /// whether or not a User interacted with this component.
    ///
    /// This is the case, for example, with link buttons, which just take the user to the given
    /// link, without giving any feedback to the bot about the user's action.
    CannotAwaitComponent,

    /// Form interrupted.
    Cancelled,

    /// Indicates a Serenity error occurred.
    Serenity(serenity::Error),
}

/// Shorthand for a [`Result`] with [`FormError`].
///
/// [`Result`]: core::result::Result
pub type Result<T> = std::result::Result<T, FormError>;

/// Shorthand for a [`Result`] with a [`ContextualError`], which may hold
/// both a FormError and a customized contextual Error.
///
/// [`Result`]: core::result::Result
pub type ContextualResult<T, E> = std::result::Result<T, ContextualError<E>>;

impl Display for FormError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoResponse => write!(f, "No interaction response received"),
            Self::InvalidUserResponse => write!(f, "Invalid user interaction response given"),
            Self::CannotAwaitComponent => {
                write!(f, "This form component is not awaitable (e.g. link button)")
            }
            Self::Cancelled => write!(f, "Form cancelled or interrupted."),
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

/// May hold either a FormError or a typical Error from the context.
pub enum ContextualError<E> {
    Form(FormError),
    Ctx(E),
}

impl<E> From<FormError> for ContextualError<E> {
    fn from(value: FormError) -> Self {
        Self::Form(value)
    }
}

impl<E> From<serenity::Error> for ContextualError<E> {
    fn from(value: serenity::Error) -> Self {
        Self::Form(value.into())
    }
}
