//! Implements the #[derive(GenerateReply)] derive macro

use darling::{util::Flag, FromAttributes, FromDeriveInput};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::{
    common::{FormContextInfo, FormData, FormDataAttr},
    util,
};

// use synthez to be able to input arbitrary expression into 'content'
#[derive(Debug, Clone, darling::FromDeriveInput)]
#[darling(attributes(reply))]
struct ReplyAttrs {
    /// The content of the message to be sent, as an expression
    /// (in context: `context` and `data`)
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

    ephemeral: Flag,

    /// A function that takes context and a `&Data` and returns a `bool` (`true` if the message
    /// should be sent as ephemeral).
    ephemeral_function: Option<syn::Path>,
}

/// Generates an implementation of GenerateReply for a given type.
pub fn reply(input: syn::DeriveInput) -> Result<TokenStream, darling::Error> {
    // use 'from_attributes' to ignore 'reply' (which has arbitrary expressions)
    let form_data = FormDataAttr::from_attributes(&input.attrs)?;
    let reply_attrs = ReplyAttrs::from_derive_input(&input)?;

    validate_attrs(&reply_attrs, &input)?;

    // ---

    let FormData {
        data: data_type,
        ctx: FormContextInfo {
            data: ctx_data,
            error: ctx_error,
        },
    } = &form_data.form_data;

    let reply_spec = create_reply_spec(&reply_attrs);

    let struct_ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        #[::async_trait::async_trait]
        impl #impl_generics ::minirustbot_forms::GenerateReply<#ctx_data, #ctx_error, #data_type> for #struct_ident #ty_generics #where_clause {
            type ReplyBuilder = ::minirustbot_forms::ReplySpec;

            async fn create_reply(
                context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
                data: &::minirustbot_forms::FormState<#data_type>,
            ) -> ::minirustbot_forms::error::ContextualResult<Self::ReplyBuilder, #ctx_error> {
                Ok(#reply_spec)
            }
        }
    }.into())
}

fn create_reply_spec(attrs: &ReplyAttrs) -> TokenStream2 {
    let content = &attrs.content;
    let attachment_function = util::wrap_option_box(&attrs.attachment_function);
    let allowed_mentions_function = util::wrap_option_box(&attrs.allowed_mentions_function);
    let embed_function = util::wrap_option_box(&attrs.embed_function);
    let is_reply = attrs.is_reply.is_present();
    let ephemeral = attrs
        .ephemeral
        .is_present()
        .then(|| quote! { true })
        .unwrap_or_else(|| {
            attrs
                .ephemeral_function
                .as_ref()
                .map(|func| quote! { #func(context, data).await?.into() })
                .unwrap_or_else(|| quote! { false })
        });

    quote! {
        ::minirustbot_forms::ReplySpec {
            content: #content.into(),
            attachment_function: #attachment_function,
            allowed_mentions_function: #allowed_mentions_function,
            embed_function: #embed_function,
            is_reply: #is_reply,
            ephemeral: #ephemeral,
        }
    }
}

fn validate_attrs(attrs: &ReplyAttrs, input: &syn::DeriveInput) -> Result<(), syn::Error> {
    if attrs.ephemeral.is_present() && attrs.ephemeral_function.is_some() {
        return Err(syn::Error::new(
            input.ident.span(),
            "Cannot specify 'ephemeral' and 'ephemeral_function' at the same time.",
        ));
    }

    Ok(())
}
