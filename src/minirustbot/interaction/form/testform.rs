use std::str::FromStr;
use async_trait::async_trait;
use poise::serenity_prelude as serenity;
use crate::minirustbot::error::{Result, FormError};
use super::{InteractionForm, InteractionFormComponent};
use strum_macros::{self, EnumString};
use crate::minirustbot::interaction::wait_for_message_interaction;
use crate::minirustbot::util::generate_custom_id;
use crate::common::ApplicationContext;

#[derive(Debug, Copy, Clone, strum_macros::Display, EnumString)]
pub enum FirstSelectionData {
    Ice,
    Fire,
    Water
}

// impl ToString for FirstSelectionData {
//     fn to_string(&self) -> String {
//         String::from(match self {
//             Self::Ice => "Ice",
//             Self::Fire => "Fire",
//             Self::Water => "Water"
//         })
//     }
// }
//
// impl FromStr for FirstSelectionData {
//     type Err = FormError::InvalidUserResponse;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "Ice" => Ok(Self::Ice),
//             "Fire" => Ok(Self::Fire),
//             "Water" => Ok(Self::Water),
//             _ => Err(Self::Err)
//         }
//     }
// }

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
//        let options = vec![
//
//                SelectMenuOption {
//                label: "Whoops".to_string(),
//                    value: FirstSelectionData::Fire.to_string(),
//                    description: Some("Whoopsie".to_string()),
//                    emoji: None,
//                    default: false,
//                },
//                SelectMenuOption {
//                label: "Abad".to_string(),
//                    value: FirstSelectionData::Ice.to_string(),
//                    description: Some("Whoopsgie".to_string()),
//                    emoji: None,
//                    default: true,
//                }
//        ];

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
    TestForm::new(vec![Box::new(TestFormFirstSelection::default()), Box::new(TestFormFirstSelection::default())])
}
