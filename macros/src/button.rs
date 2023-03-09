//! Implements the #[derive(ButtonComponent)] derive macro
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

    /// The button's fixed custom ID; if unspecified,
    /// it is auto-generated.
    custom_id: Option<String>,

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
}

#[derive(Debug, Copy, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
struct InteractionAttr {
    interaction: Option<()>,
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

    let button_spec = create_button_spec(&struct_attrs, &data_type);
    let create_with_interaction = single_button_create_with_interaction_code(&input)?
        .unwrap_or_else(|| quote! { Default::default() });

    let struct_ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics crate::form::ButtonComponent<#data_type> for #struct_ident #ty_generics #where_clause {
            fn on_build<'button_macro>(
                builder: &'button_macro mut ::poise::serenity_prelude::CreateButton,
                context: crate::form::ApplicationContext<'_>,
                data: &#data_type
            ) -> (&'button_macro mut ::poise::serenity_prelude::CreateButton, ::core::option::Option<crate::interaction::CustomId>) {
                #button_spec.on_build(builder, context, data)
            }

            fn create_with_interaction(interaction: ::poise::serenity_prelude::MessageComponentInteraction) -> ::std::boxed::Box<Self> {
                ::std::boxed::Box::new(#create_with_interaction)
            }
        }

        #[::async_trait::async_trait]
        impl #impl_generics crate::form::MessageFormComponent<#data_type> for #struct_ident #ty_generics #where_clause {
            async fn send_component(
                context: crate::common::ApplicationContext<'_>,
                data: &mut #data_type,
            ) -> crate::error::Result<::std::vec::Vec<crate::interaction::CustomId>> {

                let mut __custom_ids = vec![];
                context.send(|f|
                    <Self as crate::form::GenerateReply<#data_type>>::create_reply(f, context, data)
                        .components(|f| f
                            .create_action_row(|f| f
                                .create_button(|f| {
                                    let (builder, custom_id) = <Self as crate::form::ButtonComponent<#data_type>>::on_build(f, context, &data);
                                    if let Some(custom_id) = custom_id {
                                        __custom_ids.push(custom_id);
                                    }
                                    builder
                                })))).await?;

                Ok(__custom_ids)
            }

            async fn on_response(
                context: crate::common::ApplicationContext<'_>,
                interaction: ::std::sync::Arc<::poise::serenity_prelude::MessageComponentInteraction>,
                data: &mut #data_type,
            ) -> crate::error::Result<::std::boxed::Box<Self>> {
                ::std::result::Result::Ok(
                    <Self as crate::form::ButtonComponent<#data_type>>::create_with_interaction((*interaction).clone()))
            }
        }
    }.into())
}

fn validate_attrs(
    struct_attrs: &ButtonAttributes,
    input: &syn::DeriveInput,
) -> Result<(), darling::Error> {
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

fn create_button_spec(button_attrs: &ButtonAttributes, data: &syn::Type) -> TokenStream2 {
    let label = util::wrap_option_into(&button_attrs.label);
    let label_function = util::wrap_option_box(&button_attrs.label_function);
    let custom_id = util::wrap_option_into(&button_attrs.custom_id);
    let link = util::wrap_option_into(&button_attrs.link);
    let link_function = util::wrap_option_box(&button_attrs.link_function);
    let emoji = util::wrap_option_into(&button_attrs.emoji);
    let emoji_function = util::wrap_option_box(&button_attrs.emoji_function);
    let disabled = button_attrs.disabled.is_some();
    let disabled_function = util::wrap_option_box(&button_attrs.disabled_function);

    let style = if button_attrs.primary.is_some() {
        Some(quote! { ::poise::serenity_prelude::ButtonStyle::Primary })
    } else if button_attrs.secondary.is_some() {
        Some(quote! { ::poise::serenity_prelude::ButtonStyle::Secondary })
    } else if button_attrs.danger.is_some() {
        Some(quote! { ::poise::serenity_prelude::ButtonStyle::Danger })
    } else if button_attrs.success.is_some() {
        Some(quote! { ::poise::serenity_prelude::ButtonStyle::Success })
    } else if button_attrs.link.is_some() {
        Some(quote! { ::poise::serenity_prelude::ButtonStyle::Link })
    } else {
        None
    };

    let style = util::wrap_option_into(&style);

    quote! {
        crate::form::ButtonSpec::<#data> {
            label: #label,
            label_function: #label_function,
            custom_id: #custom_id,
            style: #style,
            link: #link,
            link_function: #link_function,
            emoji: #emoji,
            emoji_function: #emoji_function,
            disabled: #disabled,
            disabled_function: #disabled_function,
        }
    }
}
