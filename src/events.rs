use poise::{serenity_prelude as serenity, Event};

use crate::common;

pub fn handle<'a>(
    _ctx: &'a serenity::Context,
    _event: &'a Event<'a>,
    _framework_context: poise::FrameworkContext<'a, common::Data, common::ErrorBox>,
    _data: &'a common::Data,
) -> poise::BoxFuture<'a, Result<(), common::ErrorBox>> {
    Box::pin(async move { Ok(()) })
}
