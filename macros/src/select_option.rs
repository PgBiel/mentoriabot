//! Implements the macro for #\[derive(SelectOption)\].

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use crate::util;

#[derive(Debug, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
struct EnumAttributes {
    /// The Data type, used for the SelectOptionSpec.
    data: Option<syn::Type>,
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

pub fn select_option(input: syn::DeriveInput) -> Result<TokenStream, darling::Error> {
    let variants = match &input.data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => variants,
        _ => {
            return Err(syn::Error::new(
                input.ident.span(),
                "Can only derive SelectOption for enums.",
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
        validate_option_attrs(attrs, &input)?;
        variants_and_options.push((variant, attrs));
    }

    let enum_attrs: EnumAttributes = util::get_darling_attrs(&input.attrs)?;

    // ---
    let data_type = enum_attrs.data.clone().unwrap_or(util::empty_tuple_type());

    let option_specs = create_select_option_specs(&variants_and_options, &data_type);
    let create_with_interaction = from_select_value(&variants_and_options, &data_type);
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

fn create_select_option_specs(
    variants_with_options: &Vec<(syn::Variant, SelectOptionAttributes)>,
    data_type: &syn::Type,
) -> TokenStream2 {
    unimplemented!()
}

fn from_select_value(
    variants_with_options: &Vec<(syn::Variant, SelectOptionAttributes)>,
    data_type: &syn::Type,
) -> TokenStream2 {
    unimplemented!()
}
