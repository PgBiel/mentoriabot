//! Implements the #[derive(InteractionForm)] derive macro
use darling::{util::Flag, FromAttributes, FromDeriveInput};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::{
    common::{FormContextInfo, FormData, FormDataAttr},
    util::{self, parse_option},
};

/// Representation of the struct attributes
#[derive(Debug, darling::FromDeriveInput)]
#[darling(attributes(forms))]
struct StructAttributes {
    /// name of the function to run when the form is finished.
    /// It must take a single ApplicationContext object.
    #[darling(map = "parse_option")]
    on_finish: Option<syn::Path>,
}

/// Representation of the struct field attributes
#[derive(Debug, Default, darling::FromMeta)]
#[darling(allow_unknown_fields, default)]
struct FieldAttributes {
    /// Indicates this field either is a ModalFormComponent, or specify the ModalFormComponent
    /// struct manually
    modal: Flag,

    /// Indicates this field will store a MessageFormComponent (and assumes it will assign itself
    /// to it)
    component: Flag,

    /// Indicates this field will be taken from the corresponding form data field.
    from_data_field: Option<syn::Ident>,
}

pub fn form(input: syn::DeriveInput) -> Result<TokenStream, darling::Error> {
    let struct_attrs = StructAttributes::from_derive_input(&input)?;
    let form_data = FormDataAttr::from_attributes(&input.attrs)?;
    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => fields.named,
        _ => {
            return Err(syn::Error::new(
                // use Darling errors to indicate visually where the error occurred
                input.ident.span(), /* <-- Error will display at the struct's name
                                     * ('ident'/identity) */
                "Only structs with named fields can be used for deriving a Form.",
            )
            .into());
        }
    };

    let FormData {
        data: data_type,
        ctx: FormContextInfo {
            data: ctx_data,
            error: ctx_error,
        },
    } = &form_data.form_data;

    let mut components = Vec::new();
    let mut create_fields = Vec::new();
    let mut modal_creation: Option<TokenStream2> = None;

    for field in fields {
        let field_name: &syn::Ident = field.ident.as_ref().expect("Unnamed field");
        // Extract data from syn::Field
        let field_attrs: FieldAttributes = util::get_darling_attrs(&field.attrs)?;

        let field_type: &syn::Type = &field.ty;

        let field_inner_type =
            util::extract_type_parameter("Option", field_type).unwrap_or(field_type);

        if field_attrs.component.is_present() {
            // is a message component
            components.push(generate_message_component(
                field_name,
                field_type,
                field_inner_type,
                data_type,
                ctx_data,
                ctx_error,
            ));
            create_fields.push(quote! { #field_name });
        } else if field_attrs.modal.is_present() {
            if modal_creation.is_some() {
                return Err(syn::Error::new(
                    syn::spanned::Spanned::span(field_name),
                    "Multiple #[modal] are not allowed.",
                )
                .into());
            }

            modal_creation = Some(generate_modal_creation(
                field_name,
                /* modal_type: */ field_type,
                /* modal_inner_type: */ field_inner_type,
                data_type,
                ctx_data,
                ctx_error,
            ));
            create_fields.push(quote! { #field_name });
        } else if let Some(data_field) = field_attrs.from_data_field {
            create_fields.push(quote! { #field_name: __component_data.data.#data_field.into() })
        } else {
            create_fields.push(quote! { #field_name: Default::default() });
        }
    }

    // for '#[on_finish = function_name]'
    let on_finish = parse_on_finish(&struct_attrs);

    let struct_ident = input.ident; // struct's name as an object

    // get the struct's generics
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! { const _: () = {
        #[::async_trait::async_trait]
        impl #impl_generics ::mentoriabot_forms::InteractionForm for #struct_ident #ty_generics #where_clause {
            type ContextData = #ctx_data;
            type ContextError = #ctx_error;
            type FormData = #data_type;

            async fn run_components(
                context: ::poise::ApplicationContext<'_, Self::ContextData, Self::ContextError>,
                form_data: Self::FormData
            ) -> ::mentoriabot_forms::error::ContextualResult<::std::boxed::Box<Self>, Self::ContextError> {
                let mut __component_data = ::mentoriabot_forms::FormState::new(form_data);
                #modal_creation
                #( #components )*
                Ok(Box::new(Self {
                    #( #create_fields ),*
                }))
            }

            #on_finish
        }
    }; }.into())
}

fn generate_message_component(
    field_name: &syn::Ident,
    field_type: &syn::Type,
    field_inner_type: &syn::Type,
    data_type: &syn::Type,
    ctx_data: &syn::Type,
    ctx_error: &syn::Type,
) -> TokenStream2 {
    // use .into() in case it's an Option<>, Box<> etc.
    quote! {
        let #field_name: #field_type = (*<#field_inner_type as ::mentoriabot_forms::MessageFormComponent<#ctx_data, #ctx_error, #data_type>>::run(context, &mut __component_data).await?).into();
    }
}

fn generate_modal_creation(
    modal_field_name: &syn::Ident,
    modal_type: &syn::Type,
    modal_inner_type: &syn::Type,
    data_type: &syn::Type,
    ctx_data: &syn::Type,
    ctx_error: &syn::Type,
) -> TokenStream2 {
    quote! {
        let #modal_field_name: #modal_type = (*<#modal_inner_type as ::mentoriabot_forms::ModalFormComponent<#ctx_data, #ctx_error, #data_type>>::run(context, &mut __component_data).await?).into();
    }
}

fn parse_on_finish(struct_attrs: &StructAttributes) -> Option<TokenStream2> {
    struct_attrs.on_finish.as_ref().map(|on_finish| quote! {
            async fn on_finish(self, context: ::poise::ApplicationContext<'_, Self::ContextData, Self::ContextError>) -> ::mentoriabot_forms::error::ContextualResult<::std::boxed::Box<Self>, Self::ContextError> {
                #on_finish(context).into()
            }
        })
}
