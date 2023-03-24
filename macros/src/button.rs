//! Implements the #[derive(ButtonComponent)] derive macro
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::spanned::Spanned;

use crate::util::{
    self,
    macros::{
        take_attribute_optional, take_attribute_or_its_function_optional,
        take_attribute_or_its_function_required,
    },
};

/// Representation of the struct attributes
#[derive(Debug, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
struct ButtonAttributes {
    /// Type of the Data object to be passed between components.
    /// By default, ()
    data: Option<syn::Type>,

    /// Context's Data type.
    ctx_data: syn::Type,

    /// Context's Error type.
    ctx_error: syn::Type,

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

#[derive(Debug, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
struct InteractionAttr {
    /// Marks this field as the receiver of the returned MessageComponentInteraction
    /// object.
    interaction: Option<()>,

    /// The default value of this field when instantiated.
    initializer: Option<syn::Expr>,
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

    let ctx_data = &struct_attrs.ctx_data;
    let ctx_error = &struct_attrs.ctx_error;

    let button_spec = create_button_spec(&struct_attrs, &data_type);
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
                data: &mut #data_type,
            ) -> ::minirustbot_forms::error::Result<::std::vec::Vec<Self::ReturnedBuildable>> {
                Ok(::std::vec![ #button_spec ])
            }

            async fn build_from_interaction(
                context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
                interaction: ::std::sync::Arc<::poise::serenity_prelude::MessageComponentInteraction>,
                data: &mut #data_type,
            ) -> ::minirustbot_forms::error::Result<::std::boxed::Box<Self>> {
                ::core::result::Result::Ok(::std::boxed::Box::new(#create_with_interaction))
            }
        }

        // impl #impl_generics ::minirustbot_forms::Buildable<>

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
                                    f = f.create_button(|b| ::minirustbot_forms::Buildable::<::poise::serenity_prelude::CreateButton>::on_build(buildable, b));
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
                    <Self as ::minirustbot_forms::Subcomponent<#ctx_data, #ctx_error, #data_type>>::build_from_interaction(context, interaction, data)
                        .await?
                ))
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
        syn::Data::Struct(data_struct) => match data_struct {
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
                    if attrs.interaction.is_some() {
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

fn create_button_spec(button_attrs: &ButtonAttributes, _data: &syn::Type) -> TokenStream2 {
    let label = take_attribute_or_its_function_required!(button_attrs; label, label_function);
    let custom_id = take_attribute_optional!(button_attrs; custom_id);
    let link = take_attribute_or_its_function_optional!(button_attrs; link, link_function);
    let emoji = take_attribute_or_its_function_optional!(button_attrs; emoji, emoji_function);

    let disabled = if let Some(disabled_function) = &button_attrs.disabled_function {
        quote! { #disabled_function(context, data).into() }
    } else {
        let disabled = button_attrs.disabled.is_some();
        quote! { #disabled.into() }
    };

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
        ::minirustbot_forms::ButtonSpec {
            label: #label,
            custom_id: #custom_id,
            style: #style,
            link: #link,
            emoji: #emoji,
            disabled: #disabled,
        }
    }
}
