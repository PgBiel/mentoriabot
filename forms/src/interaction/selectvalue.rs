//! Implements the SelectValue newtype wrapper.

use std::{convert::Infallible, fmt::Display, str::FromStr};

/// Represents a SelectOption's value key.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SelectValue(pub String);

impl FromStr for SelectValue {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl Display for SelectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl From<&str> for SelectValue {
    fn from(value: &str) -> Self {
        SelectValue(String::from(value))
    }
}
