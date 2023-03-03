use crate::common::ApplicationContext;
use crate::error::Result;
use async_trait::async_trait;

pub mod component;
pub mod testform;

pub use component::{FormComponent, MessageFormComponent, ModalFormComponent};

/// Represents a form of sequential Discord interactions.
#[async_trait]
pub trait InteractionForm: Default + Sync {
    /// This form's Modal, if any (use '()' for none).
    /// It is always the first component to be run.
    type Modal: poise::Modal + Send + Sync;

    /// Generates this form's Modal Component (use 'None' for no modal).
    fn modal() -> Option<Box<dyn ModalFormComponent<Modal = Self::Modal, Form = Self> + Send + Sync>>
    {
        None
    }

    /// Generates this form's components, in order of execution.
    fn components() -> Vec<FormComponent<Self>>;

    async fn execute(context: ApplicationContext<'_>) -> Result<Self> {
        let mut data: Self = Default::default();

        data = match Self::modal() {
            Some(modal_component) => modal_component.run(context, data).await?,
            None => data,
        };

        for component in Self::components() {
            data = component.run(context, data).await?;
        }
        data = data.on_finish(context).await?;
        Ok(data)
    }

    async fn on_finish(self, _context: ApplicationContext<'_>) -> Result<Self> {
        Ok(self)
    }
}
