use crate::common::ApplicationContext;
use crate::error::Result;
use async_trait::async_trait;

pub mod component;
pub mod testform;

pub use component::{FormComponent, MessageFormComponent, ModalFormComponent};

pub type ModalFormComponentBox<Modal, Form> = Box<dyn ModalFormComponent<Modal = Modal, Form = Form> + Send + Sync>;

/// Represents a form of sequential Discord interactions (without a Modal).
#[async_trait]
pub trait InteractionForm: Default + Sync {

    /// Generates this form's components, in order of execution.
    fn components() -> Vec<FormComponent<Self>>;

    /// Runs each form component, in order.
    async fn execute(context: ApplicationContext<'_>) -> Result<Self> {
        let mut data: Self = Default::default();

        for component in Self::components() {
            data = component.run(context, data).await?;
        }

        data = data.on_finish(context).await?;
        Ok(data)
    }

    /// Code to run after the form has finished.
    async fn on_finish(self, _context: ApplicationContext<'_>) -> Result<Self> {
        Ok(self)  // do nothing by default
    }
}


/// Represents a form of a Discord modal followed by sequential Discord interactions.
#[async_trait]
pub trait InteractionModalForm: Default + Sync {
    /// This form's Modal type.
    /// It is always the first component to be run.
    type Modal: poise::Modal + Send + Sync;

    /// Generates this form's Modal Component (use 'None' for no modal).
    fn modal() -> ModalFormComponentBox<Self::Modal, Self>;

    /// Generates this form's components, in order of execution.
    fn components() -> Vec<FormComponent<Self>>;

    async fn execute(context: ApplicationContext<'_>) -> Result<Self> {
        let mut data: Self = Default::default();

        data = Self::modal().run(context, data).await?;

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
