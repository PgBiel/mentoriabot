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
    let author = ctx.author();
    let author_id: DiscordId = author.id.into();

    let Some((session, teacher)) = ctx.data.db.session_repository().get_with_teacher(number as i64).await? else {
        ctx.say(tr!("commands.sessions.info.no_such_session", ctx = ctx, "id" => number)).await?;
        return Ok(());
    };

    if author_id != session.student_id {
        ctx.say(tr!("commands.session.info.not_your_session", ctx = ctx, "id" => number))
            .await?;
        return Ok(());
    }

    todo!("Store event ID so we can cancel it here + send emails and stuff");

    ctx.data.db.session_repository().remove(&session).await?;

    Ok(())
}
