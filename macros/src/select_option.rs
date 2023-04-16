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

    /// Context's Data type.
    ctx_data: syn::Type,

    /// Context's Error type.
    ctx_error: syn::Type,
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
    #[darling(default)]
    is_default: bool,
}

#[derive(Debug, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
struct FieldAttributes {
    /// The default value for this field when being initialized.
    /// Otherwise, will default to Default::default().
    initializer: Option<syn::Expr>,
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
        validate_option_attrs(&attrs, variant)?;
        variants_and_options.push((variant, attrs));
    }

    let enum_attrs: EnumAttributes = util::get_darling_attrs(&input.attrs)?;

    // ---
    let data_type = enum_attrs.data.clone().unwrap_or(util::empty_tuple_type());

    let ctx_data = &enum_attrs.ctx_data;
    let ctx_error = &enum_attrs.ctx_error;

    let option_specs =
        create_select_option_specs(&variants_and_options, &data_type, ctx_data, ctx_error);
    let from_select_value = from_select_value(&variants_and_options, &data_type)?;
    let enum_ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        #[::async_trait::async_trait]
        impl #impl_generics ::minirustbot_forms::SelectOption<#ctx_data, #ctx_error, #data_type> for #enum_ident #ty_generics #where_clause {
            /// Generates the specs(/buildables) of all possible options, based on the context
            /// and data.
            async fn generate_options(
                context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
                data: &mut #data_type
            ) -> ::minirustbot_forms::error::Result<::std::vec::Vec<::minirustbot_forms::SelectMenuOptionSpec>> {
                Ok(#option_specs)
            }

            async fn build_from_selected_value(
                value: ::minirustbot_forms::interaction::SelectValue,
                context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
                data: &mut #data_type
            ) -> ::minirustbot_forms::error::Result<Self> {
                #from_select_value
            }
        }
    }.into())
}

fn validate_option_attrs(
    option_attrs: &SelectOptionAttributes,
    variant: &syn::Variant,
) -> Result<(), darling::Error> {
    if option_attrs.label.is_some() && option_attrs.label_function.is_some() {
        return Err(syn::Error::new(
            variant.ident.span(),
            "Cannot specify #[label] and #[label_function] at the same time.",
        )
        .into());
    }

    if option_attrs.label.is_none() && option_attrs.label_function.is_none() {
        return Err(syn::Error::new(
            variant.ident.span(),
            "Must specify either a #[label] or a #[label_function], so that the menu option may have a label."
        ).into());
    }

    if option_attrs.emoji.is_some() && option_attrs.emoji_function.is_some() {
        return Err(syn::Error::new(
            variant.ident.span(),
            "Cannot specify #[emoji] and #[emoji_function] at the same time.",
        )
        .into());
    }

    if option_attrs.description.is_some() && option_attrs.description_function.is_some() {
        return Err(syn::Error::new(
            variant.ident.span(),
            "Cannot specify #[description] and #[description_function] at the same time.",
        )
        .into());
    }

    Ok(())
}

fn create_select_option_specs(
    variants_with_options: &Vec<(&syn::Variant, SelectOptionAttributes)>,
    data_type: &syn::Type,
    ctx_data: &syn::Type,
    ctx_error: &syn::Type,
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
        let is_default = is_default;

        let value_key = value_key.as_ref().expect("Missing value key");

        specs.push(quote! {
            ::minirustbot_forms::SelectMenuOptionSpec {
                label: #label,

                label_function: #label_function,

                value_key: ::minirustbot_forms::interaction::SelectValue(#value_key.into()),

                description: #description,

                description_function: #description_function,

                emoji: #emoji,

                emoji_function: #emoji_function,

                is_default: #is_default,
            }
        });
    }

    let res = quote! {
        std::vec![
            #( #specs ),*
        ]
    };
    res
}

/// Generates 'match' code that maps each expected value
/// to its corresponding enum variant.
fn from_select_value(
    variants_with_options: &Vec<(&syn::Variant, SelectOptionAttributes)>,
    _data_type: &syn::Type,
) -> Result<TokenStream2, darling::Error> {
    let mut variant_match_arms = Vec::new();

    // Map each value key to the corresponding variant
    for (variant, options) in variants_with_options {
        let variant_name = &variant.ident;
        let value_key = options
            .value_key
            .as_ref()
            .unwrap_or_else(|| panic!("Missing value key for variant '{variant_name}'"));
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

        // By default, initialize fields to Default::default()
        let default_expr: syn::Expr = syn::parse_quote! { ::core::default::Default::default() };
        for field in fields {
            let attrs: FieldAttributes = util::get_darling_attrs(&field.attrs)?;
            let initializer = attrs.initializer.as_ref().unwrap_or(&default_expr);
            if is_named {
                let field_name = field.ident.as_ref().expect("Expected named field");
                field_initializers.push(quote! { #field_name: #initializer });
            } else {
                field_initializers.push(quote! { #initializer });
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

    // catch-all: error.
    variant_match_arms.push(quote! { _ => return ::core::result::Result::Err(::minirustbot_forms::error::FormError::InvalidUserResponse.into()) });

    Ok(quote! {
        ::core::result::Result::Ok(match value.0.as_str() {
            #( #variant_match_arms ),*
        })
    })
}
