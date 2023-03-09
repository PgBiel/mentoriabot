use crate::common::ApplicationContext;

/// Indicates a reply can be generated for this component type.
/// See `form::subcomponent::ReplySpec` for a struct that can be helpful
/// when using this trait.
pub trait GenerateReply<Data = ()> {
    fn create_reply<'a, 'b>(
        builder: &'a mut poise::CreateReply<'b>,
        context: ApplicationContext<'_>,
        data: &Data,
    ) -> &'a mut poise::CreateReply<'b>;
}
