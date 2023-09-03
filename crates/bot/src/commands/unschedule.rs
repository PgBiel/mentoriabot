use crate::{
    common::ApplicationContext,
    lib::{
        db::Repository,
        error::{Error, Result},
        model::DiscordId,
        util::{self, tr},
    },
};

/// Unchedules a mentorship session.
#[poise::command(
    slash_command,
    ephemeral,
    name_localized("pt-BR", "desmarcar"),
    description_localized("pt-BR", "Desmarca uma sessão de mentoria.")
)]
pub async fn unschedule(
    ctx: ApplicationContext<'_>,

    #[description = "The session's number."]
    #[description_localized("pt-BR", "O número da mentoria.")]
    number: u32,
) -> Result<()> {
    // don't timeout the interaction if this takes a bit
    ctx.defer_ephemeral().await?;

    let author = ctx.author();
    let author_id: DiscordId = author.id.into();

    let Some((session, teacher, student)) = ctx.data.db.session_repository().get_with_participants(number as i64).await? else {
        ctx.say(tr!("commands.sessions.info.no_such_session", ctx = ctx, "id" => number)).await?;
        return Ok(());
    };

    let Some(availability) = ctx.data.db.availability_repository().get(session.availability_id).await? else {
        return Err(Error::Other("Expected availability associated to session to exist"));
    };

    if author_id != session.student_id {
        ctx.say(tr!("commands.session.info.not_your_session", ctx = ctx, "id" => number))
            .await?;
        return Ok(());
    }

    ctx.data
        .google
        .calendar
        .cancel_event_for_session(&session)
        .await?;

    ctx.data.db.session_repository().remove(&session).await?;

    let response = if let Err(err) = ctx
        .data
        .google
        .email
        .send_cancel_emails_for_session(&teacher, &student, &session)
        .await
    {
        tracing::warn!("Couldn't send unscheduling email: {err:?}");
        "commands.unschedule.success_no_email"
    } else {
        "commands.unschedule.success"
    };

    ctx.send(|b| {
        b.content(tr!(
            response,
            ctx = ctx,
            time = util::time::hour_minute_display(availability.time_start),
            mentor = teacher.name,
            session = session.id
        ))
    })
    .await?;

    Ok(())
}
