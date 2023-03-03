use crate::common::{ApplicationContext, ErrorBox};
use crate::form::InteractionForm;
use crate::form::testform::TestForm;

/// Runs test form
#[poise::command(slash_command)]
pub async fn testform(
        ctx: ApplicationContext<'_>,
        ) -> Result<(), ErrorBox> {
    let data = TestForm::execute(ctx).await?;
    println!("Got testform data: {:?}", data);
    Ok(())
}
