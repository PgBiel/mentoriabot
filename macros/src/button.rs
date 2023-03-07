//! Implements the #[derive(ButtonSubComponent)] derive macro
use poise::serenity_prelude::{self as serenity, ButtonStyle};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::util;

/// Representation of the struct attributes
#[derive(Debug, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
struct ButtonAttributes {
    /// Type of the Data object to be passed between components.
    /// By default, ()
    data: Option<syn::Type>,

    /// The button's literal label (please specify this, or a `label_function`)
    label: Option<String>,

    /// Path to a function (accepting &Data, as specified by `#[data = "Data"]`)
    /// that returns the button's label as a Into<String> (specify this or `label`).
    label_function: Option<syn::Path>,

    primary: Option<()>,
    secondary: Option<()>,
    success: Option<()>,
    danger: Option<()>,

    /// Makes the button lead to the given link
    /// NOTE: Such a button cannot be awaited for.
    link: Option<String>,

    /// Function takes &Data, and returns the link the button leads to (Into<String>).
    link_function: Option<syn::Path>,

    /// An optional single emoji to display near the label
    emoji: Option<char>,

    /// Function that returns the emoji to display near the button label
    /// (takes &Data, returns Into<ReactionType>)
    emoji_function: Option<syn::Path>,

    /// If this button is disabled and cannot be clicked
    disabled: Option<()>,

    /// Function that determines if this button is disabled
    /// (takes &Data, returns bool)
    disabled_function: Option<syn::Path>,

    // --- reply sending ---
    /// The content of the sent message with the button.
    message_content: Option<String>,

    /// A function that returns the content of the message to be sent,
    /// as Into<String>, taking only &Data.
    message_content_function: Option<syn::Path>,

    /// A function that takes `&Data` and returns `Into<AttachmentType>`.
    message_attachment_function: Option<syn::Path>,

    /// A function that takes a `&mut CreateAllowedMentions` and a `&Data` and returns `&mut
    /// CreateAllowedMentions`.
    message_allowed_mentions_function: Option<syn::Path>,

    /// A function that takes a `&mut CreateEmbed` and a `&Data` and returns `&mut CreateEmbed`.
    message_embed_function: Option<syn::Path>,

    message_is_reply: Option<()>,

    message_ephemeral: Option<()>,

    /// A function that takes a `&Data` and returns a `bool` (`true` if the message should be sent
    /// as ephemeral).
    message_ephemeral_function: Option<syn::Path>,
}

#[derive(Debug, Copy, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
struct InteractionAttr {
    interaction: Option<()>,
}

pub(crate) struct FinishedButtonData {
    /// Any expression that resolves to type `ButtonStyle`
    style: Option<TokenStream2>,
    /// Either a `char` or a `Into<ReactionType>`
    emoji: Option<TokenStream2>,

    /// Resolves to `bool`
    disabled: TokenStream2,

    /// Resolves to `Into<String>`
    url: Option<TokenStream2>,

    /// Resolves to `Into<String>`
    label: TokenStream2,
}

pub(crate) struct CreateReplyData {
    /// Resolves to `Into<String>`
    content: Option<TokenStream2>,

    /// Resolves to `Into<AttachmentType>`
    attachment: Option<TokenStream2>,

    /// Resolves to `FnOnce(&mut CreateAllowedMentions) -> &mut CreateAllowedMentions`
    allowed_mentions: Option<TokenStream2>,

    /// Resolves to `FnOnce(&mut CreateEmbed) -> &mut CreateEmbed`
    embed: Option<TokenStream2>,

    /// Resolves to `FnOnce(&mut CreateComponents) -> &mut CreateComponents`
    components: Option<TokenStream2>,

    /// Resolves to `bool`
    is_reply: Option<TokenStream2>,

    /// Resolves to `bool`
    is_ephemeral: Option<TokenStream2>,
}

