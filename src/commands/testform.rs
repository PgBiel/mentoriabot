use crate::common::{ApplicationContext, ErrorBox};
use crate::interaction::form::testform::create_test_form;

/// Runs test form
#[poise::command(slash_command)]
pub async fn testform(
        ctx: ApplicationContext<'_>,
        ) -> Result<(), ErrorBox> {
    let data = create_test_form().run(ctx).await?;
    println!("Got testform data: {:?}", data);
    Ok(())
}
