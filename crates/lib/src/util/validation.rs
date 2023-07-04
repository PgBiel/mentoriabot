//! Utilities for validation in the bot's code.

/// Indicates this value was not validated yet, and must
/// be validated to be used.
/// Part of the idea of making 'invalid states
/// unrepresentable'.
#[derive(Debug, Clone, PartialEq)]
pub struct Unvalidated<T: validator::Validate>(T);

impl<T: validator::Validate> Unvalidated<T> {
    /// Creates a new 'Unvalidated' instance with
    /// the given value. It will have to be validated
    /// to be taken out.
    pub fn new(value: T) -> Self {
        Self(value)
    }

    /// Try to validate the unvalidated item, returning it
    /// if possible, or erroring if invalid.
    pub fn validate(self) -> Result<T, validator::ValidationErrors> {
        self.0.validate().map(|_| self.0)
    }
}
