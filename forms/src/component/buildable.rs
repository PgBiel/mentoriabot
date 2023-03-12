use crate::interaction::HasCustomId;

/// Types implementing this trait can modify builders,
/// generally from reply or component building.
pub trait Buildable<Builder> {

    /// Function that modifies the builder
    /// according to this instance's data,
    /// and returns the modified builder.
    fn on_build<'a>(
        &self,
        builder: &'a mut Builder,
    ) -> &'a mut Builder;
}

/// Buildable is automatically implemented for lambdas
/// with the same signature as `on_build`.
impl<B, T> Buildable<B> for T where T: Fn(&mut B) -> &mut B {
    fn on_build<'a>(
        &self,
        builder: &'a mut B,
    ) -> &'a mut B {
        self(builder)
    }
}

/// Union of the [`Buildable`] and [`HasCustomId`] traits.
pub trait BuildableWithId<Builder>: Buildable<Builder> + HasCustomId {}

impl<B, T> BuildableWithId<B> for T where T: Buildable<B> + HasCustomId {}
