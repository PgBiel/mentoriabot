use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use chrono::Timelike;
use minirustbot_forms::FormState;
use poise::serenity_prelude::MessageComponentInteraction;

use crate::{
    common::{ApplicationContext, ContextualResult, Data},
    error::Error,
    forms::{
        error::Result as FormResult, Buildable, CustomId, FormError, GenerateReply,
        InteractionForm, MessageFormComponent, SelectMenuOptionSpec, SelectMenuSpec, SelectValue,
    },
    model::{Availability, Teacher, User},
    util,
    util::{time::hour_minute_display, tr, BRAZIL_TIMEZONE},
};

#[derive(Debug, InteractionForm)]
#[form_data(data(ScheduleFormData), ctx(Data, Error))]
pub(crate) struct ScheduleForm {
    #[component]
    pub(crate) select_time: SelectTimeComponent,

    #[component]
    pub(crate) select_mentor: SelectMentorComponent,
}

#[derive(Debug, Clone, GenerateReply)]
#[form_data(data(ScheduleFormData), ctx(Data, Error))]
#[reply(content = (
    select_time_reply_content(context, data).await?
), ephemeral)]
pub(crate) struct SelectTimeComponent;

#[derive(Debug, Clone, GenerateReply)]
#[form_data(data(ScheduleFormData), ctx(Data, Error))]
#[reply(content = (
    select_mentor_reply_content(context, data).await?
), ephemeral)]
pub(crate) struct SelectMentorComponent {
    pub(crate) selected_availability: Availability,
    pub(crate) selected_mentor_user: User,
    pub(crate) selected_mentor: Teacher,
}

/// Stores data while the ScheduleForm is still being constructed.
#[derive(Debug, Default)]
pub(crate) struct ScheduleFormData {
    availabilities: Vec<Availability>,
    teacher_tuples: Vec<(Teacher, User, Availability)>,
}

// impls
async fn select_time_reply_content(
    context: ApplicationContext<'_>,
    data: &FormState<ScheduleFormData>,
) -> FormResult<String> {
    let mut seen_mentors = Vec::new();
    let mentors = data
        .availabilities
        .iter()
        .map(|avail| avail.teacher_id)
        .fold(0u32, |acc, teacher_id| {
            if !seen_mentors.contains(&teacher_id) {
                seen_mentors.push(teacher_id);
                acc.checked_add(1).unwrap()
            } else {
                acc
            }
        });

    Ok(if mentors == 1 {
        tr!("commands.schedule.please_select_time_one", ctx = context)
    } else {
        tr!(
            "commands.schedule.please_select_time_n",
            ctx = context,
            total_mentor_amount = mentors
        )
    })
}

async fn select_mentor_reply_content(
    context: ApplicationContext<'_>,
    data: &FormState<ScheduleFormData>,
) -> FormResult<String> {
    let time = data
        .availabilities
        .first()
        .ok_or(FormError::InvalidUserResponse)?
        .time_start;

    Ok(tr!(
        "commands.schedule.please_select_mentor",
        ctx = context,
        time = hour_minute_display(time),
    ))
}

