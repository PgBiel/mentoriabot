//! Implements the #[derive(ButtonComponent)] derive macro
use darling::util::Flag;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::spanned::Spanned;

use crate::{
    common::{FormContextInfo, FormData},
    model::{ButtonSpecRepr, ToSpec, ValidateAttrs},
    util,
};

#[derive(Debug, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
struct ButtonBaseAttributes {
    /// Form type parameters
    form_data: FormData,

    /// Button attributes
    button: ButtonSpecRepr,
}

#[derive(Debug, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
struct InteractionAttr {
    /// Marks this field as the receiver of the returned MessageComponentInteraction
    /// object.
    interaction: Flag,

    /// The default value of this field when instantiated.
    initializer: Option<syn::Expr>,
}

pub fn button(input: syn::DeriveInput) -> Result<TokenStream, darling::Error> {
    let struct_attrs: ButtonBaseAttributes = util::get_darling_attrs(&input.attrs)?;
    let button_attrs = &struct_attrs.button;

    button_attrs.validate_attrs()?;

    // ---
    let FormData {
        data: data_type,
        ctx: FormContextInfo {
            data: ctx_data,
            error: ctx_error,
        },
    } = &struct_attrs.form_data;

    let button_spec = button_attrs.to_spec();
    let create_with_interaction = single_button_create_with_interaction_code(&input)?
        .unwrap_or_else(|| quote! { Default::default() });

    let struct_ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        #[::async_trait::async_trait]
        impl #impl_generics ::minirustbot_forms::Subcomponent<#ctx_data, #ctx_error, #data_type> for #struct_ident #ty_generics #where_clause {
            type BuilderType = ::poise::serenity_prelude::CreateButton;
            type ReturnedBuildable = ::minirustbot_forms::ButtonSpec;
            async fn generate_buildables(
                context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
                data: &mut ::minirustbot_forms::FormState<#data_type>,
            ) -> ::minirustbot_forms::error::ContextualResult<::std::vec::Vec<Self::ReturnedBuildable>, #ctx_error> {
                Ok(::std::vec![ #button_spec ])
            }

            async fn build_from_interaction(
                context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
                interaction: ::std::sync::Arc<::poise::serenity_prelude::MessageComponentInteraction>,
                data: &mut ::minirustbot_forms::FormState<#data_type>,
            ) -> ::minirustbot_forms::error::ContextualResult<::std::boxed::Box<Self>, #ctx_error> {
                ::core::result::Result::Ok(::std::boxed::Box::new(#create_with_interaction))
            }
        }

        // impl #impl_generics ::minirustbot_forms::Buildable<>

        #[::async_trait::async_trait]
        impl #impl_generics ::minirustbot_forms::MessageFormComponent<#ctx_data, #ctx_error, #data_type> for #struct_ident #ty_generics #where_clause {
            async fn send_component(
                context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
                data: &mut ::minirustbot_forms::FormState<#data_type>,
            ) -> ::minirustbot_forms::error::ContextualResult<::std::vec::Vec<::minirustbot_forms::interaction::CustomId>, #ctx_error> {

                let __buildables = <Self as ::minirustbot_forms::Subcomponent<#ctx_data, #ctx_error, #data_type>>::generate_buildables(context, data).await?;
                let __custom_ids = ::minirustbot_forms::util::id_vec_from_has_custom_ids(&__buildables);

                let __reply = <Self as ::minirustbot_forms::GenerateReply<#ctx_data, #ctx_error, #data_type>>::create_reply(context, data).await?;

                context.send(|f|
                    <::minirustbot_forms::ReplySpec as ::minirustbot_forms::Buildable<::poise::CreateReply>>::on_build(&__reply, f)
                        .components(|f| f
                            .create_action_row(|mut f| {
                                for buildable in &__buildables {
                                    f = f.create_button(|b| ::minirustbot_forms::Buildable::<::poise::serenity_prelude::CreateButton>::on_build(buildable, b));
                                }
                                f
                            }))).await?;

                Ok(__custom_ids.into_iter().map(::core::clone::Clone::clone).collect())
            }

            async fn on_response(
                context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
                interaction: ::std::sync::Arc<::poise::serenity_prelude::MessageComponentInteraction>,
                data: &mut ::minirustbot_forms::FormState<#data_type>,
            ) -> ::minirustbot_forms::error::ContextualResult<::core::option::Option<::std::boxed::Box<Self>>, #ctx_error> {
                ::core::result::Result::Ok(::core::option::Option::Some(
                    <Self as ::minirustbot_forms::Subcomponent<#ctx_data, #ctx_error, #data_type>>::build_from_interaction(context, interaction, data)
                        .await?
                ))
            }
        }
    }.into())
}

fn single_button_create_with_interaction_code(
    input: &syn::DeriveInput,
) -> Result<Option<TokenStream2>, darling::Error> {
    let mut result: Option<TokenStream2> = None;
    match &input.data {
        syn::Data::Struct(data_struct) => match data_struct {
            syn::DataStruct {
                fields: syn::Fields::Named(fields),
                ..
            } => {
                let len = fields.named.len();
                for field in &fields.named {
                    let attrs: InteractionAttr = util::get_darling_attrs(&field.attrs)?;
                    if attrs.interaction.is_present() {
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
                    if attrs.interaction.is_present() {
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
            } => {
                result = Some(quote! { Self });
            }
        },
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let mut fallback = quote! { Default::default() }; // as a last resort
            for variant in variants {
                let variant_name = &variant.ident;
                let mut is_named = false;
                let fields = match &variant.fields {
                    syn::Fields::Named(fields) => {
                        is_named = true;
                        &fields.named
                    }
                    syn::Fields::Unnamed(fields) => &fields.unnamed,
                    syn::Fields::Unit => {
                        fallback = quote! { Self::#variant_name };
                        continue;
                    }
                };

                let mut field_initializers = Vec::new();
                let mut interaction_seen = false;
                let mut non_default_field = None;
                for field in fields {
                    let attrs: InteractionAttr = util::get_darling_attrs(&field.attrs)?;
                    if attrs.interaction.is_present() {
                        if interaction_seen {
                            return Err(syn::Error::new(
                                field.ident.as_ref().map(|i| i.span()).unwrap_or_else(|| field.span()),
                                "Cannot have more than one field in the same variant with #[interaction]."
                            ).into());
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
                    } else if let Some(initializer) = attrs.initializer {
                        if is_named {
                            let field_name = field.ident.as_ref().expect("Expected named field");
                            field_initializers.push(quote! { #field_name: #initializer });
                        } else {
                            field_initializers.push(quote! { #initializer });
                        }
                    } else {
                        non_default_field = Some(field);
                        if interaction_seen {
                            break; // cannot mix #[interaction] with non-#[initializer] fields
                        }
                    };
                }

                if interaction_seen {
                    if let Some(non_default_field) = non_default_field {
                        return Err(syn::Error::new(
                            non_default_field.ident.as_ref().map(|i| i.span()).unwrap_or_else(|| non_default_field.span()),
                            "Cannot specify #[interaction] for a field and not specify #[initializer = \"expr\"] on the others."
                        ).into());
                    }
                    result = if is_named {
                        Some(quote! { Self::#variant_name {
                            #( #field_initializers ),*
                        } })
                    } else {
                        Some(quote! { Self::#variant_name( #( #field_initializers ),* ) })
                    };
                    break;
                }
            }

            if result.is_none() {
                result = Some(fallback);
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
