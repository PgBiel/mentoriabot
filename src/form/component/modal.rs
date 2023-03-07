use async_trait::async_trait;
use poise::Modal as PoiseModal;

use crate::{
    common::ApplicationContext,
    error::{FormError, Result},
};

#[async_trait]
pub trait ModalFormComponent<Data: Send + Sync = ()>: Send + Sync {
    type Modal: poise::Modal + Send + Sync;

    async fn on_response(modal: Self::Modal, data: &mut Data) -> Result<Box<Self>>;

    async fn run(context: ApplicationContext<'_>, data: &mut Data) -> Result<Box<Self>> {
        let response: Option<Self::Modal> = Self::Modal::execute(context).await?;

        match response {
            Some(modal) => Self::on_response(modal, data).await,
            None => Err(FormError::NoResponse.into()),
        }
    }
}

#[async_trait]
impl<T, D> ModalFormComponent<D> for T
where
    T: poise::Modal + Send + Sync,
    D: Send + Sync,
{
    type Modal = Self;

    async fn on_response(modal: Self, _: &mut D) -> Result<Box<Self>> {
        Ok(Box::new(modal))
    }
}
