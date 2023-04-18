//! Implements the #[derive(SelectComponent)] derive macro
#![allow(dead_code)] // temporary

use darling::FromAttributes;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use synthez::ParseAttrs;

use crate::{
    common::{FormContextInfo, FormData, FormDataAttr},
    util,
};

/// Representation of the struct attributes
#[derive(Debug, Default, Clone, synthez::ParseAttrs)]
struct SelectAttributes {
    /// The menu's fixed custom ID; if unspecified,
    /// it is auto-generated.
    #[parse(value)]
    custom_id: Option<syn::LitStr>,

    /// Minimum amount of options the user must select.
    #[parse(value)]
    min_values: Option<syn::LitInt>,

    /// Maximum amount of options the user can select.
    #[parse(value)]
    max_values: Option<syn::LitInt>,

    /// If this menu is disabled and cannot be clicked
    #[parse(ident)]
    disabled: Option<syn::Ident>,

    /// Function that determines if this menu is disabled
    /// (takes context and &Data, returns bool)
    #[parse(value)]
    disabled_function: Option<syn::Path>,

    /// This select menu's options. If unspecified,
    /// will attempt to rely on a field marked with `#[field(selected_options)]`.
    #[parse(value)]
    options: Option<syn::Expr>,

    /// Code to run when an interaction is received. Must evaluate to a "Self",
    /// or `return` an error.
    /// If not given, `#[field(selected_options)]` will be initialized if possible.
    /// Otherwise, `Default::default()` will be attempted.
    #[parse(value)]
    on_interaction: Option<syn::Expr>,
}

#[derive(Debug, Default, Clone, synthez::ParseAttrs)]
struct SelectFieldAttributes {
    /// Marks a field as receiving the response interaction object.
    #[parse(ident)]
    interaction: Option<syn::Ident>,

    /// Marks a field as receiving the selected option(s) by the user.
    /// Its type also specifies the options that will be presented.
    #[parse(ident)]
    selected_options: Option<syn::Ident>,

    /// The initializer of any extra fields. Defaults to "Default::default()".
    #[parse(value, validate = util::require_token)]
    initializer: Option<syn::Expr>,
}

// TODO: This macro should be for enum that implements Select Options;
// we will need a struct that holds all the selected values, as a Vec.
pub fn select(input: syn::DeriveInput) -> Result<TokenStream, darling::Error> {
    let fields = match &input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => {
            return Err(syn::Error::new(
                input.ident.span(),
                "Only structs with named fields can be used for deriving a Select.",
            )
            .into());
        }
    };

    let form_data = FormDataAttr::from_attributes(&input.attrs)?;
    let select_attrs: SelectAttributes = SelectAttributes::parse_attrs("select", &input)?;

    validate_select_attrs(&select_attrs, &input)?;

    // ---
    let FormData {
        data: data_type,
        ctx: FormContextInfo {
            data: ctx_data,
            error: ctx_error,
        },
    } = &form_data.form_data;

    let fields_with_attrs = fields
        .iter()
        .map(|f| (f, SelectFieldAttributes::parse_attrs("field", f)))
        .flat_map(|(f, attrs)| {
            attrs.map(|a| vec![(f, a)]).unwrap_or(vec![]) // remove empty optional attrs
        })
        .collect::<Vec<(&syn::Field, SelectFieldAttributes)>>();

    let selected_fields = fields_with_attrs
        .iter()
        .filter(|(_, attrs)| attrs.selected_options.is_some())
        .collect::<Vec<&(&syn::Field, SelectFieldAttributes)>>();

    if selected_fields.len() > 1 {
        return Err(syn::Error::new(
            selected_fields[1].0.span(),
            "Must not mark more than one field as #[field(selected_option)].",
        )
        .into());
    }

    let options;

    if let Some(options_expr) = select_attrs.options.as_ref() {
        options = options_expr.to_token_stream()
    } else if let Some((selected_field, _)) = selected_fields.first() {
        let selected_field_type = &selected_field.ty;
        options = quote! {
            <#selected_field_type as ::minirustbot_forms::SelectMenuOptionSpec>::generate_options(
            context, data)
        }
    } else {
        return Err(syn::Error::new(
            input.ident.span(),
            "Either provide #[select(options)] explicitly, or mark a field with \
            #[field(selected_option)] (such that its type will be the options provider).",
        )
        .into());
    };

    let select_spec = create_select_spec(&select_attrs, options);
    let create_with_interaction = create_build_with_interaction(&select_attrs, &fields_with_attrs)?;
    let struct_ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        #[::async_trait::async_trait]
        impl #impl_generics ::minirustbot_forms::Subcomponent<#ctx_data, #ctx_error, #data_type> for #struct_ident #ty_generics #where_clause {
            async fn generate_buildables(
                context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
                data: &mut ::minirustbot_forms::FormState<#data_type>,
            ) -> ::minirustbot_forms::error::ContextualResult<::std::vec::Vec<Self::ReturnedBuildable>, #ctx_error> {
                Ok(::std::vec![ #select_spec ])
            }

            async fn build_from_interaction(
                context: ApplicationContext<'_, #ctx_data, #ctx_error>,
                interaction: ::std::sync::Arc<::poise::serenity_prelude::MessageComponentInteraction>,
                data: &mut ::minirustbot_forms::FormState<#data_type>,
            ) -> ::minirustbot_forms::error::ContextualResult<::std::boxed::Box<Self>, #ctx_error> {
                ::core::result::Result::Ok(::std::boxed::Box::new(#create_with_interaction))
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

fn create_build_with_interaction(
    select_attrs: &SelectAttributes,
    fields_with_attrs: &Vec<(&syn::Field, SelectFieldAttributes)>,
) -> Result<TokenStream2, darling::Error> {
    if select_attrs.on_interaction.is_some() {
        // override init
        return Ok(select_attrs.on_interaction.to_token_stream());
    }

    let mut field_initializers = vec![];
    let mut any_field_non_initialized = false;
    for (field, attrs) in fields_with_attrs {
        let field_ident = field.ident.as_ref().expect("Expected named field");

        if attrs.interaction.is_some() {
            field_initializers.push(quote! { #field_ident: interaction.into(), });
        } else if attrs.selected_options.is_some() {
            field_initializers.push(quote! {
                #field_ident: interaction.data.values.into_iter().map(From::from).collect().into(),
            })
        } else if let Some(init_expr) = attrs.initializer.as_ref() {
            field_initializers.push(quote! { #field_ident: #init_expr, })
        } else if !any_field_non_initialized {
            any_field_non_initialized = true; // this one wasn't
        }
    }

    let default_for_other_fields = if any_field_non_initialized {
        Some(quote! { ..Default::default() })
    } else {
        None
    };

    Ok(quote! {
        Self {
            #(#field_initializers)*
            #default_for_other_fields
        }
    })
}

fn create_select_spec(select_attrs: &SelectAttributes, options: TokenStream2) -> TokenStream2 {
    let custom_id = util::wrap_option_into(&select_attrs.custom_id);
    let min_values = util::wrap_option_into(&select_attrs.min_values);
    let max_values = util::wrap_option_into(&select_attrs.max_values);

    let disabled = if let Some(disabled_function) = select_attrs.disabled_function.as_ref() {
        quote! { #disabled_function(context, data).await?.into() }
    } else {
        let disabled = select_attrs.disabled.is_some();
        quote! { #disabled }
    };

    quote! {
        ::minirustbot_forms::SelectMenuSpec {
            custom_id: #custom_id,
            options: #options,
            min_values: #min_values,
            max_values: #max_values,
            disabled: #disabled,
        }
    }
}
