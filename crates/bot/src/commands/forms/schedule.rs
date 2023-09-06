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
    pub(crate) select_mentor: SelectMentorComponent,

    #[component]
    #[allow(dead_code)]
    pub(crate) select_weekday: SelectWeekdayComponent,

    #[component]
    #[allow(dead_code)]
    pub(crate) select_time: SelectTimeComponent,
}

/// Component which allows the user to pick a mentor
/// available in the next 7 days.
#[derive(Debug, Clone, GenerateReply)]
#[form_data(data(ScheduleFormData), ctx(Data, Error))]
#[reply(content = (
    select_mentor_reply_content(context, data).await?
), ephemeral)]
pub(crate) struct SelectMentorComponent;

/// Component which allows the user to pick a weekday
/// in which the mentor is available.
#[derive(Debug, Clone, GenerateReply)]
#[form_data(data(ScheduleFormData), ctx(Data, Error))]
#[reply(content = (
    select_weekday_reply_content(context, data).await?
), ephemeral)]
pub(crate) struct SelectWeekdayComponent;

/// Component which allows the user to pick the time
/// in that weekday to schedule with the chosen mentor.
#[derive(Debug, Clone, GenerateReply)]
#[form_data(data(ScheduleFormData), ctx(Data, Error))]
#[reply(content = (
    select_time_reply_content(context, data).await?
), ephemeral)]
pub(crate) struct SelectTimeComponent {
    pub(crate) selected_availability: Availability,
    pub(crate) selected_mentor: Teacher,
}

/// Stores data while the ScheduleForm is still being constructed.
#[derive(Debug, Default)]
pub(crate) struct ScheduleFormData {
    form_start_datetime: Option<chrono::DateTime<chrono::FixedOffset>>,

    // updated by each component as availabilities get filtered further
    availabilities: Vec<Availability>,

    // used by SelectTeacher to store the retrieved teachers from the DB
    available_mentors: Vec<Teacher>,

    // used by SelectTime to store available times (to build the reply message)
    available_times: Vec<chrono::NaiveTime>,

    // defined by SelectTeacher's on_response
    selected_mentor: Option<Teacher>,
}

// --- impls ---

impl ScheduleForm {
    /// Retrieves the user's selected availability and mentor from the form.
    pub fn retrieve_selection(self) -> (Availability, Teacher) {
        (
            self.select_time.selected_availability,
            self.select_time.selected_mentor,
        )
    }
}

impl ScheduleFormData {
    /// Reduces the global set of availabilities, thus refining the pool based on user input.
    /// Only keeps the availabilities for which the 'filter' function returns true.
    fn filter_availabilities(&mut self, filter: impl FnMut(&Availability) -> bool) {
        self.availabilities.retain(filter);
    }

    /// Clears the list of potential / available mentors and attempts to select the mentor
    /// with the given ID from it, returning a Form Error if no such teacher exists.
    fn select_mentor(&mut self, teacher_id: i64) -> FormResult<()> {
        let available_mentors = std::mem::take(&mut self.available_mentors);
        if let Some(teacher) = available_mentors
            .into_iter()
            .find(|teacher| teacher.id == teacher_id)
        {
            self.selected_mentor = Some(teacher);
            Ok(())
        } else {
            Err(FormError::InvalidUserResponse)
        }
    }

    /// Clears the list of availabilities and returns the availability with the given ID,
    /// if it exists. Otherwise, returns a FormError.
    fn select_availability(&mut self, avail_id: i64) -> FormResult<Availability> {
        let availabilities = std::mem::take(&mut self.availabilities);
        if let Some(availability) = availabilities
            .into_iter()
            .find(|availability| availability.id == avail_id)
        {
            Ok(availability)
        } else {
            Err(FormError::InvalidUserResponse)
        }
    }
}

/// Inits the form data by pulling valid availabilities and recording the current timestamp.
/// Cancels the form if there are none.
/// This should be run by the first form component.
async fn init_form_data(
    context: ApplicationContext<'_>,
    data: &mut FormState<ScheduleFormData>,
) -> ContextualResult<()> {
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

    data.form_start_datetime = Some(now);
    data.availabilities = availabilities;

    Ok(())
}

/// Given an interaction and a parser, attempts to apply the parser on the first received value.
/// This is usually an ID or something that is inserted into a select menu option's value key.
/// If parsing fails, an Invalid User Response error is returned.
fn parse_interaction_response_or_error<T>(
    interaction: Arc<MessageComponentInteraction>,
    parser: impl FnOnce(&String) -> Option<T>,
) -> FormResult<T> {
    interaction
        .data
        .values
        .first()
        .and_then(parser)
        .ok_or(FormError::InvalidUserResponse)
}

async fn select_mentor_reply_content(
    context: ApplicationContext<'_>,
    data: &FormState<ScheduleFormData>,
) -> FormResult<String> {
    let mentor_count = data.available_mentors.len();
    Ok(if mentor_count == 1 {
        tr!("commands.schedule.please_select_mentor_one", ctx = context)
    } else {
        tr!(
            "commands.schedule.please_select_mentor_n",
            ctx = context,
            mentor_count = mentor_count
        )
    })
}

