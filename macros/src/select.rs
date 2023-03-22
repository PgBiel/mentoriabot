//! Implements the #[derive(SelectComponent)] derive macro
#![allow(dead_code)] // temporary

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

    /// Context's Data type.
    ctx_data: syn::Type,

    /// Context's Error type.
    ctx_error: syn::Type,

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
struct SelectFieldAttributes {
    /// Marks a field as receiving the response interaction object.
    interaction: Option<()>,

    /// Marks a field as receiving the selected option by the user.
    /// Also specifies the options that will be presented.
    selected_option: Option<()>,

    /// The initializer of any extra fields. Defaults to "Default::default()".
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

    let select_attrs: SelectAttributes = util::get_darling_attrs(&input.attrs)?;

    validate_select_attrs(&select_attrs, &input)?;

    // ---
    let data_type = select_attrs
        .data
        .clone()
        .unwrap_or(util::empty_tuple_type());

    let ctx_data = &select_attrs.ctx_data;
    let ctx_error = &select_attrs.ctx_error;

    let selected_fields = fields
        .iter()
        .map(|f| {
            (
                f,
                util::get_darling_attrs::<SelectFieldAttributes>(&f.attrs),
            )
        })
        .flat_map(|(f, attrs)| attrs.map(|a| vec![(f, a)]).unwrap_or(vec![]))
        .filter(|(f, attrs)| attrs.selected_option.is_some())
        .collect::<Vec<(&syn::Field, SelectFieldAttributes)>>();

    if selected_fields.is_empty() {
        return Err(syn::Error::new(
            input.ident.span(),
            "One field must be marked as #[selected_option], which provides all options.",
        )
        .into());
    }

    if selected_fields.len() > 1 {
        return Err(syn::Error::new(
            selected_fields[1].0.span(),
            "Must not mark more than one field as #[selected_option].",
        )
        .into());
    }

    let selected_field = selected_fields[0].0;

    let select_spec = create_select_spec(&select_attrs, selected_field, &data_type);
    let create_with_interaction = create_build_with_interaction(fields)?;
    let struct_ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        #[::async_trait::async_trait]
        impl #impl_generics ::minirustbot_forms::Subcomponent<#ctx_data, #ctx_error, #data_type> for #struct_ident #ty_generics #where_clause {
            async fn generate_buildables(
                context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
                data: &mut #data_type,
            ) -> ::minirustbot_forms::error::Result<::std::vec::Vec<Self::ReturnedBuildable>> {
                Ok(::std::vec![ #select_spec ])
            }

            async fn build_from_interaction(
                context: ApplicationContext<'_, #ctx_data, #ctx_error>,
                interaction: ::std::sync::Arc<::poise::serenity_prelude::MessageComponentInteraction>,
                data: &mut #data_type,
            ) -> ::minirustbot_forms::error::Result<::std::boxed::Box<Self>> {
                ::core::result::Result::Ok(::std::boxed::Box::new(#create_with_interaction))
            }
        }

        #[::async_trait::async_trait]
        impl #impl_generics ::minirustbot_forms::MessageFormComponent<#ctx_data, #ctx_error, #data_type> for #struct_ident #ty_generics #where_clause {
            async fn send_component(
                context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
                data: &mut #data_type,
            ) -> ::minirustbot_forms::error::Result<::std::vec::Vec<::minirustbot_forms::interaction::CustomId>> {

                let mut __custom_ids = vec![];
                context.send(|f|
                    <Self as ::minirustbot_forms::GenerateReply<#data_type>>::create_reply(f, context, data)
                        .components(|f| f
                            .create_action_row(|f| f
                                .create_select(|f| {
                                    let (builder, custom_id) = <Self as ::minirustbot_forms::ButtonComponent<#ctx_data, #ctx_error, #data_type>>::on_build(f, context, &data);
                                    if let Some(custom_id) = custom_id {
                                        __custom_ids.push(custom_id);
                                    }
                                    builder
                                })))).await?;

                Ok(__custom_ids)
            }

            async fn on_response(
                context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
                interaction: ::std::sync::Arc<::poise::serenity_prelude::MessageComponentInteraction>,
                data: &mut #data_type,
            ) -> ::minirustbot_forms::error::Result<::std::boxed::Box<Self>> {
                ::std::result::Result::Ok(
                    <Self as ::minirustbot_forms::ButtonComponent<#ctx_data, #ctx_error, #data_type>>::create_with_interaction((*interaction).clone()))
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
    fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
) -> Result<TokenStream2, darling::Error> {
    todo!()
}

fn create_select_spec(
    select_attrs: &SelectAttributes,
    selected_field: &syn::Field,
    data: &syn::Type,
) -> TokenStream2 {
    let custom_id = util::wrap_option_into(&select_attrs.custom_id);
    let min_values = util::wrap_option_into(&select_attrs.min_values);
    let max_values = util::wrap_option_into(&select_attrs.max_values);

    let selected_field_type = &selected_field.ty;
    let options = quote! { #selected_field_type::generate_options(context, data) };

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
