use poise::serenity_prelude as serenity;

pub trait GenerateReply {
    fn create_reply<'a>(
        create_reply: &'a mut poise::CreateReply<'a>,
    ) -> &'a mut poise::CreateReply<'a>;
}
