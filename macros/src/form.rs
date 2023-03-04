//! Implements the #[derive(InteractionForm)] derive macro
use crate::util::{self, wrap_option};
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
    /// Indicates this field either is a ModalFormComponent, or specify the ModalFormComponent struct manually
    modal: Option<darling::util::Override<darling::util::PathList>>,

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
            return Err(syn::Error::new(  // use Darling errors to indicate visually where the error occurred
                input.ident.span(),  // <-- Error will display at the struct's name ('ident'/identity)
                "Only structs with named fields can be used for deriving a Form.",
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
    let mut modal_form_body: Option<TokenStream2> = None;

    for field in fields {
        let field_name: &syn::Ident = &field.ident.clone().expect("Unnamed field");
        // Extract data from syn::Field
        let field_attrs: FieldAttributes = util::get_field_attrs(&field)?;

        let field_inner_type_opt = util::extract_type_parameter("Option", &field.ty);

        // Option<T> => T
        // T => T
        let field_inner_type: &syn::Type = field_inner_type_opt.unwrap_or(&field.ty);

        if field_attrs.message_component.is_some() {  // is a message component
            components.push(generate_message_component(field_inner_type)?);
        } else if let Some(modal_component_path) = field_attrs.modal {
            if !is_modal {
                return Err(syn::Error::new(
                    syn::spanned::Spanned::span(&field.ident),
                    "Cannot specify a #[modal] for the InteractionForm trait; please derive InteractionModalForm instead.",
                )
                .into());
            }

            modal_form_body = Some(generate_modal_form_body(
                field_name,
                /*modal_type:*/field_inner_type,
                match modal_component_path {
                    darling::util::Override::Explicit(path) => util::pathlist_to_string(path),
                    darling::util::Override::Inherit => quote!(#field_inner_type).to_string()  // assume the field type is the Component
                }
            )?);
        }
    }

    if is_modal && modal_form_body.is_none() {
        return Err(syn::Error::new(
            input.ident.span(),
            "Please specify #[modal] or #[modal(ModalComponentType)] for a modal field in the form, or derive InteractionForm instead.",
        )
        .into());
    }

    // for '#[on_finish="function()"]'
    let on_finish = parse_on_finish(&struct_attrs);

    let struct_ident = input.ident;  // struct's name as an object
    
    let form_trait = if is_modal { quote! { InteractionModalForm } } else { quote! { InteractionForm } };

    // get the struct's generics
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! { const _: () = {
        #[::async_trait::async_trait]
        impl #impl_generics crate::form::#form_trait for #struct_ident #ty_generics #where_clause {
            #modal_form_body

            fn components() -> ::std::vec::Vec<crate::form::FormComponent<Self>> {
                ::std::vec![
                    #( #components ),*
                ]
            }

            #on_finish
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
    modal_field_name: &syn::Ident,
    modal_type: &syn::Type,
    modal_component_path: String
) -> Result<TokenStream2, darling::Error> {
    let modal_component_path: TokenStream2 = modal_component_path.parse().expect("Could not parse Modal Component type");
    Ok(quote! {
        type Modal = #modal_type;
        type ModalComponent = #modal_component_path;

        fn set_modal(mut self, modal: Self::Modal) -> Self {
            self.#modal_field_name = modal.into();
            self
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
