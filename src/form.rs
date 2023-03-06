use crate::common::ApplicationContext;
use crate::error::Result;
use async_trait::async_trait;

pub mod component;
pub mod testform;

pub use component::{MessageFormComponent, ModalFormComponent, SelectComponent, ButtonComponent, ButtonsComponent};

/// Represents a form of sequential Discord interactions (without a Modal).
#[async_trait]
pub trait InteractionForm: Sync {
    /// Runs this form's components.
    async fn run_components(context: ApplicationContext<'_>) -> Result<Box<Self>>;

    async fn execute(context: ApplicationContext<'_>) -> Result<Box<Self>> {
        let mut data = Self::run_components(context).await?;
        data = data.on_finish(context).await?;
        Ok(data)
    }

    async fn on_finish(self: Box<Self>, _context: ApplicationContext<'_>) -> Result<Box<Self>> {
        Ok(self)
    }
}