#[async_trait]
impl MessageFormComponent<Data, Error, ScheduleFormData> for SelectTimeComponent {
    async fn send_component(
        context: ApplicationContext<'_>,
        data: &mut FormState<ScheduleFormData>,
    ) -> ContextualResult<Vec<CustomId>> {
        let availabilities = context
            .data
            .db
            .availability_repository()
            .find_nontaken_at_date(chrono::Utc::now().with_timezone(&*BRAZIL_TIMEZONE))
            .await?;

        // No mentors have time available for sessions today
        if availabilities.is_empty() {
            context
                .send(|b| b.content(tr!("commands.schedule.no_mentors_available", ctx = context)))
                .await?;
            return Err(FormError::Cancelled.into());
        }

        let mut unique_availability_times: HashMap<chrono::NaiveTime, i32> = HashMap::new();
        for avail in &availabilities {
            let time_start = avail.time_start.with_second(0).unwrap();
            if let Some(amount) = unique_availability_times.get_mut(&time_start) {
                *amount = amount
                    .checked_add(1i32)
                    .ok_or_else(|| Error::Other("too many mentors"))?;
            } else {
                unique_availability_times.insert(time_start, 1);
            }
        }
        let mut unique_availability_times: Vec<(chrono::NaiveTime, i32)> =
            unique_availability_times.into_iter().collect();

        // sort by increasing times
        unique_availability_times.sort();

        let custom_id = CustomId::generate();
        let select_menu = SelectMenuSpec {
            custom_id: custom_id.clone(),
            options: unique_availability_times
                .into_iter()
                .map(|(time, amount_mentors)| {
                    let time_string = hour_minute_display(time).to_string();
                    SelectMenuOptionSpec {
                        label: time_string.clone(),
                        value_key: SelectValue::from(time_string),
                        description: Some(if amount_mentors == 1 {
                            tr!("commands.schedule.one_mentor", ctx = context)
                        } else {
                            tr!(
                                "commands.schedule.n_mentors",
                                ctx = context,
                                amount = amount_mentors
                            )
                        }),
                        ..Default::default()
                    }
                })
                .collect(),
            ..Default::default()
        };

        data.availabilities = availabilities;

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
        _context: ApplicationContext<'_>,
        interaction: Arc<MessageComponentInteraction>,
        data: &mut FormState<ScheduleFormData>,
    ) -> ContextualResult<Option<Box<Self>>> {
        let selected = interaction.data.values.first();

        let Some(selected) = selected else {
            return Err(FormError::InvalidUserResponse.into());
        };

        let Ok(selected) = util::time::parse_hour_minute(selected) else {
            return Err(FormError::InvalidUserResponse.into());
        };

        let availabilities = std::mem::take(&mut data.availabilities);
        let selected_availabilities = availabilities
            .into_iter()
            .filter(|avail| {
                // ignore seconds
                avail.time_start.with_second(0).unwrap() == selected.with_second(0).unwrap()
            })
            .collect::<Vec<_>>();

        data.availabilities = selected_availabilities;

        Ok(Some(Box::new(SelectTimeComponent)))
    }
}

#[async_trait]
impl MessageFormComponent<Data, Error, ScheduleFormData> for SelectMentorComponent {
    async fn send_component(
        context: ApplicationContext<'_>,
        data: &mut FormState<ScheduleFormData>,
    ) -> ContextualResult<Vec<CustomId>> {
        let reply =
            <Self as GenerateReply<Data, Error, ScheduleFormData>>::create_reply(context, data)
                .await?;

        let availabilities = std::mem::take(&mut data.availabilities);
        let mut teachers = context
            .data
            .db
            .teacher_repository()
            .find_by_availabilities(&availabilities)
            .await?;

        if teachers.is_empty() {
            return Err(FormError::InvalidUserResponse.into());
        }

        teachers.sort_by_cached_key(|(_, user, avail)| (avail.time_start, user.name.clone()));

        let custom_id = CustomId::generate();
        let select_menu = SelectMenuSpec {
            custom_id: custom_id.clone(),
            options: teachers
                .iter()
                .map(|(teacher, user, avail)| SelectMenuOptionSpec {
                    label: user.name.clone(),
                    value_key: SelectValue::from(avail.id.to_string()),
                    description: Some(teacher.specialty.to_string()),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        };

        // place the teacher tuples in data so we can use later
        std::mem::swap(&mut data.teacher_tuples, &mut teachers);

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
        _: ApplicationContext<'_>,
        interaction: Arc<MessageComponentInteraction>,
        data: &mut FormState<ScheduleFormData>,
    ) -> ContextualResult<Option<Box<Self>>> {
        let teacher_tuples = std::mem::take(&mut data.teacher_tuples);

        let (teacher, user, availability) = interaction
            .data
            .values
            .first()
            .and_then(|v| v.parse::<i64>().ok())
            .and_then(|avail_id| {
                teacher_tuples
                    .into_iter()
                    .find(|(_, _, avail)| avail.id == avail_id)
            })
            .ok_or(FormError::InvalidUserResponse)?;

        Ok(Some(Box::new(Self {
            selected_availability: availability,
            selected_mentor_user: user,
            selected_mentor: teacher,
        })))
    }
}
