mod message;
mod modal;

pub use message::{MessageFormComponent, MessageFormComponentBuilder};
pub use modal::{ModalFormComponent, ModalFormComponentBuilder};

use async_trait::async_trait;
use crate::common::ApplicationContext;
use crate::error::Result;

/// Trait for interaction senders/receivers which serve as components
/// of a form.
#[async_trait]
pub trait InteractionFormComponent {
    type FormResultData;

    /// Sends the interaction and awaits for its response, then updates
    /// the existing FormResultData object based on the response.
    async fn run(&self, context: ApplicationContext<'_>, form_data: Self::FormResultData)
        -> Result<Self::FormResultData>;
}
