use crate::common::ApplicationContext;
use crate::error::Result;
use async_trait::async_trait;

pub mod testform;
pub mod component;

pub use component::{MessageFormComponent, ModalFormComponent, FormComponent};

/// Represents a form of sequential Discord interactions.
#[async_trait]
pub trait InteractionForm: Default + Sync {  // TODO: Only one modal per form (=> separate method, modal can be specialized etc.)
    fn components() -> Vec<FormComponent<Self>>;

    async fn execute(context: ApplicationContext<'_>) -> Result<Self> {
        let mut data: Self = Default::default();
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
