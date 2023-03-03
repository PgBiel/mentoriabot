use async_trait::async_trait;
use super::InteractionFormComponent;
use crate::error::{Result, FormError};
use crate::common::ApplicationContext;

pub type OnResponseFun<'a, Modal: poise::Modal, FormResultData: Default> =
    Box<dyn Fn(Modal, FormResultData) -> poise::BoxFuture<'a, Result<FormResultData>> + Send + Sync>;

/// A form component for either a Button or a Select Menu;
/// that is, one that can be displayed on a message.
pub struct ModalFormComponent<'a, Modal: poise::Modal, FormResultData: Default> {
    on_response: OnResponseFun<'a, Modal, FormResultData>
}

impl<'a, M: poise::Modal, F: Default> ModalFormComponent<'a, M, F> {
    pub fn builder() -> ModalFormComponentBuilder<'a, M, F> {
        Default::default()
    }
}

#[async_trait]
impl<'a, Modal, FormResultData> InteractionFormComponent for ModalFormComponent<'a, Modal, FormResultData>
where
    Modal: poise::Modal + Send + Sync,
    for<'async_trait> FormResultData: Default + Send + Sync + 'async_trait,
{

    type FormResultData = FormResultData;

    async fn run(&self, context: ApplicationContext<'_>, form_data: FormResultData) -> Result<FormResultData>
    {
        let response = Modal::execute(context).await?;

        match response {
            Some(modal) => {
                (self.on_response)(modal, form_data).await
            },
            None => Err(FormError::NoResponse.into())
        }
    }
}

/// Builder for MessageFormComponent for a certain FormResultData object.
/// Panics on build if not all fields (prepare_component_reply and on_response) were specified.
pub struct ModalFormComponentBuilder<'a, Modal, FormResultData>
where
    Modal: poise::Modal,
    FormResultData: Default
{
    on_response: Option<OnResponseFun<'a, Modal, FormResultData>>
}

impl<'a, Modal, FormResultData> Default for ModalFormComponentBuilder<'a, Modal, FormResultData>
where
    Modal: poise::Modal,
    FormResultData: Default
{
    fn default() -> Self {
        ModalFormComponentBuilder { on_response: None }
    }
}

impl<'a, Modal, FormResultData> ModalFormComponentBuilder<'a, Modal, FormResultData>
where
    Modal: poise::Modal,
    FormResultData: Default
{

    /// Function to be run when a response is received. Takes a `serenity::MessageComponentInteraction`
    /// and the `FormResultData` object to be modified.
    pub fn on_response(mut self, fun: OnResponseFun<'a, Modal, FormResultData>) -> Self {
        self.on_response = Some(fun);
        self
    }

    pub fn build(self) -> ModalFormComponent<'a, Modal, FormResultData> {
        if let Some(on_response) = self.on_response {
            return ModalFormComponent {
                on_response
            };
        }
        panic!("Missing builder parameters.");
    }
}
