use super::forms::schedule::ScheduleForm;
use crate::{
    common::{ApplicationContext, Context},
    error::Result,
    forms::InteractionForm,
    util::tr,
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
    let availability = form.select_time.selected_availability;
    let mentor = form.select_mentor.selected_mentor;

    ctx.send(|b| {
        b.content(tr!(
            "commands.schedule.success",
            ctx = ctx,
            time = availability.time_start.format("%H:%M:%S"),
            mentor = "Jonathan"
        ))
    })
    .await?;

    Ok(())
}
