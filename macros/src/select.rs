//! Implements the #[derive(SelectComponent)] derive macro
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::spanned::Spanned;

use crate::util;

/// Representation of the struct attributes
#[derive(Debug, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
struct SelectAttributes {
    /// Type of the Data object to be passed between components.
    /// By default, ()
    data: Option<syn::Type>,

    /// The button's fixed custom ID; if unspecified,
    /// it is auto-generated.
    custom_id: Option<String>,

    /// Minimum amount of options the user must select.
    min_values: Option<u64>,

    /// Maximum amount of options the user can select.
    max_values: Option<u64>,

    /// If this menu is disabled and cannot be clicked
    disabled: Option<()>,

    /// Function that determines if this menu is disabled
    /// (takes context and &Data, returns bool)
    disabled_function: Option<syn::Path>,
}

#[derive(Debug, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
struct SelectOptionAttributes {
    /// The option's literal label (please specify this, or a `label_function`)
    label: Option<String>,

    /// A function (accepting context and &Data)
    /// that returns the option's label as a String (specify this or `label`).
    label_function: Option<syn::Path>,

    /// This option's unique value key.
    /// By default, this is set to the name of the enum variant
    /// this is applied to.
    value_key: Option<String>,

    /// The option's description (small explanation text), optional.
    description: Option<String>,

    /// Function that takes context and &Data
    /// and returns the option's description (optional).
    description_function: Option<syn::Path>,

    /// An optional single emoji to display near the label
    emoji: Option<char>,

    /// Function that returns the emoji to display near the button label
    /// (takes context and &Data, returns ReactionType)
    emoji_function: Option<syn::Path>,

    /// If this is the default selection option.
    is_default: bool,
}

#[derive(Debug, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
struct InteractionAttr {
    interaction: Option<()>,

    default: Option<syn::Expr>,
}

// TODO: This macro should be for enum that implements Select Options;
// we will need a struct that holds all the selected values, as a Vec.
pub fn select(input: syn::DeriveInput) -> Result<TokenStream, darling::Error> {
    let variants = match &input.data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => variants,
        _ => {
            return Err(syn::Error::new(
                input.ident.span(),
                "Can only derive SelectComponent for enums.",
            )
            .into())
        }
    };

    let variants_and_attrs = variants.iter().map(|variant| {
        (
            variant,
            util::get_darling_attrs::<SelectOptionAttributes>(&variant.attrs),
        )
    });

    let mut variants_and_options = Vec::new();

    for (variant, attrs) in variants_and_attrs {
        let mut attrs = attrs?;
        if attrs.value_key.is_none() {
            attrs.value_key = Some(variant.ident.to_string()); // default value key is the variant's
                                                               // name
        }
        variants_and_options.push((variant, attrs));
    }

    let select_attrs: SelectAttributes = util::get_darling_attrs(&input.attrs)?;

    validate_select_attrs(&select_attrs, &input)?;

    // ---
    let data_type = select_attrs
        .data
        .clone()
        .unwrap_or(util::empty_tuple_type());

    let button_spec = create_select_spec(&select_attrs, &data_type);
    let create_with_interaction = create_variant_with_interaction(&variants_and_options)?;
    let struct_ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics crate::form::SelectComponent<#data_type> for #struct_ident #ty_generics #where_clause {
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

fn validate_select_attrs(
    select_attrs: &SelectAttributes,
    input: &syn::DeriveInput,
) -> Result<(), darling::Error> {
    if select_attrs.disabled.is_some() && select_attrs.disabled_function.is_some() {
        return Err(syn::Error::new(
            input.ident.span(),
            "Cannot specify #[disabled] and #[disabled_function] at the same time.",
        )
        .into());
    }

    Ok(())
}

fn validate_option_attrs(
    option_attrs: &SelectOptionAttributes,
    input: &syn::DeriveInput,
) -> Result<(), darling::Error> {
    if option_attrs.label.is_some() && option_attrs.label_function.is_some() {
        return Err(syn::Error::new(
            input.ident.span(),
            "Cannot specify #[label] and #[label_function] at the same time.",
        )
        .into());
    }

    if option_attrs.label.is_none() && option_attrs.label_function.is_none() {
        return Err(syn::Error::new(
            input.ident.span(),
            "Must specify either a #[label] or a #[label_function], so that the button may have a label."
        ).into());
    }

    if option_attrs.emoji.is_some() && option_attrs.emoji_function.is_some() {
        return Err(syn::Error::new(
            input.ident.span(),
            "Cannot specify #[emoji] and #[emoji_function] at the same time.",
        )
        .into());
    }

    if option_attrs.description.is_some() && option_attrs.description_function.is_some() {
        return Err(syn::Error::new(
            input.ident.span(),
            "Cannot specify #[description] and #[description_function] at the same time.",
        )
        .into());
    }

    Ok(())
}

