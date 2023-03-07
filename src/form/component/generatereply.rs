pub trait GenerateReply<Data = ()> {
    fn create_reply<'a>(
        create_reply: &'a mut poise::CreateReply<'a>,
        data: &Data,
    ) -> &'a mut poise::CreateReply<'a>;
}
