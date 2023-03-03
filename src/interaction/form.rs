use crate::common::ApplicationContext;
use crate::error::Result;

pub mod testform;
pub mod component;

pub use component::{InteractionFormComponent, MessageFormComponent, ModalFormComponent};
pub use component::{MessageFormComponentBuilder, ModalFormComponentBuilder};

pub type FormComponentsVec<FormResultData> =
    Vec<Box<dyn InteractionFormComponent<FormResultData=FormResultData> + Send + Sync>>;

/// A stateless object for spawning multiple interactions in a row and obtaining data from their
/// responses.
#[derive(Default)]
pub struct InteractionForm<FormResultData: Default> {
    /// The interaction components in this form, in order of execution.
    components: FormComponentsVec<FormResultData>
}

impl<FormResultData: Default> InteractionForm<FormResultData> {
    pub fn new(components: FormComponentsVec<FormResultData>) -> Self {
        InteractionForm { components }
    }

    /// Runs all interaction components in this form, sequentially, in the given context.
    pub async fn run(&self, context: ApplicationContext<'_>) -> Result<FormResultData> {
        let mut data: FormResultData = Default::default();
        for component in &self.components {
            data = component.run(context, data).await?;
        }
        Ok(data)
    }
}