fn create_variant_with_interaction(
    variants_and_options: &Vec<(&syn::Variant, SelectOptionAttributes)>,
) -> Result<TokenStream2, darling::Error> {
    let mut variant_match_arms = Vec::new();

    for (variant, options) in variants_and_options {
        let variant_name = &variant.ident;
        let value_key = options
            .value_key
            .as_ref()
            .expect(&*format!("Missing value key for variant '{variant_name}'"));
        let mut is_named = false;
        let fields = match &variant.fields {
            syn::Fields::Named(fields) => {
                is_named = true;
                &fields.named
            }
            syn::Fields::Unnamed(fields) => &fields.unnamed,
            syn::Fields::Unit => {
                variant_match_arms.push(quote! { #value_key => Self::#variant_name });
                continue;
            }
        };

        let mut field_initializers = Vec::new();
        let mut interaction_seen = false;
        let mut non_default_field = None;
        for field in fields {
            let attrs: InteractionAttr = util::get_darling_attrs(&field.attrs)?;
            if attrs.interaction.is_some() {
                if interaction_seen {
                    return Err(syn::Error::new(
                        field
                            .ident
                            .as_ref()
                            .map(|i| i.span())
                            .unwrap_or_else(|| field.span()),
                        "Cannot have more than one field in the same variant with #[interaction].",
                    )
                    .into());
                }

                interaction_seen = true;

                if non_default_field.is_some() {
                    break; // see below
                }

                if is_named {
                    let field_name = field.ident.as_ref().expect("Expected named field");
                    field_initializers.push(quote! { #field_name: interaction.into() });
                } else {
                    field_initializers.push(quote! { interaction.into() });
                }
                break;
            } else if let Some(initializer) = attrs.default {
                if is_named {
                    let field_name = field.ident.as_ref().expect("Expected named field");
                    field_initializers.push(quote! { #field_name: #initializer });
                } else {
                    field_initializers.push(quote! { #initializer });
                }
            } else {
                non_default_field = Some(field);
                if interaction_seen {
                    break; // cannot mix #[interaction] with non-#[default] fields
                }
            };
        }

        if interaction_seen {
            if let Some(non_default_field) = non_default_field {
                return Err(syn::Error::new(
                    non_default_field.ident.as_ref().map(|i| i.span()).unwrap_or_else(|| non_default_field.span()),
                    "Cannot specify #[interaction] for a field and not specify #[default = \"expr\"] on the others."
                ).into());
            }
        }

        variant_match_arms.push(if is_named {
            quote! { #value_key => Self::#variant_name {
                #( #field_initializers ),*
            } }
        } else {
            quote! { #value_key => Self::#variant_name( #( #field_initializers ),* ) }
        });
    }

    Ok(quote! {
        match __value {
            #( #variant_match_arms ),*
        }
    })
}

fn create_select_spec(button_attrs: &SelectAttributes, data: &syn::Type) -> TokenStream2 {
    unimplemented!()
    // let label = util::wrap_option_into(&button_attrs.label);
    // let label_function = util::wrap_option_box(&button_attrs.label_function);
    // let custom_id = util::wrap_option_into(&button_attrs.custom_id);
    // let link = util::wrap_option_into(&button_attrs.link);
    // let link_function = util::wrap_option_box(&button_attrs.link_function);
    // let emoji = util::wrap_option_into(&button_attrs.emoji);
    // let emoji_function = util::wrap_option_box(&button_attrs.emoji_function);
    // let disabled = button_attrs.disabled.is_some();
    // let disabled_function = util::wrap_option_box(&button_attrs.disabled_function);

    // let style = if button_attrs.primary.is_some() {
    //     Some(quote! { ::poise::serenity_prelude::ButtonStyle::Primary })
    // } else if button_attrs.secondary.is_some() {
    //     Some(quote! { ::poise::serenity_prelude::ButtonStyle::Secondary })
    // } else if button_attrs.danger.is_some() {
    //     Some(quote! { ::poise::serenity_prelude::ButtonStyle::Danger })
    // } else if button_attrs.success.is_some() {
    //     Some(quote! { ::poise::serenity_prelude::ButtonStyle::Success })
    // } else if button_attrs.link.is_some() {
    //     Some(quote! { ::poise::serenity_prelude::ButtonStyle::Link })
    // } else {
    //     None
    // };

    // let style = util::wrap_option_into(&style);

    // quote! {
    //     crate::form::ButtonSpec::<#data> {
    //         label: #label,
    //         label_function: #label_function,
    //         custom_id: #custom_id,
    //         style: #style,
    //         link: #link,
    //         link_function: #link_function,
    //         emoji: #emoji,
    //         emoji_function: #emoji_function,
    //         disabled: #disabled,
    //         disabled_function: #disabled_function,
    //     }
    // }
}
