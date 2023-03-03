mod message;
mod modal;

pub use message::MessageFormComponent;
pub use modal::ModalFormComponent;

use crate::common::ApplicationContext;
use crate::error::Result;

use super::InteractionForm;

/// An interaction sender/receiver which serves as component
/// of a form.
pub enum FormComponent<Form: InteractionForm> {
    Message(Box<dyn MessageFormComponent<Form = Form> + Send + Sync>),
    Modal(Box<dyn ModalFormComponent<Form = Form> + Send + Sync>)
}

impl<Form: InteractionForm + Send + Sync> FormComponent<Form> {
    pub async fn run(&self, context: ApplicationContext<'_>, form_data: Form) -> Result<Form> {
        match self {
            Self::Message(component) => component.run(context, form_data).await,
            Self::Modal(component) => component.run(context, form_data).await
        }
    }
}
