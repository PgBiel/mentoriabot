use poise::ApplicationContext;

/// Indicates a reply can be generated for this component type.
/// See [`ReplySpec`] for a struct that can be helpful
/// when using this trait.
///
/// [`ReplySpec`]: super::subcomponent::ReplySpec
pub trait GenerateReply<ContextData, ContextError, Data = ()> {
    fn create_reply<'a, 'b>(
        builder: &'a mut poise::CreateReply<'b>,
        context: ApplicationContext<'_, ContextData, ContextError>,
        data: &Data,
    ) -> &'a mut poise::CreateReply<'b>;
}
