use darling::util::Flag;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Error;

use crate::{
    model::{macros::validate_attr, ToSpec, ValidateAttrs},
    util,
};

/// A representation of the ReplySpec struct.
#[derive(Debug, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
pub(crate) struct ReplySpecRepr {
    /// The content of the message to be sent, as an expression
    /// (in context: `context` and `data`)
    #[darling(map = "util::parse_expr")]
    content: syn::Expr,

    /// A function that takes a `&mut poise::CreateReply`, context, `&Data` and adds attachments to
    /// the CreateReply, returning the modified builder as `&mut poise::CreateReply`.
    attachment_function: Option<syn::Path>,

    /// A function that takes a `&mut CreateAllowedMentions`, context and a `&Data` and returns
    /// `&mut CreateAllowedMentions`.
    allowed_mentions_function: Option<syn::Path>,

    /// A function that takes a `&mut CreateEmbed`, context and a `&Data` and returns `&mut
    /// CreateEmbed`.
    embed_function: Option<syn::Path>,

    is_reply: Flag,

    /// If this reply should be sent as ephemeral.
    /// Exclusive with `ephemeral_expr`.
    ephemeral: Flag,

    /// An expression evaluating to `bool` (`true` if the message
    /// should be sent as ephemeral).
    ephemeral_expr: Option<syn::Path>,
}

impl ValidateAttrs for ReplySpecRepr {
    fn validate_attrs(&self) -> Result<(), Error> {
        validate_attr!(self (reply): @not_both_some(@flag ephemeral, ephemeral_expr));

        Ok(())
    }
}

impl ToSpec for ReplySpecRepr {
    fn to_spec(&self) -> TokenStream {
        let content = &self.content;
        let attachment_function = util::wrap_option_box(&self.attachment_function);
        let allowed_mentions_function = util::wrap_option_box(&self.allowed_mentions_function);
        let embed_function = util::wrap_option_box(&self.embed_function);
        let is_reply = self.is_reply.is_present();
        let ephemeral = self
            .ephemeral
            .is_present()
            .then(|| quote! { true })
            .unwrap_or_else(|| {
                self.ephemeral_expr
                    .as_ref()
                    .map(|expr| quote! { #expr.into() })
                    .unwrap_or_else(|| quote! { false })
            });

        quote! {
            ::mentoriabot_forms::ReplySpec {
                content: #content.into(),
                attachment_function: #attachment_function,
                allowed_mentions_function: #allowed_mentions_function,
                embed_function: #embed_function,
                is_reply: #is_reply,
                ephemeral: #ephemeral,
            }
        }
    }
}
