use async_trait::async_trait;
use poise::{ApplicationContext, Modal as PoiseModal};

use crate::error::{FormError, Result};

/// A Modal Component. This is blanket-implemented for all [`poise::Modal`] types,
/// but you may wish to override its behavior (by default, the `Modal` is itself).
///
/// [`poise::Modal`]: ::poise::Modal
#[async_trait]
pub trait ModalFormComponent<
    ContextData: Send + Sync,
    ContextError: Send + Sync,
    FormData: Send + Sync = (),
>: Send + Sync
{
    type Modal: poise::Modal + Send + Sync;

    async fn on_response(modal: Self::Modal, data: &mut FormData) -> Result<Box<Self>>;

    async fn run(
        context: ApplicationContext<'_, ContextData, ContextError>,
        data: &mut FormData,
    ) -> Result<Box<Self>> {
        let response: Option<Self::Modal> = Self::Modal::execute(context).await?;

        match response {
            Some(modal) => Self::on_response(modal, data).await,
            None => Err(FormError::NoResponse),
        }
    }
}

#[async_trait]
impl<T, CD, CE, D> ModalFormComponent<CD, CE, D> for T
where
    T: poise::Modal + Send + Sync,
    CD: Send + Sync,
    CE: Send + Sync,
    D: Send + Sync,
{
    type Modal = Self;

    async fn on_response(modal: Self, _: &mut D) -> Result<Box<Self>> {
        Ok(Box::new(modal))
    }
}
