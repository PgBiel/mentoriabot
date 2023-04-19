use std::ops::{Deref, DerefMut};

use crate::CustomId;

/// Holds State between components during the evaluation of an [`InteractionForm`].
/// Use the `data` field to access a user-defined data struct. By default, it is an empty
/// tuple (no user data).
///
/// [`InteractionForm`]: crate::form::InteractionForm
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormState<T = ()> {
    /// User-defined data shared between components.
    pub data: T,

    /// A stack of subcomponent ID lists. Useful
    /// to be able to correlate sent components and received responses.
    /// Mostly intended for use with the component-deriving macro.
    pub subcomponent_id_stack: Vec<Vec<CustomId>>,
}

impl<T> FormState<T> {
    /// Creates a new [`FormState`] with the given user data.
    pub fn new(data: T) -> Self {
        Self {
            data,
            subcomponent_id_stack: Vec::new(),
        }
    }
}

impl<T> Deref for FormState<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for FormState<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