async fn select_weekday_reply_content(
    context: ApplicationContext<'_>,
    data: &FormState<ScheduleFormData>,
) -> FormResult<String> {
    let Some(mentor) = data.selected_mentor.as_ref() else {
        return Err(FormError::InvalidUserResponse);
    };
    let mentor = &mentor.name;
    let avail_count = data.availabilities.len();
    Ok(if avail_count == 1 {
        tr!(
            "commands.schedule.please_select_weekday_one",
            ctx = context,
            mentor = mentor
        )
    } else {
        tr!(
            "commands.schedule.please_select_weekday_n",
            ctx = context,
            session_count = avail_count,
            mentor = mentor
        )
    })
}

async fn select_time_reply_content(
    context: ApplicationContext<'_>,
    data: &FormState<ScheduleFormData>,
) -> ContextualResult<String> {
    let sample_availability = data.availabilities.first();
    let (selected_weekday, selected_date) = sample_availability
        .zip(data.form_start_datetime.as_ref())
        .map(|(avail, initial_date)| (avail.weekday, avail.first_possible_date_after(initial_date)))
        .ok_or_else(|| Error::Other("Couldn't get the selected date at 'schedule' form."))?;
    let date_string = util::time::day_month_display(&selected_date.date_naive());
    let weekday_string = selected_weekday
        .to_locale_shorthand_string(util::locale::get_defaulted_app_ctx_locale(context));

    // already fully filtered
    let available_times = data.availabilities.len();

    Ok(if available_times == 1 {
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
            time_count = available_times,
            day = date_string,
            weekday = weekday_string
        )
    })
}

#[async_trait]
impl MessageFormComponent<Data, Error, ScheduleFormData> for SelectMentorComponent {
    async fn send_component(
        context: ApplicationContext<'_>,
        data: &mut FormState<ScheduleFormData>,
    ) -> ContextualResult<Vec<CustomId>> {
        init_form_data(context, data).await?;

        let availabilities = &data.availabilities;

        let unique_teachers: Vec<(i64, usize)> =
            util::iter::group_by_count(availabilities.iter(), |avail: &Availability| {
                avail.teacher_id
            })
            .into_iter()
            .collect();

        let mut teachers = context
            .data
            .db
            .teacher_repository()
            .find_by_ids(
                &unique_teachers
                    .iter()
                    .map(|(teacher, _)| *teacher)
                    .collect::<Vec<_>>(),
            )
            .await?;

        if teachers.is_empty() {
            return Err(FormError::InvalidUserResponse.into());
        }

        util::iter::sort_by_reference_key(&mut teachers, |teacher| &teacher.name);

        let custom_id = CustomId::generate();
        let select_menu = SelectMenuSpec {
            custom_id: custom_id.clone(),
            options: teachers
                .iter()
                .map(|teacher| {
                    util::apply_limits_to_select_option_spec(SelectMenuOptionSpec {
                        label: teacher.name.clone(),
                        // encode the teacher id into the select option's value key
                        // we retrieve this back and parse it on response below.
                        value_key: SelectValue::from(teacher.id.to_string()),
                        description: Some(teacher.specialty.to_string()),
                        ..Default::default()
                    })
                })
                .collect(),
            ..Default::default()
        };

        data.available_mentors = teachers;

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
        _: ApplicationContext<'_>,
        interaction: Arc<MessageComponentInteraction>,
        data: &mut FormState<ScheduleFormData>,
    ) -> ContextualResult<Option<Box<Self>>> {
        let teacher_id: i64 = parse_interaction_response_or_error(interaction, |teacher_id| {
            teacher_id.parse::<i64>().ok()
        })?;

        data.select_mentor(teacher_id)?;
        data.filter_availabilities(|avail| avail.teacher_id == teacher_id);

        Ok(Some(Box::new(Self)))
    }
}

#[async_trait]
impl MessageFormComponent<Data, Error, ScheduleFormData> for SelectWeekdayComponent {
    async fn send_component(
        context: ApplicationContext<'_>,
        data: &mut FormState<ScheduleFormData>,
    ) -> ContextualResult<Vec<CustomId>> {
        // 'init_form_data' should have set this.
        let now = data
            .form_start_datetime
            .ok_or_else(|| Error::Other("could not get the form's starting datetime"))?;

        let availabilities = &data.availabilities;

        let mut unique_availability_weekdays: Vec<(Weekday, usize)> =
            util::iter::group_by_count(availabilities.iter(), |avail: &Availability| avail.weekday)
                .into_iter()
                .collect();

        // sort by increasing dates
        unique_availability_weekdays
            .sort_unstable_by_key(|(weekday, _)| weekday.next_day_with_this_weekday(&now));

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

                    util::apply_limits_to_select_option_spec(SelectMenuOptionSpec {
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
                    })
                })
                .collect(),
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
        _context: ApplicationContext<'_>,
        interaction: Arc<MessageComponentInteraction>,
        data: &mut FormState<ScheduleFormData>,
    ) -> ContextualResult<Option<Box<Self>>> {
        // we encode the weekday as an int in the selected option's value key.
        let selected_weekday = parse_interaction_response_or_error(interaction, |selection| {
            selection
                .parse::<i16>()
                .ok()
                .map(Weekday::try_from)
                .and_then(Result::ok)
        })?;

        data.filter_availabilities(|avail| avail.weekday == selected_weekday);

        Ok(Some(Box::new(Self)))
    }
}

