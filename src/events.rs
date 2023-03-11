use poise::{serenity_prelude as serenity, Event};

use crate::{common, error::Error};

pub fn handle<'a>(
    _ctx: &'a serenity::Context,
    _event: &'a Event<'a>,
    _framework_context: poise::FrameworkContext<'a, common::Data, Error>,
    _data: &'a common::Data,
) -> poise::BoxFuture<'a, Result<(), Error>> {
    Box::pin(async move { Ok(()) })
}
