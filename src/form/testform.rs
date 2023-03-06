use super::MessageFormComponent;
use crate::common::ApplicationContext;
use crate::error::{FormError, Result, Error};
use crate::interaction;
use crate::macros::{InteractionForm, ButtonComponent};
use crate::util::generate_custom_id;
use async_trait::async_trait;
use poise::serenity_prelude as serenity;
use std::str::FromStr;
use std::sync::Arc;
use strum_macros::{self, EnumString};

#[derive(Debug, Copy, Clone, strum_macros::Display, EnumString)]
pub enum TestFormFirstSelection {
    Ice,
    Fire,
    Water,
}

#[async_trait]
impl MessageFormComponent for TestFormFirstSelection {
    async fn send_component_and_wait(
        context: ApplicationContext<'_>,
        _: &mut (),
    ) -> Result<Option<Arc<serenity::MessageComponentInteraction>>> {
        let custom_id = generate_custom_id();
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
                                                .value(Self::Fire)
                                                .description("Whoopsie")
                                        })
                                        .create_option(
                                            |f| {
                                                f.label("Second")
                                                    .value(Self::Ice)
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

        interaction::wait_for_message_interaction(context, &custom_id).await.map_err(Error::Serenity)
    }

    async fn on_response(
        _context: ApplicationContext<'_>,
        interaction: Arc<serenity::MessageComponentInteraction>,
        _: &mut (),
    ) -> Result<Box<Self>> {
        let values = &interaction.data.values;

        if !values.is_empty() {
            let first_selection = Self::from_str(values[0].as_ref())?;

            Ok(Box::new(first_selection))
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

#[derive(ButtonComponent, Clone, Debug)]
#[message_content = "bruh"]
#[label = "Click me!"]
#[danger]
#[message_ephemeral]
pub struct Button (#[interaction] serenity::MessageComponentInteraction);

#[derive(InteractionForm, Debug, Clone)]
#[data = "()"]
pub struct TestForm {
    #[modal]
    modal_answers: TestFormModal,

    #[component]
    first_sel_comp: TestFormFirstSelection,

    #[component]
    button: Button,
}