#[async_trait]
impl MessageFormComponent<Data, Error, ScheduleFormData> for SelectTimeComponent {
    async fn send_component(
        context: ApplicationContext<'_>,
        data: &mut FormState<ScheduleFormData>,
    ) -> ContextualResult<Vec<CustomId>> {
        let availabilities = &data.availabilities;

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

        let mut availability_times: Vec<(chrono::NaiveTime, &Availability)> = availabilities
            .iter()
            .map(|avail| (avail.time_start.with_second(0).unwrap(), avail))
            .collect();

        // sort by increasing times
        availability_times.sort_unstable_by_key(|(time, _)| *time);

        let custom_id = CustomId::generate();
        let select_menu = SelectMenuSpec {
            custom_id: custom_id.clone(),
            options: availability_times
                .iter()
                .map(|(time, availability)| {
                    let time_string = hour_minute_display(*time).to_string();
                    util::apply_limits_to_select_option_spec(SelectMenuOptionSpec {
                        label: time_string.clone(),
                        // encode the availability id in the option's value key
                        value_key: SelectValue::from(availability.id.to_string()),
                        description: None,
                        ..Default::default()
                    })
                })
                .collect(),
            ..Default::default()
        };

        data.available_times = availability_times
            .into_iter()
            .map(|(time, _)| time)
            .collect();

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
        let avail_id = parse_interaction_response_or_error(interaction, |selection| {
            selection.parse::<i64>().ok()
        })?;

        let selected_availability = data.select_availability(avail_id)?;
        let selected_mentor = std::mem::take(&mut data.selected_mentor).ok_or_else(|| {
            Error::Other("Selected mentor was missing in form data for some reason")
        })?;

        // This is the final step of the form, so we return the selected things
        Ok(Some(Box::new(Self {
            selected_availability,
            selected_mentor,
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_availability(id: i64) -> Availability {
        Availability {
            id,
            teacher_id: 0,
            weekday: Weekday::Friday,
            time_start: chrono::NaiveTime::from_hms_opt(12, 30, 30).unwrap(),
            expired: false,
            duration: 1,
        }
    }

    fn make_mentor(id: i64) -> Teacher {
        Teacher {
            id,
            name: "Teacher".to_owned(),
            email: "exists@at.com".to_owned(),
            specialty: "Everything".to_owned(),
            applied_at: None,
            bio: None,
            course_info: None,
            company: None,
            company_role: None,
            whatsapp: None,
            linkedin: None,
            comment_general: None,
            comment_experience: None,
        }
    }

    #[test]
    fn test_form_data_availability_filtering_works_properly() {
        let mut data = ScheduleFormData {
            availabilities: vec![
                make_availability(1),
                make_availability(2),
                make_availability(3),
                make_availability(10),
                make_availability(15),
            ],
            ..Default::default()
        };

        data.filter_availabilities(|avail| avail.id > 5);

        assert_eq!(
            data.availabilities,
            vec![make_availability(10), make_availability(15),]
        );
    }

    #[test]
    fn test_select_mentor_works_when_the_mentor_exists() {
        let mut data = ScheduleFormData {
            available_mentors: vec![
                make_mentor(1),
                make_mentor(2),
                make_mentor(3),
                make_mentor(10),
                make_mentor(15),
            ],
            ..Default::default()
        };

        data.select_mentor(10).unwrap();

        assert_eq!(data.selected_mentor, Some(make_mentor(10)));
    }

    #[test]
    fn test_select_mentor_fails_when_there_is_no_such_mentor() {
        let mut data = ScheduleFormData {
            available_mentors: vec![
                make_mentor(1),
                make_mentor(2),
                make_mentor(3),
                make_mentor(10),
                make_mentor(15),
            ],
            ..Default::default()
        };

        assert!(matches!(
            data.select_mentor(11),
            Err(FormError::InvalidUserResponse)
        ));
    }

    #[test]
    fn test_select_availability_works_when_the_availability_exists() {
        let mut data = ScheduleFormData {
            availabilities: vec![
                make_availability(1),
                make_availability(2),
                make_availability(3),
                make_availability(10),
                make_availability(15),
            ],
            ..Default::default()
        };

        assert_eq!(
            data.select_availability(15).ok(),
            Some(make_availability(15))
        );
    }

    #[test]
    fn test_select_availability_fails_when_there_is_no_such_availability() {
        let mut data = ScheduleFormData {
            availabilities: vec![
                make_availability(1),
                make_availability(2),
                make_availability(3),
                make_availability(10),
                make_availability(15),
            ],
            ..Default::default()
        };

        assert!(matches!(
            data.select_availability(12),
            Err(FormError::InvalidUserResponse)
        ));
    }
}
