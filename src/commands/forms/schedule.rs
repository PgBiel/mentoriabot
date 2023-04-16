use std::sync::Arc;

use async_trait::async_trait;
use minirustbot_forms::{
    Buildable, CustomId, FormError, ReplySpec, SelectMenuOptionSpec, SelectMenuSpec, SelectValue,
};
use poise::{
    serenity_prelude as serenity, serenity_prelude::MessageComponentInteraction, ApplicationContext,
};

use crate::{
    common::Data,
    error::Error,
    forms::{error::Result as FormResult, GenerateReply, InteractionForm, MessageFormComponent},
    model::{Availability, DiscordId, Teacher, Weekday},
    util::tr,
};

#[derive(Debug, InteractionForm)]
#[ctx_data = "Data"]
#[ctx_error = "Error"]
#[data = "ScheduleFormData"]
pub(crate) struct ScheduleForm {
    #[component]
    pub(crate) select_time: SelectTimeComponent,

    #[component]
    pub(crate) select_mentor: SelectMentorComponent,
}

#[derive(Debug, Clone)]
pub(crate) struct SelectTimeComponent {
    pub(crate) selected_availability: Availability,
}

#[derive(Debug)]
pub(crate) struct SelectMentorComponent {
    pub(crate) selected_mentor: Teacher,
}

/// Stores data while the ScheduleForm is still being constructed.
#[derive(Default)]
struct ScheduleFormData {
    selected_availability: Option<Availability>,
}

// impls
#[async_trait]
impl GenerateReply<Data, Error, ScheduleFormData> for SelectTimeComponent {
    type ReplyBuilder = ReplySpec;

    async fn create_reply<'a, 'b>(
        context: ApplicationContext<'_, Data, Error>,
        data: &ScheduleFormData,
    ) -> FormResult<Self::ReplyBuilder> {
        Ok(ReplySpec {
            content: tr!(
                "commands.schedule.please_select_time",
                ctx = context,
                total_mentor_amount = "6"
            ),
            ephemeral: true,
            ..Default::default()
        })
    }
}

#[async_trait]
impl MessageFormComponent<Data, Error, ScheduleFormData> for SelectTimeComponent {
    async fn send_component(
        context: ApplicationContext<'_, Data, Error>,
        data: &mut ScheduleFormData,
    ) -> FormResult<Vec<CustomId>> {
        let custom_id = CustomId::generate();
        let select_menu = SelectMenuSpec {
            custom_id: custom_id.clone(),
            options: vec![
                SelectMenuOptionSpec {
                    label: "12:00".to_string(),
                    value_key: SelectValue::from("1"),
                    description: Some(tr!("commands.schedule.one_mentor", ctx = context)),
                    emoji: None,
                    is_default: false,
                },
                SelectMenuOptionSpec {
                    label: "13:00".to_string(),
                    value_key: SelectValue::from("2"),
                    description: Some(tr!(
                        "commands.schedule.n_mentors",
                        ctx = context,
                        amount = "5"
                    )),
                    emoji: None,
                    is_default: false,
                },
            ],
            ..Default::default()
        };
        let reply =
            <Self as GenerateReply<Data, Error, ScheduleFormData>>::create_reply(context, data)
                .await?;

        context
            .send(|b| {
                reply.on_build(b.components(|b| {
                    b.create_action_row(|b| b.create_select_menu(|b| select_menu.on_build(b)))
                }))
            })
            .await?;

        Ok(vec![custom_id])
    }

    async fn on_response(
        context: ApplicationContext<'_, Data, Error>,
        interaction: Arc<MessageComponentInteraction>,
        data: &mut ScheduleFormData,
    ) -> FormResult<Option<Box<Self>>> {
        let selected = interaction.data.values.first();

        let Some(selected) = selected else {
            return Err(FormError::InvalidUserResponse);
        };

        let Ok(selected) = selected.parse::<i64>() else {
            return Err(FormError::InvalidUserResponse);
        };

        let availability = Availability {
            id: selected,
            teacher_id: DiscordId(123),
            weekday: Weekday::MONDAY,
            time_start: chrono::NaiveTime::from_hms_opt(23, 0, 0).unwrap(),
            time_end: chrono::NaiveTime::from_hms_opt(23, 0, 0).unwrap(),
        };
        data.selected_availability = Some(availability.clone());

        Ok(Some(Box::new(SelectTimeComponent {
            selected_availability: availability,
        })))
    }
}

#[async_trait]
impl GenerateReply<Data, Error, ScheduleFormData> for SelectMentorComponent {
    type ReplyBuilder = ReplySpec;

    async fn create_reply<'a, 'b>(
        context: ApplicationContext<'_, Data, Error>,
        data: &ScheduleFormData,
    ) -> FormResult<Self::ReplyBuilder> {
        let time = data
            .selected_availability
            .as_ref()
            .ok_or(FormError::InvalidUserResponse)?
            .time_start
            .format("%H:%M:%S");

        Ok(ReplySpec {
            content: tr!(
                "commands.schedule.please_select_mentor",
                ctx = context,
                time = time.to_string()
            ),
            ephemeral: true,
            ..Default::default()
        })
    }
}

#[async_trait]
impl MessageFormComponent<Data, Error, ScheduleFormData> for SelectMentorComponent {
    async fn send_component(
        context: ApplicationContext<'_, Data, Error>,
        data: &mut ScheduleFormData,
    ) -> FormResult<Vec<CustomId>> {
        // let avail_id = data.selected_availability;
        let custom_id = CustomId::generate();
        let select_menu = SelectMenuSpec {
            custom_id: custom_id.clone(),
            options: vec![
                SelectMenuOptionSpec {
                    label: "João Silva".to_string(),
                    value_key: SelectValue::from("1"),
                    description: Some("Cálculo, Programação, etc.".to_string()),
                    ..Default::default()
                },
                SelectMenuOptionSpec {
                    label: "Parmênides Ferreira".to_string(),
                    value_key: SelectValue::from("2"),
                    description: Some("Análise de Algoritmos ou qualquer outra coisa".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let reply =
            <Self as GenerateReply<Data, Error, ScheduleFormData>>::create_reply(context, data)
                .await?;

        context
            .send(|b| {
                reply.on_build(b.components(|b| {
                    b.create_action_row(|b| b.create_select_menu(|b| select_menu.on_build(b)))
                }))
            })
            .await?;

        Ok(vec![custom_id])
    }

    async fn on_response(
        context: ApplicationContext<'_, Data, Error>,
        interaction: Arc<MessageComponentInteraction>,
        data: &mut ScheduleFormData,
    ) -> FormResult<Option<Box<Self>>> {
        let mentor_id = interaction
            .data
            .values
            .first()
            .map(|v| v.parse::<DiscordId>())
            .ok_or(FormError::InvalidUserResponse)?
            .map_err(|_| FormError::InvalidUserResponse)?;

        Ok(Some(Box::new(Self {
            selected_mentor: Teacher {
                user_id: mentor_id,
                email: Some("test@gmail.com".to_string()),
                specialty: "Test".to_string(),
            },
        })))
    }
}
