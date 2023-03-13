//! Implements the CustomId newtype wrapper.

use std::{convert::Infallible, fmt::Display, str::FromStr};

use crate::util::generate_custom_id;

/// Represents an interaction's custom ID.
///
/// Note that this struct implements [`Default`] by
/// generating a pseudorandom value, based on the current Unix
/// timestamp.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CustomId(pub String);

impl CustomId {
    /// Generates a CustomId based on the current Unix time.
    pub fn generate() -> Self {
        Self(generate_custom_id())
    }
}

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

/// Defaults the Custom ID to a pseudorandom value,
/// based on the current Unix timestamp.
impl Default for CustomId {
    fn default() -> Self {
        Self::generate()
    }
}

/// For types that may hold a custom ID.
pub trait HasCustomId {
    /// Get this instance's Custom ID, if present.
    fn get_custom_id(&self) -> Option<&CustomId>;
}

impl HasCustomId for CustomId {
    fn get_custom_id(&self) -> Option<&CustomId> {
        Some(self)
    }
}
