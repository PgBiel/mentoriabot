//! Implements the SelectValue newtype wrapper.

use std::{convert::Infallible, fmt::Display, str::FromStr};

/// Represents a SelectOption's value key. Holds it as a String.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SelectValue(pub String);

impl FromStr for SelectValue {
    type Err = Infallible;

    /// Conversion from a string will always work.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl Display for SelectValue {
    /// Displays the value as a string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl From<String> for SelectValue {
    /// Converts from a String by simply wrapping it.
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for SelectValue {
    /// Converts from a string reference to a SelectValue
    /// by turning into a String in the process.
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl From<SelectValue> for String {
    /// Unwraps the String value held within.
    fn from(value: SelectValue) -> Self {
        value.0
    }
}
