use chrono::{TimeZone, Timelike};

use super::forms::schedule::ScheduleForm;
use crate::{
    commands::forms::schedule::SelectMentorComponent,
    common::ApplicationContext,
    error::{Error, Result},
    forms::InteractionForm,
    model::{Availability, DiscordId, NewSession, NewUser},
    repository::Repository,
    util,
    util::{
        time::{brazil_now, datetime_as_utc, datetime_with_time},
        tr, BRAZIL_TIMEZONE,
    },
};

/// Schedules a session with a Mentor
#[poise::command(
    slash_command,
    ephemeral,
    name_localized("pt-BR", "marcar"),
    description_localized("pt-BR", "Marca uma sessão com um mentor.")
)]
pub async fn schedule(ctx: ApplicationContext<'_>) -> Result<()> {
    let form = *ScheduleForm::execute(ctx).await?;
    let SelectMentorComponent {
        selected_availability,
        selected_mentor,
        selected_mentor_user,
    } = form.select_mentor;

    let Availability {
        time_start,
        time_end,
        ..
    } = selected_availability;

    let author = ctx.author();
    let author_id: DiscordId = author.id.into();

    // Insert Student into database first
    ctx.data()
        .db
        .user_repository()
        .insert_if_not_exists(&NewUser {
            discord_id: author_id.clone(),
            name: author.name.clone(),
            bio: None,
        })
        .await?;

    let session = NewSession {
        availability_id: Some(selected_availability.id),
        teacher_id: selected_mentor.user_id,
        student_id: ctx.author().id.into(),
        name: "".to_string(),
        description: "".to_string(),
        start_at: datetime_as_utc(
            &datetime_with_time(brazil_now(), time_start)
                .ok_or_else(|| Error::Other("failed to create session datetime object"))?,
        ),
        end_at: datetime_as_utc(
            &datetime_with_time(brazil_now(), time_end)
                .ok_or_else(|| Error::Other("failed to create session datetime object"))?,
        ),
        notified: false,
    };

    // Now insert the Session between the Teacher and the Student.
    ctx.data.db.session_repository().insert(&session).await?;

    ctx.send(|b| {
        b.content(tr!(
            "commands.schedule.success",
            ctx = ctx,
            time = util::time::hour_minute_display(selected_availability.time_start),
            mentor = selected_mentor_user.name
        ))
    })
    .await?;

    Ok(())
}