pub fn button(input: syn::DeriveInput) -> Result<TokenStream, darling::Error> {
    let struct_attrs = input
        .attrs
        .iter()
        .map(|attr| attr.parse_meta().map(syn::NestedMeta::Meta))
        .collect::<Result<Vec<_>, _>>()?;

    let struct_attrs = <ButtonAttributes as darling::FromMeta>::from_list(&struct_attrs)?;

    validate_attrs(&struct_attrs, &input)?;

    // ---

    let data_type = struct_attrs
        .data
        .clone()
        .unwrap_or(util::empty_tuple_type());

    let button_data: FinishedButtonData = struct_attrs.clone().into();

    let button_builder_code = create_button(button_data, Some(quote! { __custom_id.clone() }));
    let create_with_interaction = single_button_create_with_interaction_code(&input)?
        .unwrap_or_else(|| quote! { Default::default() });

    let create_reply_data: CreateReplyData = struct_attrs.into();
    let create_reply_data = CreateReplyData {
        components: Some(quote! {
            |f| f.create_action_row(|f| f.create_button(__create_button))
        }),
        ..create_reply_data
    };
    let create_reply_code = compose_create_reply(create_reply_data);

    let struct_ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics crate::form::ButtonComponent<#data_type> for #struct_ident #ty_generics #where_clause {
            fn create<'button_macro>(builder: &'button_macro mut ::poise::serenity_prelude::CreateButton, data: &#data_type) -> (&'button_macro mut ::poise::serenity_prelude::CreateButton, crate::interaction::CustomId) {
                let __custom_id = crate::util::generate_custom_id();
                let __builder = builder
                    #button_builder_code;
                (__builder, crate::interaction::CustomId(__custom_id))
            }

            fn create_with_interaction(interaction: ::poise::serenity_prelude::MessageComponentInteraction) -> Self {
                #create_with_interaction
            }
        }

        #[::async_trait::async_trait]
        impl #impl_generics crate::form::MessageFormComponent<#data_type> for #struct_ident #ty_generics #where_clause {
            async fn send_component_and_wait(
                context: crate::common::ApplicationContext<'_>,
                data: &mut #data_type,
            ) -> crate::error::Result<::std::option::Option<::std::sync::Arc<::poise::serenity_prelude::MessageComponentInteraction>>> {
                let mut __custom_id = crate::interaction::CustomId(crate::util::generate_custom_id());

                // needed for the higher-ranked lifetime bound on the closure
                fn __constrain_closure<F>(f: F) -> F
                where
                    F: for<'a> FnOnce(&'a mut ::poise::serenity_prelude::CreateButton) -> &'a mut ::poise::serenity_prelude::CreateButton,
                {
                    f
                }

                let __create_button = __constrain_closure(|f| {
                    let res = <Self as crate::form::ButtonComponent<#data_type>>::create(f, &data);
                    __custom_id = res.1;
                    res.0
                });

                context.send(|f| f
                    #create_reply_code).await?;

                crate::interaction::wait_for_message_interaction(context, &__custom_id).await.map_err(crate::error::Error::Serenity)
            }

            async fn on_response(
                context: crate::common::ApplicationContext<'_>,
                interaction: ::std::sync::Arc<::poise::serenity_prelude::MessageComponentInteraction>,
                data: &mut #data_type,
            ) -> crate::error::Result<::std::boxed::Box<Self>> {
                ::std::result::Result::Ok(::std::boxed::Box::new(
                    <Self as crate::form::ButtonComponent<#data_type>>::create_with_interaction((*interaction).clone())))
            }
        }
    }.into())
}

fn validate_attrs(
    struct_attrs: &ButtonAttributes,
    input: &syn::DeriveInput,
) -> Result<(), darling::Error> {
    // ambiguity / validity checks:

    // BUTTON ATTRS

    if struct_attrs.label.is_some() && struct_attrs.label_function.is_some() {
        return Err(syn::Error::new(
            input.ident.span(),
            "Cannot specify #[label] and #[label_function] at the same time.",
        )
        .into());
    }

    if struct_attrs.label.is_none() && struct_attrs.label_function.is_none() {
        return Err(syn::Error::new(
            input.ident.span(),
            "Must specify either a #[label] or a #[label_function], so that the button may have a label."
        ).into());
    }

    if struct_attrs.emoji.is_some() && struct_attrs.emoji_function.is_some() {
        return Err(syn::Error::new(
            input.ident.span(),
            "Cannot specify #[emoji] and #[emoji_function] at the same time.",
        )
        .into());
    }

    if struct_attrs.link.is_some() && struct_attrs.link_function.is_some() {
        return Err(syn::Error::new(
            input.ident.span(),
            "Cannot specify #[link] and #[link_function] at the same time.",
        )
        .into());
    }

    if struct_attrs.disabled.is_some() && struct_attrs.disabled_function.is_some() {
        return Err(syn::Error::new(
            input.ident.span(),
            "Cannot specify #[disabled] and #[disabled_function] at the same time.",
        )
        .into());
    }

    // REPLY ATTRS

    if struct_attrs.message_content.is_some() && struct_attrs.message_content_function.is_some() {
        return Err(syn::Error::new(
            input.ident.span(),
            "Cannot specify #[message_content] and #[message_content_function] at the same time.",
        )
        .into());
    }

    if struct_attrs.message_ephemeral.is_some() && struct_attrs.message_ephemeral_function.is_some()
    {
        return Err(syn::Error::new(
            input.ident.span(),
            "Cannot specify #[message_ephemeral] and #[message_ephemeral_function] at the same time."
        ).into());
    }

    Ok(())
}

