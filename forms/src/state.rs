/// Holds State between components during the evaluation of an [`InteractionForm`].
/// Use the `data` field to access a user-defined data struct. By default, it is an empty
/// tuple (no user data).
///
/// [`InteractionForm`]: crate::form::InteractionForm
pub struct FormState<T = ()> {
    /// User-defined data shared between components.
    pub data: T,

    /// A queue of subcomponent ID prefixes. Useful
    /// to be able to correlate sent components and received responses.
    pub(crate) subcomponent_id_queue: Vec<String>,
}

impl<T> FormState<T> {
    /// Creates a new [`FormState`] with the given user data.
    pub fn new(data: T) -> Self {
        Self {
            data,
            subcomponent_id_queue: Vec::new(),
        }
    }
}
