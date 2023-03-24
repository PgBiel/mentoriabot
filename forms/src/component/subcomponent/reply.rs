use poise::serenity_prelude as serenity;

use crate::component::Buildable;

/// Type for the attachment function property
/// of [`ReplySpec`]. Takes a CreateReply
/// and returns it, together with a vector
/// of [`serenity::AttachmentType`].
pub type AttachmentFunction = Box<
    dyn for<'a, 'b> Fn(
            &'a mut poise::CreateReply<'b>,
        ) -> (
            &'a mut poise::CreateReply<'b>,
            Vec<serenity::AttachmentType<'b>>,
        ) + Send
        + Sync,
>;

/// Holds all data necessary to display a component's reply message.
#[derive(Default)]
pub struct ReplySpec {
    /// The content of the sent message.
    pub content: String,

    /// A function that takes the `CreateReply` builder,
    /// and returns attachments.
    pub attachment_function: Option<AttachmentFunction>,

    /// A function that takes a `&mut CreateAllowedMentions`, and returns
    /// `&mut CreateAllowedMentions`.
    pub allowed_mentions_function:
        Option<Box<dyn Buildable<serenity::CreateAllowedMentions> + Send + Sync>>,

    /// A function that takes a `&mut CreateEmbed`, the context and a `&Data` and returns `&mut
    /// CreateEmbed`.
    pub embed_function: Option<Box<dyn Buildable<serenity::CreateEmbed> + Send + Sync>>,

    /// True if this message should display as a reply.
    pub is_reply: bool,

    /// True if this message should be ephemeral.
    pub ephemeral: bool,
}

impl Buildable<poise::CreateReply<'_>> for ReplySpec {
    fn on_build<'a, 'b>(
        &self,
        mut builder: &'a mut poise::CreateReply<'b>,
    ) -> &'a mut poise::CreateReply<'b> {
        builder = builder.content(self.content.clone());

        if let Some(attachment_function) = self.attachment_function.as_ref() {
            let (ret_builder, attachments) = attachment_function(builder);
            builder = ret_builder;
            for attachment in attachments {
                builder = builder.attachment(attachment);
            }
        }

        if let Some(allowed_mentions_function) = self.allowed_mentions_function.as_ref() {
            builder = builder.allowed_mentions(|b| allowed_mentions_function.on_build(b));
        }

        if let Some(embed_function) = self.embed_function.as_ref() {
            builder = builder.embed(|b| embed_function.on_build(b));
        }

        builder = builder.reply(self.is_reply).ephemeral(self.ephemeral);

        builder
    }
}
