use std::str::FromStr;
use async_trait::async_trait;
use crate::error::{Result, FormError};
use super::{InteractionForm, InteractionFormComponent, MessageFormComponent};
use strum_macros::{self, EnumString};
use crate::interaction::wait_for_message_interaction;
use crate::util::generate_custom_id;
use crate::common::ApplicationContext;

#[derive(Debug, Copy, Clone, strum_macros::Display, EnumString)]
pub enum FirstSelectionData {
    Ice,
    Fire,
    Water
}

#[derive(Debug, Default, Copy, Clone)]
pub struct TestFormData {
    pub first_selection: Option<FirstSelectionData>,
}

#[derive(Debug, Default, Copy, Clone)]
struct TestFormFirstSelection;

#[async_trait]
impl InteractionFormComponent for TestFormFirstSelection {
    type FormResultData = TestFormData;

    async fn run(&self, context: ApplicationContext<'_>, mut form_data: TestFormData)
                 -> Result<TestFormData> {
        // TODO: Extract generic FormComponent for Button, SelectMenu, Modal
        let custom_id = generate_custom_id();

        context.send(|f| f
            .content("Choose one:")
            .components(|f| f
                .create_action_row(|f| f.create_select_menu(|f| f
                    .custom_id(&custom_id)
                    .options(|f| f
                        .create_option(|f| f
                            .label("Whoops")
                            .value(FirstSelectionData::Fire)
                            .description("Whoopsie"))
                        .create_option(|f| f
                            .label("Second")
                            .value(FirstSelectionData::Ice)
                            .description("Epic")
                            .default_selection(true))))))
            .ephemeral(true))
        .await?;

        let response = wait_for_message_interaction(context, custom_id)
                    .await?;

        match response {
            Some(interaction) => {
                let values = &interaction.data.values;
                if !values.is_empty() {
                    form_data.first_selection = Some(FirstSelectionData::from_str(values[0].as_ref())?);

                    // strip components from original message
                    interaction.edit_original_interaction_response(context.serenity_context,
                                                                    |f| f.components(|f| f)).await?;
                    Ok(form_data)
                } else {
                    interaction.edit_original_interaction_response(context.serenity_context,
                                                                    |f| f.components(|f| f)).await?;
                    Err(FormError::InvalidUserResponse.into())
                }
            },
            None => Err(FormError::NoResponse.into())
        }
    }
}

pub type TestForm = InteractionForm<TestFormData>;

pub fn create_test_form() -> TestForm {
    let select_menu_comp = Box::new(MessageFormComponent::<TestFormData>::builder())

    TestForm::new(vec![Box::new(TestFormFirstSelection::default()), Box::new(TestFormFirstSelection::default())])
}
