use poise::Modal;

use crate::common::{ApplicationContext, ErrorBox};

#[derive(Default, Modal)]
#[name = "Modal title"]
struct TestModal {
    #[name = "Name"]
    #[placeholder = "Joseph"]
    #[min_length = 3]
    #[max_length = 20]
    name: String,

    #[name = "age"]
    #[max_length = 3]
    age: Option<String>,

    #[name = "More"]
    #[max_length = 500]
    #[paragraph]
    more: Option<String>,
}

/// Modal command.
#[poise::command(slash_command)]
pub async fn modal(
    ctx: ApplicationContext<'_>,
    #[description = "Modal title"] title: Option<String>,
) -> Result<(), ErrorBox> {
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
