use std::sync::Arc;

use async_trait::async_trait;
use chrono::Timelike;
use poise::serenity_prelude::MessageComponentInteraction;

use crate::{
    common::{ApplicationContext, ContextualResult, Data},
    forms::{
        error::Result as FormResult, Buildable, CustomId, FormError, FormState, GenerateReply,
        InteractionForm, MessageFormComponent, SelectMenuOptionSpec, SelectMenuSpec, SelectValue,
    },
    lib::{
        error::Error,
        model::{Availability, Teacher, Weekday},
        util::{
            self,
            time::{brazil_now, hour_minute_display},
            tr,
        },
    },
};

#[derive(Debug, InteractionForm)]
#[form_data(data(ScheduleFormData), ctx(Data, Error))]
pub(crate) struct ScheduleForm {
    #[from_data_field = "form_start_datetime"]
    pub(crate) form_start_datetime: Option<chrono::DateTime<chrono::FixedOffset>>,

    #[component]
    #[allow(dead_code)]
    pub(crate) select_weekday: SelectWeekdayComponent,

    #[component]
    #[allow(dead_code)]
    pub(crate) select_time: SelectTimeComponent,

    #[component]
    #[allow(dead_code)]
    pub(crate) select_mentor: SelectMentorComponent,
}

#[derive(Debug, Clone, GenerateReply)]
#[form_data(data(ScheduleFormData), ctx(Data, Error))]
#[reply(content = (
    select_weekday_reply_content(context, data).await?
), ephemeral)]
pub(crate) struct SelectWeekdayComponent;

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
    pub(crate) selected_mentor: Teacher,
}

/// Stores data while the ScheduleForm is still being constructed.
#[derive(Debug, Default)]
pub(crate) struct ScheduleFormData {
    form_start_datetime: Option<chrono::DateTime<chrono::FixedOffset>>,
    availabilities: Vec<Availability>,
    teacher_tuples: Vec<(Teacher, Availability)>,
}

// impls
async fn select_weekday_reply_content(
    context: ApplicationContext<'_>,
    data: &FormState<ScheduleFormData>,
) -> FormResult<String> {
    let avail_count = data.availabilities.len();
    Ok(if avail_count == 1 {
        tr!("commands.schedule.please_select_weekday_one", ctx = context)
    } else {
        tr!(
            "commands.schedule.please_select_weekday_n",
            ctx = context,
            total_sessions_amount = avail_count
        )
    })
}

