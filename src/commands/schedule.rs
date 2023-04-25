use super::forms::schedule::ScheduleForm;
use crate::{
    commands::forms::schedule::SelectMentorComponent, common::ApplicationContext, error::Result,
    forms::InteractionForm, util, util::tr,
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
        selected_mentor_user,
        ..
    } = form.select_mentor;

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
