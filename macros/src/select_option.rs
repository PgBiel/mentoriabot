//! Implements the macro for #\[derive(SelectOption)\].

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

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
        validate_option_attrs(&attrs, &input)?;
        variants_and_options.push((variant, attrs));
    }

    let enum_attrs: EnumAttributes = util::get_darling_attrs(&input.attrs)?;

    // ---
    let data_type = enum_attrs.data.clone().unwrap_or(util::empty_tuple_type());

    let option_specs = create_select_option_specs(&variants_and_options, &data_type);
    let from_select_value = from_select_value(&variants_and_options, &data_type);
    let struct_ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics crate::form::SelectOption<#data_type> for #struct_ident #ty_generics #where_clause {
            fn get_specs() -> ::std::vec::Vec<crate::form::SelectMenuOptionSpec<#data_type>> {
                #option_specs
            }
        }

        #[::async_trait::async_trait]
        impl #impl_generics ::core::convert::From<crate::interaction::SelectValue> for #struct_ident #ty_generics #where_clause {
            fn from(
                value: crate::interaction::SelectValue
            ) -> Self {
                #from_select_value
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
            "Must specify either a #[label] or a #[label_function], so that the menu option may have a label."
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
    variants_with_options: &Vec<(&syn::Variant, SelectOptionAttributes)>,
    data_type: &syn::Type,
) -> TokenStream2 {
    let mut specs = Vec::new();
    for (_, options) in variants_with_options {
        let SelectOptionAttributes {
            label,
            label_function,
            value_key,
            description,
            description_function,
            emoji,
            emoji_function,
            is_default,
        } = options;

        let label = util::wrap_option_into(label);
        let label_function = util::wrap_option_box(label_function);
        let description = util::wrap_option_into(description);
        let description_function = util::wrap_option_box(description_function);
        let emoji = util::wrap_option_into(emoji);
        let emoji_function = util::wrap_option_box(emoji_function);

        let value_key = value_key.as_ref().expect("Missing value key");

        specs.push(quote! {
            crate::form::SelectMenuOptionSpec<#data_type> {
                label: #label,

                label_function: #label_function,

                value_key: #value_key,

                description: #description,

                description_function: #description_function,

                emoji: #emoji,

                emoji_function: #emoji_function,

                is_default: #is_default,
            }
        });
    }

    quote! {
        std::vec![
            #( #specs ),*
        ]
    }
}

fn from_select_value(
    variants_with_options: &Vec<(&syn::Variant, SelectOptionAttributes)>,
    data_type: &syn::Type,
) -> TokenStream2 {
    todo!()
}
