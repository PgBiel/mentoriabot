use poise::Modal;

use super::modals::modal::TestModal;
use crate::{common::ApplicationContext, lib::error::Error};

/// Modal command.
#[poise::command(slash_command, owners_only)]
pub async fn modal(
    ctx: ApplicationContext<'_>,
    #[description = "Modal title"] title: Option<String>,
) -> Result<(), Error> {
    let data = TestModal::execute(ctx).await?;
    if let Some(modal_data) = data {
        let response = format!(
            "Hey, {}! Thanks for '{}'.",
            modal_data.name,
            title.unwrap_or("nothing".into())
        );
        ctx.send(|f| f.content(response).ephemeral(true)).await?;
    }
    //    ctx.say(response).await?;
    Ok(())
}
