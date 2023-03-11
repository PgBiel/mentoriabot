use async_trait::async_trait;

use crate::error::Result;
use poise::ApplicationContext;

/// Represents a form of sequential Discord interactions (without a Modal).
#[async_trait]
pub trait InteractionForm: Sync {
    type ContextData: Send + Sync;
    type ContextError: Send + Sync;

    /// Runs this form's components.
    async fn run_components(context: ApplicationContext<'_, Self::ContextData, Self::ContextError>) -> Result<Box<Self>>;

    async fn execute(context: ApplicationContext<'_, Self::ContextData, Self::ContextError>) -> Result<Box<Self>> {
        let mut data = Self::run_components(context).await?;
        data = data.on_finish(context).await?;
        Ok(data)
    }

    async fn on_finish(self: Box<Self>, _context: ApplicationContext<'_, Self::ContextData, Self::ContextError>) -> Result<Box<Self>> {
        Ok(self)
    }
}