fn single_button_create_with_interaction_code(
    input: &syn::DeriveInput,
) -> Result<Option<TokenStream2>, darling::Error> {
    let mut result: Option<TokenStream2> = None;
    match &input.data {
        syn::Data::Struct(data_struct) => {
            match data_struct {
                syn::DataStruct {
                    fields: syn::Fields::Named(fields),
                    ..
                } => {
                    let len = fields.named.len();
                    for field in &fields.named {
                        let attrs: InteractionAttr = util::get_darling_attrs(&field.attrs)?;
                        if attrs.interaction.is_some() {
                            let field_name = field.ident.as_ref().expect("Expected named field");

                            let other_fields = if len == 1 {
                                None
                            } else {
                                Some(quote! { ..Default::default() })
                            };
                            result = Some(
                                quote! { Self { #field_name: interaction.into(), #other_fields } },
                            );
                            break;
                        }
                    }
                }
                syn::DataStruct {
                    fields: syn::Fields::Unnamed(fields),
                    ..
                } => {
                    let len = fields.unnamed.len();
                    for (i, field) in fields.unnamed.iter().enumerate() {
                        let attrs: InteractionAttr = util::get_darling_attrs(&field.attrs)?;
                        if attrs.interaction.is_some() {
                            if len == 1 && i == 0 {
                                result = Some(quote! { Self(interaction.into()) });
                            } else {
                                result = Some(quote! {
                                    {
                                        let mut __new: Self = Default::default();
                                        __new.#i = interaction.into();
                                        __new
                                    }
                                });
                            }
                            break;
                        }
                    }
                }
                syn::DataStruct {
                    fields: syn::Fields::Unit,
                    ..
                } => {} // can't hold data there
            }
        }
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            for variant in variants {
                let variant_name = &variant.ident;
                match &variant.fields {
                    syn::Fields::Named(fields) if fields.named.len() == 1 => {
                        for field in &fields.named {
                            let attrs: InteractionAttr = util::get_darling_attrs(&field.attrs)?;
                            if attrs.interaction.is_some() {
                                let field_name =
                                    field.ident.as_ref().expect("Expected named field");

                                result = Some(quote! {
                                    Self::#variant_name { #field_name: interaction.into() }
                                });
                                break;
                            }
                        }
                    }
                    syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                        for field in &fields.unnamed {
                            let attrs: InteractionAttr = util::get_darling_attrs(&field.attrs)?;
                            if attrs.interaction.is_some() {
                                result = Some(quote! { Self(interaction.into()) })
                            }
                        }
                    }
                    syn::Fields::Unit => break, // can't hold data
                    _ => break,
                }
            }
        }
        syn::Data::Union(data_union) => {
            return Err(syn::Error::new(
                data_union.union_token.span,
                "#[derive(ButtonComponent)] not supported in unions.",
            )
            .into())
        }
    };

    Ok(result)
}

