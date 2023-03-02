use poise::serenity_prelude as serenity;
use poise::Event;
use super::common;

pub fn handle<'a>(_ctx: &'a serenity::Context,
                  event: &'a Event<'a>,
                  _framework_context: poise::FrameworkContext<'a, common::Data, common::Error>,
                  data: &'a common::Data)
        -> poise::BoxFuture<'a, Result<(), common::Error>> {
    Box::pin(async move {
        Ok(())
    })
}
