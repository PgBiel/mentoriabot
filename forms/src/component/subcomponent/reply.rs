use poise::{serenity_prelude as serenity, ApplicationContext};

/// Holds all data necessary to display a component's reply message.
pub struct ReplySpec<ContextData, ContextError, Data = ()> {
    /// The content of the sent message with the button.
    pub content: Option<String>,

    /// A function that returns the content of the message to be sent,
    /// as a String, taking as parameters the context and &Data.
    pub content_function: Option<Box<dyn Fn(ApplicationContext<'_, ContextData, ContextError>, &Data) -> String>>,

    /// A function that takes the CreateReply builder, the context, `&Data`,
    /// and internally adds attachments.
    pub attachment_function: Option<
        Box<
            dyn for<'a, 'b> Fn(
                &'a mut poise::CreateReply<'b>,
                ApplicationContext<'_, ContextData, ContextError>,
                &Data,
            ) -> &'a mut poise::CreateReply<'b>,
        >,
    >,

    /// A function that takes a `&mut CreateAllowedMentions`, the context and a `&Data` and returns
    /// `&mut CreateAllowedMentions`.
    pub allowed_mentions_function: Option<
        Box<
            dyn for<'a, 'b> Fn(
                &'a mut serenity::CreateAllowedMentions,
                ApplicationContext<'b, ContextData, ContextError>,
                &Data,
            ) -> &'a mut serenity::CreateAllowedMentions,
        >,
    >,

    /// A function that takes a `&mut CreateEmbed`, the context and a `&Data` and returns `&mut
    /// CreateEmbed`.
    pub embed_function: Option<
        Box<
            dyn for<'a, 'b> Fn(
                &'a mut serenity::CreateEmbed,
                ApplicationContext<'b, ContextData, ContextError>,
                &Data,
            ) -> &'a mut serenity::CreateEmbed,
        >,
    >,

    /// True if this message should display as a reply.
    pub is_reply: bool,

    /// True if this message should be ephemeral.
    pub ephemeral: bool,

    /// A function that takes a `&Data` and returns a `bool` (`true` if the message should be sent
    /// as ephemeral).
    pub ephemeral_function: Option<Box<dyn Fn(ApplicationContext<'_, ContextData, ContextError>, &Data) -> bool>>,
}

impl<CD, CE, D> ReplySpec<CD, CE, D> {
    pub fn create_reply<'a, 'b>(
        &self,
        mut builder: &'a mut poise::CreateReply<'b>,
        context: ApplicationContext<'_, CD, CE>,
        data: &D,
    ) -> &'a mut poise::CreateReply<'b> {
        builder = builder.content(
            self.content_function
                .as_ref()
                .map(|function| function(context, data))
                .or_else(|| self.content.clone())
                .unwrap_or(String::from("")),
        );

        if let Some(attachment_function) = self.attachment_function.as_ref() {
            builder = attachment_function(builder, context, data);
        }

        if let Some(allowed_mentions_function) = self.allowed_mentions_function.as_ref() {
            builder = builder.allowed_mentions(|b| allowed_mentions_function(b, context, data));
        }

        if let Some(embed_function) = self.embed_function.as_ref() {
            builder = builder.embed(|b| embed_function(b, context, data));
        }

        builder = builder.reply(self.is_reply).ephemeral(
            self.ephemeral_function
                .as_ref()
                .map(|function| function(context, data))
                .unwrap_or(self.ephemeral),
        );

        builder
    }
}
