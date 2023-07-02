use async_trait::async_trait;
use poise::ApplicationContext;

use super::Buildable;
use crate::{ContextualResult, FormState};

/// Indicates a reply can be generated for this component type.
/// Does so by generating asynchronously a [`Buildable`] for a [`poise::CreateReply`].
/// This can be a lambda or a [`ReplySpec`], for example.
///
/// [`ReplySpec`]: super::subcomponent::ReplySpec
#[async_trait]
pub trait GenerateReply<ContextData, ContextError, FormData = ()> {
    type ReplyBuilder: for<'a> Buildable<poise::CreateReply<'a>>;

    async fn create_reply(
        context: ApplicationContext<'_, ContextData, ContextError>,
        data: &FormState<FormData>,
    ) -> ContextualResult<Self::ReplyBuilder, ContextError>;
}
