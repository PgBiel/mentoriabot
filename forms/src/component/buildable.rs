use poise::serenity_prelude as serenity;

use crate::interaction::HasCustomId;

/// Types implementing this trait can modify builders,
/// generally from reply or component building.
pub trait Buildable<Builder> {
    /// Function that modifies the builder
    /// according to this instance's data,
    /// and returns the modified builder.
    fn on_build<'a>(&self, builder: &'a mut Builder) -> &'a mut Builder;
}

macro_rules! implement_buildable_for_lambda_with_type {
    ($Builder:ty) => {
        /// [`Buildable`] is automatically implemented for certain lambdas
        /// with the same signature as `on_build`.
        impl<T> Buildable<$Builder> for T
        where
            T: Fn(&mut $Builder) -> &mut $Builder,
        {
            fn on_build<'a>(&self, builder: &'a mut $Builder) -> &'a mut $Builder {
                self(builder)
            }
        }
    };
}

implement_buildable_for_lambda_with_type!(serenity::CreateAllowedMentions);
implement_buildable_for_lambda_with_type!(serenity::CreateEmbed);
implement_buildable_for_lambda_with_type!(serenity::CreateActionRow);
implement_buildable_for_lambda_with_type!(serenity::CreateButton);
implement_buildable_for_lambda_with_type!(serenity::CreateSelectMenu);

/// Union of the [`Buildable`] and [`HasCustomId`] traits.
/// (Automatically implemented for any trait that implements both of those traits.)
pub trait BuildableWithId<Builder>: Buildable<Builder> + HasCustomId {}

impl<B, T> BuildableWithId<B> for T where T: Buildable<B> + HasCustomId {}

/// Automatically implement a [`Buildable`] for a Component with a single row
/// when a type implements `Buildable` for [`serenity::CreateActionRow`].
impl<T> Buildable<serenity::CreateComponents> for T
where
    T: Buildable<serenity::CreateActionRow>,
{
    fn on_build<'a>(
        &self,
        builder: &'a mut serenity::CreateComponents,
    ) -> &'a mut serenity::CreateComponents {
        builder.create_action_row(|builder| {
            <Self as Buildable<serenity::CreateActionRow>>::on_build(&self, builder)
        })
    }
}
