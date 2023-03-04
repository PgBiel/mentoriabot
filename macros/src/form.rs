//! Implements the #[derive(InteractionForm)] derive macro
extern crate quote;
use crate::util;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Representation of the struct attributes
#[derive(Debug, Default, darling::FromMeta)]
#[darling(allow_unknown_fields, default)]
struct StructAttributes {
    // expression/function to run when the form is finished.
    // Use the variable 'context' to get the ApplicationContext object.
    on_finish: Option<String>
}

/// Representation of the struct field attributes
#[derive(Debug, Default, darling::FromMeta)]
#[darling(allow_unknown_fields, default)]
struct FieldAttributes {
    modal: Option<darling::util::PathList>,

    /// Indicates this field will store a MessageFormComponent (and assumes it will assign itself to it)
    message_component: Option<()>
}

pub fn form(input: syn::DeriveInput, is_modal: bool) -> Result<TokenStream, darling::Error> {
    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => fields.named,
        _ => {
            return Err(syn::Error::new(
                input.ident.span(),
                "Only structs with named fields can be used for choice parameters",
            )
            .into())
        }
    };

    let struct_attrs = input
        .attrs
        .iter()
        .map(|attr| attr.parse_meta().map(syn::NestedMeta::Meta))
        .collect::<Result<Vec<_>, _>>()?;

    let struct_attrs = <StructAttributes as darling::FromMeta>::from_list(&struct_attrs)?;

    let mut components = Vec::new();
    let mut modal_form_body_vec = Vec::new();

    for field in fields {
        // Extract data from syn::Field
        let field_attrs: FieldAttributes = util::get_field_attrs(&field)?;

        // Option<T> => T
        // T => T
        let field_inner_type: &syn::Type = util::extract_type_parameter("Option", &field.ty).unwrap_or(&field.ty);

        if field_attrs.message_component.is_some() {  // is a message component
            components.push(generate_message_component(field_inner_type)?);
        } else if let Some(modal_component_path) = field_attrs.modal {
            if !is_modal {
                panic!("Cannot specify a #[modal] for the InteractionForm trait; please derive InteractionModalForm instead.");
            }

            modal_form_body_vec.push(generate_modal_form_body(field_inner_type, modal_component_path)?)
        }
    }

    if is_modal && modal_form_body_vec.is_empty() {
        panic!("Please specify #[modal(ModalComponentType)] for a modal field in the form, or derive InteractionForm instead.");
    }

    // Empty vec => no on_finish (keep default impl)
    // One element => override
    let mut on_finish_vec = Vec::new();

    // '#[on_finish="function()"]' was given
    if let Some(on_finish) = parse_on_finish(&struct_attrs) {
        on_finish_vec.push(on_finish);
    }

    let struct_ident = input.ident;  // struct's name as an object
    
    let form_trait = if is_modal { quote! { InteractionModalForm } } else { quote! { InteractionForm } };

    // get the struct's generics
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! { const _: () = {
        #[::async_trait::async_trait]
        impl #impl_generics crate::form::#form_trait for #struct_ident #ty_generics #where_clause {
            #( #modal_form_body_vec )*

            fn components() -> ::std::vec::Vec<crate::form::FormComponent<Self>> {
                ::std::vec![
                    #( #components ),*
                ]
            }

            #( #on_finish_vec )*
        }
    }; }
    .into())
}

fn generate_message_component(field_inner_type: &syn::Type) -> Result<TokenStream2, darling::Error> {
    Ok(quote! {
        crate::form::FormComponent::Message(::std::boxed::Box::new(#field_inner_type::default()))
    })
}

fn generate_modal_form_body(
    modal_type: &syn::Type,
    modal_component_path: darling::util::PathList
) -> Result<TokenStream2, darling::Error> {
    let modal_component_path = util::pathlist_to_string(modal_component_path);
    let modal_component_path: TokenStream2 = modal_component_path.parse().expect("Could not parse Modal Component type");

    Ok(quote! {
        type Modal = #modal_type;

        fn modal() -> crate::form::ModalFormComponentBox<#modal_type, Self> {
            ::std::boxed::Box::new(#modal_component_path::default())
        }
    })
}

fn parse_on_finish(struct_attrs: &StructAttributes) -> Option<TokenStream2> {
    if let Some(on_finish) = &struct_attrs.on_finish {
        let on_finish: TokenStream2 = on_finish.parse().expect("Invalid Rust code given for 'on_finish' from derivation code.");

        Some(quote! {
            async fn on_finish(self, context: crate::common::ApplicationContext<'_>) -> crate::error::Result<Self> {
                #on_finish
            }
        })
    } else {
        None
    }
}