async fn select_time_reply_content(
    context: ApplicationContext<'_>,
    data: &FormState<ScheduleFormData>,
) -> ContextualResult<String> {
    let selected_weekday = data.availabilities.first().map(|avail| avail.weekday);
    let (selected_weekday, selected_date) = selected_weekday
        .zip(data.form_start_datetime.as_ref())
        .map(|(weekday, initial_date)| (weekday, weekday.next_day_with_this_weekday(initial_date)))
        .ok_or_else(|| Error::Other("Couldn't get the selected date at 'schedule' form."))?;
    let date_string = util::time::day_month_display(&selected_date.date_naive());
    let weekday_string = selected_weekday
        .to_locale_shorthand_string(util::locale::get_defaulted_app_ctx_locale(context));

    let unique_mentors =
        util::iter::count_unique(data.availabilities.iter().map(|avail| avail.teacher_id));

    Ok(if unique_mentors == 1 {
        tr!(
            "commands.schedule.please_select_time_one",
            ctx = context,
            day = date_string,
            weekday = weekday_string
        )
    } else {
        tr!(
            "commands.schedule.please_select_time_n",
            ctx = context,
            total_mentor_amount = unique_mentors,
            day = date_string,
            weekday = weekday_string
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
impl MessageFormComponent<Data, Error, ScheduleFormData> for SelectWeekdayComponent {
    async fn send_component(
        context: ApplicationContext<'_>,
        data: &mut FormState<ScheduleFormData>,
    ) -> ContextualResult<Vec<CustomId>> {
        let now = brazil_now();
        let availabilities = context
            .data
            .db
            .availability_repository()
            .find_nontaken_within_a_week_of_date(now)
            .await?;

        // No mentors have time available for sessions in the next week
        if availabilities.is_empty() {
            context
                .send(|b| {
                    b.content(tr!(
                        "commands.schedule.no_mentors_available_week",
                        ctx = context
                    ))
                })
                .await?;
            return Err(FormError::Cancelled.into());
        }

        let mut unique_availability_weekdays: Vec<(Weekday, usize)> =
            util::iter::group_by_count(availabilities.iter(), |avail: &Availability| avail.weekday)
                .into_iter()
                .collect();

        // sort by increasing times
        unique_availability_weekdays.sort();

        let custom_id = CustomId::generate();
        let select_menu = SelectMenuSpec {
            custom_id: custom_id.clone(),
            options: unique_availability_weekdays
                .into_iter()
                .map(|(weekday, amount_times)| {
                    let weekday_string =
                        weekday.to_locale_shorthand_string(context.locale().unwrap_or("en"));
                    let next_day = weekday.next_day_with_this_weekday(&now);
                    let date_string = util::time::day_month_display(&next_day.date_naive());

                    SelectMenuOptionSpec {
                        // e.g. "Tue (23/10)"
                        label: format!("{weekday_string} ({date_string})"),
                        // value should be the weekday number so we can parse back later
                        value_key: SelectValue::from(i16::from(weekday).to_string()),
                        description: Some(if amount_times == 1 {
                            tr!("commands.schedule.one_time", ctx = context)
                        } else {
                            tr!(
                                "commands.schedule.n_times",
                                ctx = context,
                                amount = amount_times
                            )
                        }),
                        ..Default::default()
                    }
                })
                .collect(),
            ..Default::default()
        };

        data.form_start_datetime = Some(now);
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

        // parse into int then back into weekday, if possible
        let Some(selected) =
            selected.parse::<i16>().ok().map(Weekday::try_from).and_then(Result::ok)
        else {
            return Err(FormError::InvalidUserResponse.into());
        };

        let availabilities = std::mem::take(&mut data.availabilities);
        let selected_availabilities = availabilities
            .into_iter()
            .filter(|avail| avail.weekday == selected)
            .collect::<Vec<_>>();

        data.availabilities = selected_availabilities;

        Ok(Some(Box::new(SelectWeekdayComponent)))
    }
}

#[async_trait]
impl MessageFormComponent<Data, Error, ScheduleFormData> for SelectTimeComponent {
    async fn send_component(
        context: ApplicationContext<'_>,
        data: &mut FormState<ScheduleFormData>,
    ) -> ContextualResult<Vec<CustomId>> {
        let availabilities = std::mem::take(&mut data.availabilities);

        // No mentors have time available for sessions today
        if availabilities.is_empty() {
            context
                .send(|b| {
                    b.content(tr!(
                        "commands.schedule.no_mentors_available_time",
                        ctx = context
                    ))
                })
                .await?;
            return Err(FormError::Cancelled.into());
        }

        let mut unique_availability_times: Vec<(chrono::NaiveTime, usize)> =
            util::iter::group_by_count(availabilities.iter(), |avail: &Availability| {
                avail.time_start.with_second(0).unwrap()
            })
            .into_iter()
            .collect();

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

        teachers.sort_by_cached_key(|(teacher, avail)| (avail.time_start, teacher.name.clone()));

        let custom_id = CustomId::generate();
        let select_menu = SelectMenuSpec {
            custom_id: custom_id.clone(),
            options: teachers
                .iter()
                .map(|(teacher, avail)| SelectMenuOptionSpec {
                    label: teacher.name.clone(),
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

        let (teacher, availability) = interaction
            .data
            .values
            .first()
            .and_then(|v| v.parse::<i64>().ok())
            .and_then(|avail_id| {
                teacher_tuples
                    .into_iter()
                    .find(|(_, avail)| avail.id == avail_id)
            })
            .ok_or(FormError::InvalidUserResponse)?;

        Ok(Some(Box::new(Self {
            selected_availability: availability,
            selected_mentor: teacher,
        })))
    }
}
