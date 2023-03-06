//! Implements the CustomId newtype wrapper.

use std::{str::FromStr, convert::Infallible, fmt::Display};

/// Represents an interaction's custom ID.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CustomId(pub String);

impl FromStr for CustomId {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CustomId(String::from(s)))
    }
}

impl Display for CustomId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
