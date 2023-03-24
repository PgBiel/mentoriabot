//! Implements the #[derive(GenerateReply)] derive macro

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::util;
use crate::util::macros::take_attribute_or_its_function_required;

#[derive(Debug, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
struct ReplyAttrs {
    /// The Data type to be accepted by the Reply.
    data: Option<syn::Type>,

    /// Context's Data type.
    ctx_data: syn::Type,

    /// Context's Error type.
    ctx_error: syn::Type,

    /// The content of the message to be sent.
    message_content: Option<String>,

    /// A function that returns the content of the message to be sent,
    /// as a String, taking the context and &Data.
    message_content_function: Option<syn::Path>,

    /// A function that takes a `&mut poise::CreateReply`, context, `&Data` and adds attachments to
    /// the CreateReply, returning the modified builder as `&mut poise::CreateReply`.
    message_attachment_function: Option<syn::Path>,

    /// A function that takes a `&mut CreateAllowedMentions`, context and a `&Data` and returns
    /// `&mut CreateAllowedMentions`.
    message_allowed_mentions_function: Option<syn::Path>,

    /// A function that takes a `&mut CreateEmbed`, context and a `&Data` and returns `&mut
    /// CreateEmbed`.
    message_embed_function: Option<syn::Path>,

    message_is_reply: Option<()>,

    message_ephemeral: Option<()>,

    /// A function that takes context and a `&Data` and returns a `bool` (`true` if the message
    /// should be sent as ephemeral).
    message_ephemeral_function: Option<syn::Path>,
}

/// Generates an implementation of GenerateReply for a given type.
pub fn reply(input: syn::DeriveInput) -> Result<TokenStream, darling::Error> {
    let struct_attrs = input
        .attrs
        .iter()
        .map(|attr| attr.parse_meta().map(syn::NestedMeta::Meta))
        .collect::<Result<Vec<_>, _>>()?;

    let struct_attrs = <ReplyAttrs as darling::FromMeta>::from_list(&struct_attrs)?;

    validate_attrs(&struct_attrs, &input)?;

    // ---

    let data_type = struct_attrs
        .data
        .clone()
        .unwrap_or(util::empty_tuple_type());

    let ctx_data = &struct_attrs.ctx_data;
    let ctx_error = &struct_attrs.ctx_error;

    let reply_spec = create_reply_spec(&struct_attrs, &data_type);

    let struct_ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        #[::async_trait::async_trait]
        impl #impl_generics ::minirustbot_forms::GenerateReply<#ctx_data, #ctx_error, #data_type> for #struct_ident #ty_generics #where_clause {
            type ReplyBuilder = ::minirustbot_forms::ReplySpec;

            async fn create_reply<'a, 'b>(
                context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
                data: &#data_type,
            ) -> ::minirustbot_forms::error::Result<Self::ReplyBuilder> {
                Ok(#reply_spec)
            }
        }
    }.into())
}

fn create_reply_spec(attrs: &ReplyAttrs, data: &syn::Type) -> TokenStream2 {
    let content = take_attribute_or_its_function_required!(attrs; message_content, message_content_function);
    let attachment_function = util::wrap_option_box(&attrs.message_attachment_function);
    let allowed_mentions_function = util::wrap_option_box(&attrs.message_allowed_mentions_function);
    let embed_function = util::wrap_option_box(&attrs.message_embed_function);
    let is_reply = attrs.message_is_reply.is_some();
    let ephemeral = attrs.message_ephemeral.map(|_| quote! { true })
        .or_else(|| {
            attrs.message_ephemeral_function.as_ref()
                .map(|func| quote! { #func(context, data).await?.into() })
        })
        .unwrap_or_else(|| quote! { false });

    quote! {
        ::minirustbot_forms::ReplySpec {
            content: #content,
            attachment_function: #attachment_function,
            allowed_mentions_function: #allowed_mentions_function,
            embed_function: #embed_function,
            is_reply: #is_reply,
            ephemeral: #ephemeral,
        }
    }
}

fn validate_attrs(attrs: &ReplyAttrs, input: &syn::DeriveInput) -> Result<(), syn::Error> {
    if attrs.message_content.is_some() && attrs.message_content_function.is_some() {
        return Err(syn::Error::new(
            input.ident.span(),
            "Cannot specify #[message_content] and #[message_content_function] at the same time.",
        ));
    }

    if attrs.message_ephemeral.is_some() && attrs.message_ephemeral_function.is_some() {
        return Err(syn::Error::new(
            input.ident.span(),
            "Cannot specify #[message_ephemeral] and #[message_ephemeral_function] at the same time."
        ));
    }

    Ok(())
}
