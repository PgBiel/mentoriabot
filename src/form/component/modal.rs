use crate::common::ApplicationContext;
use crate::error::{FormError, Result};
use crate::form::InteractionModalForm;
use async_trait::async_trait;
use poise::Modal as PoiseModal;

#[async_trait]
pub trait ModalFormComponent: Sync {
    type Modal: poise::Modal + Send + Sync;
    type Form: InteractionModalForm<Modal = Self::Modal, ModalComponent = Self> + Send + Sync;

    async fn on_response(&self, modal: Self::Modal, form_data: Self::Form) -> Result<Self::Form> {
        Ok(form_data.set_modal(modal))
    }

    async fn run(
        &self,
        context: ApplicationContext<'_>,
        form_data: Self::Form,
    ) -> Result<Self::Form> {
        let response: Option<Self::Modal> = Self::Modal::execute(context).await?;

        match response {
            Some(modal) => self.on_response(modal, form_data).await,
            None => Err(FormError::NoResponse.into()),
        }
    }
}
