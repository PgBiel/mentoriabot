use async_trait::async_trait;
use poise::ApplicationContext;

use crate::error::Result;

/// Represents a form of sequential Discord interactions.
#[async_trait]
pub trait InteractionForm: Sync {
    type ContextData: Send + Sync;
    type ContextError: Send + Sync;
    type FormData: Default + Send + Sync;

    /// Runs this form's components and builds this form's instance.
    ///
    /// Usually you'll want to use the [`execute`] method; this method
    /// is for behavior overriding.
    ///
    /// [`execute`]: InteractionForm::execute
    async fn run_components(
        context: ApplicationContext<'_, Self::ContextData, Self::ContextError>,
        form_data: Self::FormData,
    ) -> Result<Box<Self>>;

    /// Executes this form with the given context.
    /// This will run each component in order, and then run [`on_finish`].
    ///
    /// [`on_finish`]: InteractionForm::on_finish
    async fn execute(
        context: ApplicationContext<'_, Self::ContextData, Self::ContextError>,
    ) -> Result<Box<Self>> {
        let mut data = Self::run_components(context, Default::default()).await?;
        data = data.on_finish(context).await?;
        Ok(data)
    }

    /// Similar to [`execute`], but providing default values
    /// for data.
    async fn execute_with_defaults(
        context: ApplicationContext<'_, Self::ContextData, Self::ContextError>,
        form_data: Self::FormData,
    ) -> Result<Box<Self>> {
        let mut data = Self::run_components(context, form_data).await?;
        data = data.on_finish(context).await?;
        Ok(data)
    }

    /// Optional function to run when the form is finished.
    /// Normally, you'll just run [`execute`] and then use the finished
    /// form object.
    ///
    /// [`execute`]: InteractionForm::execute
    async fn on_finish(
        self: Box<Self>,
        _context: ApplicationContext<'_, Self::ContextData, Self::ContextError>,
    ) -> Result<Box<Self>> {
        Ok(self)
    }
}
