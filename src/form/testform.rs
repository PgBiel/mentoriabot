use super::{FormComponent, MessageFormComponent, ModalFormComponent, InteractionModalForm};
use crate::common::ApplicationContext;
use crate::error::{FormError, Result};
use async_trait::async_trait;
use poise::serenity_prelude as serenity;
use std::str::FromStr;
use std::sync::Arc;
use strum_macros::{self, EnumString};

#[derive(Debug, Copy, Clone, strum_macros::Display, EnumString)]
pub enum FirstSelectionData {
    Ice,
    Fire,
    Water,
}

#[derive(Debug, Default, Copy, Clone)]
struct TestFormFirstSelection;

#[async_trait]
impl MessageFormComponent for TestFormFirstSelection {
    type Form = TestForm;

    async fn send_component(
        &self,
        context: ApplicationContext<'_>,
        custom_id: &str,
        form_data: TestForm,
    ) -> Result<TestForm> {
        context
            .send(|f| {
                f.content("Choose one:")
                    .components(|f| {
                        f.create_action_row(|f| {
                            f.create_select_menu(|f| {
                                f.custom_id(&custom_id)
                                    .placeholder("Choose please")
                                    .options(|f| {
                                        f.create_option(|f| {
                                            f.label("Whoops")
                                                .value(FirstSelectionData::Fire)
                                                .description("Whoopsie")
                                        })
                                        .create_option(
                                            |f| {
                                                f.label("Second")
                                                    .value(FirstSelectionData::Ice)
                                                    .description("Epic")
                                            },
                                        )
                                    })
                            })
                        })
                    })
                    .ephemeral(true)
            })
            .await?;
        Ok(form_data)
    }

    async fn on_response(
        &self,
        context: ApplicationContext<'_>,
        interaction: Arc<serenity::MessageComponentInteraction>,
        mut form_data: TestForm,
    ) -> Result<TestForm> {
        let values = &interaction.data.values;

        if !values.is_empty() {
            form_data.first_selection = Some(FirstSelectionData::from_str(values[0].as_ref())?);
            interaction
                .edit_original_interaction_response(context.serenity_context, |f| {
                    f.content(format!("Thanks for {}", form_data.first_selection.unwrap()))
                })
                .await?;

            Ok(form_data)
        } else {
            Err(FormError::InvalidUserResponse.into())
        }
    }
}

#[derive(Debug, Default, Clone, poise::Modal)]
#[name = "Random modal"]
pub struct TestFormModal {
    #[name = "Name"]
    #[placeholder = "Joseph"]
    #[min_length = 3]
    #[max_length = 20]
    name: String,

    #[name = "age"]
    #[max_length = 3]
    age: Option<String>,

    #[name = "More"]
    #[max_length = 500]
    #[paragraph]
    more: Option<String>,
}

#[derive(Debug, Default, Clone, Copy)]
struct TestFormModalComponent;

#[async_trait]
impl ModalFormComponent for TestFormModalComponent {
    type Modal = TestFormModal;
    type Form = TestForm;

    async fn on_response(
        &self,
        modal: Self::Modal,
        mut form_data: Self::Form,
    ) -> Result<Self::Form> {
        form_data.modal_answers = Some(modal);
        Ok(form_data)
    }
}

#[derive(Debug, Default, Clone)]
pub struct TestForm {
    first_selection: Option<FirstSelectionData>,
    modal_answers: Option<TestFormModal>,
}

impl InteractionModalForm for TestForm {
    type Modal = TestFormModal;

    fn modal() -> super::ModalFormComponentBox<TestFormModal, Self>
    {
        Box::new(TestFormModalComponent::default())
    }

    fn components() -> Vec<FormComponent<Self>> {
        vec![
            FormComponent::Message(Box::new(TestFormFirstSelection::default())),
            FormComponent::Message(Box::new(TestFormFirstSelection::default())),
        ]
    }
}
