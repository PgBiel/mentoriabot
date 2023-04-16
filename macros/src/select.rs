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
    #[darling(default)]
    disabled: bool,

    /// Function that determines if this menu is disabled
    /// (takes context and &Data, returns bool)
    disabled_function: Option<syn::Path>,
}

#[derive(Debug, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
struct SelectFieldAttributes {
    /// Marks a field as receiving the response interaction object.
    #[darling(default)]
    interaction: bool,

    /// Marks a field as receiving the selected option(s) by the user.
    /// Its type also specifies the options that will be presented.
    #[darling(default)]
    selected_options: bool,

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

    let fields_with_attrs = fields
        .iter()
        .map(|f| {
            (
                f,
                util::get_darling_attrs::<SelectFieldAttributes>(&f.attrs),
            )
        })
        .flat_map(|(f, attrs)| {
            attrs.map(|a| vec![(f, a)]).unwrap_or(vec![]) // remove empty optional attrs
        })
        .collect::<Vec<(&syn::Field, SelectFieldAttributes)>>();

    let selected_fields = fields_with_attrs
        .iter()
        .filter(|(_, attrs)| attrs.selected_options)
        .collect::<Vec<&(&syn::Field, SelectFieldAttributes)>>();

    if selected_fields.len() > 1 {
        return Err(syn::Error::new(
            selected_fields[1].0.span(),
            "Must not mark more than one field as #[selected_option].",
        )
        .into());
    }

    let Some((selected_field, _)) = selected_fields.first() else {
        return Err(syn::Error::new(
            input.ident.span(),
            "One field must be marked as #[selected_option], which provides all options.",
        )
        .into());
    };

    let select_spec = create_select_spec(&select_attrs, selected_field, &data_type);
    let create_with_interaction = create_build_with_interaction(&fields_with_attrs)?;
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
                let __buildables = <Self as ::minirustbot_forms::Subcomponent<#ctx_data, #ctx_error, #data_type>>::generate_buildables(context, data).await?;
                let __custom_ids = ::minirustbot_forms::util::id_vec_from_has_custom_ids(&__buildables);
                let __reply = <Self as ::minirustbot_forms::GenerateReply<#ctx_data, #ctx_error, #data_type>>::create_reply(context, data).await?;
                context.send(|f|
                    <::minirustbot_forms::ReplySpec as ::minirustbot_forms::Buildable<::poise::CreateReply>>::on_build(&__reply, f)
                        .components(|f| f
                            .create_action_row(|mut f| {
                                for buildable in &__buildables {
                                    f = f.create_select(|b| ::minirustbot_forms::Buildable::<::poise::serenity_prelude::CreateSelectMenu>::on_build(buildable, b));
                                }
                                f
                            }))).await?;

                Ok(__custom_ids.into_iter().map(::core::clone::Clone::clone).collect())
            }

            async fn on_response(
                context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
                interaction: ::std::sync::Arc<::poise::serenity_prelude::MessageComponentInteraction>,
                data: &mut #data_type,
            ) -> ::minirustbot_forms::error::Result<::core::option::Option<::std::boxed::Box<Self>>> {
                ::core::result::Result::Ok(::core::option::Option::Some(
                    <Self as ::minirustbot_forms::Subcomponent<#ctx_data, #ctx_error, #data_type>>::build_from_interaction(interaction).await?
                ))
            }
        }
    }.into())
}

fn validate_select_attrs(
    select_attrs: &SelectAttributes,
    input: &syn::DeriveInput,
) -> Result<(), darling::Error> {
    if select_attrs.disabled && select_attrs.disabled_function.is_some() {
        return Err(syn::Error::new(
            input.ident.span(),
            "Cannot specify #[disabled] and #[disabled_function] at the same time.",
        )
        .into());
    }

    Ok(())
}

fn create_build_with_interaction(
    fields_with_attrs: &Vec<(&syn::Field, SelectFieldAttributes)>,
) -> Result<TokenStream2, darling::Error> {
    let mut field_initializers = vec![];
    let mut any_field_non_initialized = false;
    for (field, attrs) in fields_with_attrs {
        let field_ident = field.ident.as_ref().expect("Expected named field");

        if attrs.interaction {
            field_initializers.push(quote! { #field_ident: interaction.into(), });
        } else if attrs.selected_options {
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

fn create_select_spec(
    select_attrs: &SelectAttributes,
    selected_field: &syn::Field,
    data: &syn::Type,
) -> TokenStream2 {
    let custom_id = util::wrap_option_into(&select_attrs.custom_id);
    let min_values = util::wrap_option_into(&select_attrs.min_values);
    let max_values = util::wrap_option_into(&select_attrs.max_values);

    let selected_field_type = &selected_field.ty;
    let options = quote! {
        <#selected_field_type as ::minirustbot_forms::SelectMenuOptionSpec>::generate_options(
            context, data)
    };

    let disabled = if let Some(disabled_function) = select_attrs.disabled_function.as_ref() {
        quote! { #disabled_function(context, data).await?.into() }
    } else {
        let disabled = select_attrs.disabled;
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
