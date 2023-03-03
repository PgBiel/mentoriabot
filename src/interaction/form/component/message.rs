use std::sync::Arc;
use poise::serenity_prelude as serenity;
use async_trait::async_trait;
use super::InteractionFormComponent;
use crate::error::{Result, FormError};
use crate::common::ApplicationContext;
use crate::util::generate_custom_id;
use crate::interaction;


pub type SendComponentFun<'a, FormResultData> =
    Box<dyn Fn(String, FormResultData) -> poise::BoxFuture<'a, Result<FormResultData>> + Send + Sync>;

pub type OnResponseFun<'a, FormResultData> =
    Box<dyn Fn(Arc<serenity::MessageComponentInteraction>, FormResultData) -> poise::BoxFuture<'a, Result<FormResultData>> + Send + Sync>;



/// A form component for either a Button or a Select Menu;
/// that is, one that can be displayed on a message.
pub struct MessageFormComponent<'a, FormResultData: Default> {
    send_component: SendComponentFun<'a, FormResultData>,
    on_response: OnResponseFun<'a, FormResultData>
}

impl<'a, F: Default> MessageFormComponent<'a, F> {
    pub fn builder() -> MessageFormComponentBuilder<'a, F> {
        Default::default()
    }
}

#[async_trait]
impl<'a, FormResultData> InteractionFormComponent for MessageFormComponent<'a, FormResultData>
where
    for<'async_trait> FormResultData: Default + Send + Sync + 'async_trait,
{

    type FormResultData = FormResultData;

    async fn run(&self, context: ApplicationContext<'_>, form_data: FormResultData) -> Result<FormResultData>
    {
        let custom_id = generate_custom_id();

        let form_data = (self.send_component)(custom_id.clone(), form_data).await?;

        let response = interaction::wait_for_message_interaction(context, custom_id)
                    .await?;

        match response {
            Some(interaction) => {
                
                // strip original components (avoid repeated interactions)
                interaction.edit_original_interaction_response(context.serenity_context,
                    |f| f.components(|f| f)).await?;
                
                (self.on_response)(interaction, form_data).await
            },
            None => Err(FormError::NoResponse.into())
        }
    }
}

/// Builder for MessageFormComponent for a certain FormResultData object.
/// Panics on build if not all fields (prepare_component_reply and on_response) were specified.
#[derive(Default)]
pub struct MessageFormComponentBuilder<'a, FormResultData: Default> {
    prepare_component_reply: Option<SendComponentFun<'a, FormResultData>>,
    on_response: Option<OnResponseFun<'a, FormResultData>>
}

impl<'a, FormResultData: Default> MessageFormComponentBuilder<'a, FormResultData> {

    /// Function that takes a `poise::CreateReply` object and adds the necessary
    /// content and components to it, preparing the message to be sent.
    pub fn prepare_component_reply(mut self, fun: SendComponentFun<'a, FormResultData>) -> Self {
        self.prepare_component_reply = Some(fun);
        self
    }

    /// Function to be run when a response is received. Takes a `serenity::MessageComponentInteraction`
    /// and the `FormResultData` object to be modified.
    pub fn on_response(mut self, fun: OnResponseFun<'a, FormResultData>) -> Self {
        self.on_response = Some(fun);
        self
    }

    pub fn build(self) -> MessageFormComponent<'a, FormResultData> {
        if let Some(send_component) = self.prepare_component_reply {
            if let Some(on_response) = self.on_response {
                return MessageFormComponent {
                    send_component,
                    on_response
                };
            }
        }
        panic!("Missing builder parameters.");
    }
}