pub(crate) fn create_button(
    button_data: FinishedButtonData,
    custom_id: Option<TokenStream2>,
) -> TokenStream2 {
    let mut builder_steps: Vec<TokenStream2> = Vec::new();

    let FinishedButtonData {
        style,
        emoji,
        disabled,
        url,
        label,
    } = button_data;

    builder_steps.push(quote! { .label(#label) });
    builder_steps.push(quote! { .disabled(#disabled) });
    if let Some(url) = url {
        builder_steps.push(quote! { .url(#url) });
    }
    if let Some(style) = style {
        builder_steps.push(quote! { .style(#style) });
    }
    if let Some(custom_id) = custom_id {
        builder_steps.push(quote! { .custom_id(#custom_id) });
    }
    if let Some(emoji) = emoji {
        builder_steps.push(quote! { .emoji(#emoji) });
    }

    quote! {
        #( #builder_steps )*
    }
}

pub(crate) fn compose_create_reply(data: CreateReplyData) -> TokenStream2 {
    let CreateReplyData {
        content,
        attachment,
        allowed_mentions,
        embed,
        components,
        is_reply,
        is_ephemeral,
    } = data;

    let content = content.map(|c| quote! { .content(#c) });
    let attachment = attachment.map(|c| quote! { .attachment(#c) });
    let allowed_mentions = allowed_mentions.map(|c| quote! { .allowed_mentions(#c) });
    let embed = embed.map(|c| quote! { .embed(#c) });
    let components = components.map(|c| quote! { .components(#c) });
    let is_reply = is_reply.map(|c| quote! { .reply(#c) });
    let is_ephemeral = is_ephemeral.map(|c| quote! { .ephemeral(#c) });

    quote! {
        #content
        #attachment
        #allowed_mentions
        #embed
        #components
        #is_reply
        #is_ephemeral
    }
}

impl From<ButtonAttributes> for FinishedButtonData {
    fn from(value: ButtonAttributes) -> Self {
        let style = if value.primary.is_some() {
            Some(quote! { ::poise::serenity_prelude::ButtonStyle::Primary })
        } else if value.secondary.is_some() {
            Some(quote! { ::poise::serenity_prelude::ButtonStyle::Secondary })
        } else if value.danger.is_some() {
            Some(quote! { ::poise::serenity_prelude::ButtonStyle::Danger })
        } else if value.success.is_some() {
            Some(quote! { ::poise::serenity_prelude::ButtonStyle::Success })
        } else if value.link.is_some() {
            Some(quote! { ::poise::serenity_prelude::ButtonStyle::Link })
        } else {
            None
        };

        let emoji = if let Some(emoji) = value.emoji {
            Some(quote! { #emoji })
        } else if let Some(emoji_function) = value.emoji_function {
            Some(quote! { { #emoji_function(&data).into() } })
        } else {
            None
        };

        let disabled = if let Some(disabled_function) = value.disabled_function {
            quote! { { #disabled_function(&data).into() } }
        } else {
            let disabled = value.disabled.is_some();
            quote! { #disabled }
        };

        let url = if let Some(link) = value.link {
            Some(quote! { #link })
        } else if let Some(link_function) = value.link_function {
            Some(quote! { { #link_function(&data).into() } })
        } else {
            None
        };

        let label = if let Some(label) = value.label {
            quote! { #label }
        } else if let Some(label_function) = value.label_function {
            quote! { { #label_function(&data).into() } }
        } else {
            panic!("Missing button label.");
        };

        FinishedButtonData {
            style,
            emoji,
            disabled,
            url,
            label,
        }
    }
}

impl From<ButtonAttributes> for CreateReplyData {
    fn from(value: ButtonAttributes) -> Self {
        let content = if let Some(content) = value.message_content {
            Some(quote! { #content })
        } else if let Some(content_function) = value.message_content_function {
            Some(quote! { { #content_function(&data).into() } })
        } else {
            None
        };

        let is_ephemeral = if value.message_ephemeral.is_some() {
            Some(quote! { true })
        } else if let Some(ephemeral_function) = value.message_ephemeral_function {
            Some(quote! { { #ephemeral_function(&data).into() } })
        } else {
            None
        };

        let attachment = value
            .message_attachment_function
            .map(|attachment_function| quote! { { #attachment_function(&data).into() } });

        let allowed_mentions = value.message_allowed_mentions_function.map(
            |allowed_mentions_function| quote! { { #allowed_mentions_function(&data).into() } },
        );

        let embed = value
            .message_embed_function
            .map(|embed_function| quote! { { #embed_function(&data).into() } });

        let components = None; // specified separately

        let is_reply = value.message_is_reply.map(|_| quote! { true });

        Self {
            content,
            attachment,
            allowed_mentions,
            embed,
            components,
            is_reply,
            is_ephemeral,
        }
    }
}

// ---

// #[cfg(test)]
// mod tests {
//     use syn::{parse_quote, DeriveInput};
//     use super::*;

//     #[test]
//     fn deriving_with_no_extra_attributes_fails() {
//         let input: DeriveInput = parse_quote! {
//             struct Button;
//         };
//         assert!(button(input).is_err());
//     }

//     #[test]
//     fn deriving_using_just_label_attribute_succeeds() {
//         let input: DeriveInput = parse_quote! {
//             #[label = "Label"]
//             struct Button;
//         };
//         assert!(button(input).is_ok());
//     }

//     #[test]
//     fn deriving_using_just_label_function_attribute_succeeds() {
//         let input: DeriveInput = parse_quote! {
//             #[label_function = my::own::function]
//             struct Button;
//         };
//         assert!(button(input).is_ok());
//     }
// }
