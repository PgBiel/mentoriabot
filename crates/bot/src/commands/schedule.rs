use super::{forms::schedule::ScheduleForm, modals::register::RegisterModal};
use crate::{
    commands::forms::schedule::SelectMentorComponent,
    common::ApplicationContext,
    forms::InteractionForm,
    lib::{
        db::Repository,
        error::{Error, Result},
        model::{Availability, DiscordId, NewSession},
        util::{
            self,
            time::{datetime_as_utc, datetime_with_time},
            tr,
        },
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
    let author = ctx.author();
    let author_id: DiscordId = author.id.into();

    // Insert Student into database first
    let student = {
        let student = ctx.data().db.user_repository().get(author_id).await?;

        if let Some(student) = student {
            student
        } else {
            // User not in DB => call registration modal
            if let Some(register) = RegisterModal::ask(ctx).await? {
                ctx.data()
                    .db
                    .user_repository()
                    .insert(&register.generate_new_user(author_id))
                    .await?
            } else {
                // modal cancelled
                return Ok(());
            }
        }
    };

    ctx.defer_ephemeral().await?;
    let form = *ScheduleForm::execute(ctx).await?;
    let initial_datetime = form
        .form_start_datetime
        .ok_or_else(|| Error::Other("could not get the form's starting datetime"))?;

    let SelectMentorComponent {
        selected_availability,
        selected_mentor,
        selected_mentor_user,
    } = form.select_mentor;

    let Availability {
        id: avail_id,
        time_start,
        duration,
        weekday,
        ..
    } = selected_availability;

    if ctx
        .data
        .db
        .availability_repository()
        .check_is_taken_at(avail_id, &initial_datetime)
        .await?
    {
        ctx.send(|b| b.content(tr!("commands.schedule.time_already_taken", ctx = ctx)))
            .await?;
        return Ok(());
    }

    let start_at = weekday.next_day_with_this_weekday(&initial_datetime);
    let start_at = datetime_as_utc(
        &datetime_with_time(start_at, time_start)
            .ok_or_else(|| Error::Other("failed to create session datetime object"))?,
    );
    let end_at = start_at + chrono::Duration::hours(duration as i64);
    let session = NewSession {
        teacher_id: selected_mentor.user_id,
        student_id: ctx.author().id.into(),
        availability_id: selected_availability.id,
        summary: None,
        start_at,
        end_at,
        notified: false,
    };

    // Now insert the Session between the Teacher and the Student.
    let session = ctx.data.db.session_repository().insert(&session).await?;

    ctx.data
        .google
        .email
        .send_emails_for_session(&selected_mentor, &student, &session)
        .await?;

    let event = ctx
        .data
        .google
        .calendar
        .create_event_for_session(&selected_mentor, &session)
        .await?;

    if event
        .conference_data
        .and_then(|conf| conf.create_request)
        .and_then(|req| req.status)
        .and_then(|status| status.status_code)
        .map_or(false, |status| status == "failure")
    {
        eprintln!("Creating conference failed!");
    }

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
