//! Implements the #[derive(InteractionForm)] derive macro
use crate::util;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Representation of the struct attributes
#[derive(Debug, Default, darling::FromMeta)]
#[darling(allow_unknown_fields, default)]
struct StructAttributes {
    /// Type of the Data object to be passed to components.
    /// By default, ()
    data: Option<syn::Type>,

    /// name of the function to run when the form is finished.
    /// It must take a single ApplicationContext object.
    on_finish: Option<syn::Path>,
}

/// Representation of the struct field attributes
#[derive(Debug, Default, darling::FromMeta)]
#[darling(allow_unknown_fields, default)]
struct FieldAttributes {
    /// Indicates this field either is a ModalFormComponent, or specify the ModalFormComponent struct manually
    modal: Option<()>,

    /// Indicates this field will store a MessageFormComponent (and assumes it will assign itself to it)
    component: Option<()>,
}

pub fn form(input: syn::DeriveInput) -> Result<TokenStream, darling::Error> {
    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => fields.named,
        _ => {
            return Err(syn::Error::new(
                // use Darling errors to indicate visually where the error occurred
                input.ident.span(), // <-- Error will display at the struct's name ('ident'/identity)
                "Only structs with named fields can be used for deriving a Form.",
            )
            .into());
        }
    };

    let struct_attrs = input
        .attrs
        .iter()
        .map(|attr| attr.parse_meta().map(syn::NestedMeta::Meta))
        .collect::<Result<Vec<_>, _>>()?;

    let struct_attrs = <StructAttributes as darling::FromMeta>::from_list(&struct_attrs)?;

    let data_type = struct_attrs
        .data
        .clone()
        .unwrap_or(util::empty_tuple_type());
    //  ^^^^^^^^^^ default to ()

    let mut components = Vec::new();
    let mut create_fields = Vec::new();
    let mut modal_creation: Option<TokenStream2> = None;

    for field in fields {
        let field_name: &syn::Ident = &field.ident.as_ref().expect("Unnamed field");
        // Extract data from syn::Field
        let field_attrs: FieldAttributes = util::get_darling_attrs(&field.attrs)?;

        let field_type: &syn::Type = &field.ty;

        let field_inner_type =
            util::extract_type_parameter("Option", field_type).unwrap_or(field_type);

        if field_attrs.component.is_some() {
            // is a message component
            components.push(generate_message_component(
                field_name,
                field_type,
                field_inner_type,
                &data_type,
            ));
            create_fields.push(quote! { #field_name });
        } else if field_attrs.modal.is_some() {
            if modal_creation.is_some() {
                return Err(syn::Error::new(
                    syn::spanned::Spanned::span(field_name),
                    "Multiple #[modal] are not allowed.",
                )
                .into());
            }

            modal_creation = Some(generate_modal_creation(
                field_name,
                /*modal_type:*/ field_type,
                /*modal_inner_type:*/ field_inner_type,
                &data_type,
            ));
            create_fields.push(quote! { #field_name });
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
        impl #impl_generics crate::form::InteractionForm for #struct_ident #ty_generics #where_clause {

            async fn run_components(context: crate::common::ApplicationContext<'_>) -> crate::error::Result<::std::boxed::Box<Self>> {
                let mut __component_data: #data_type = ::std::default::Default::default();
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
) -> TokenStream2 {
    // use .into() in case it's an Option<>, Box<> etc.
    quote! {
        let #field_name: #field_type = (*<#field_inner_type as crate::form::MessageFormComponent<#data_type>>::run(context, &mut __component_data).await?).into();
    }
}

fn generate_modal_creation(
    modal_field_name: &syn::Ident,
    modal_type: &syn::Type,
    modal_inner_type: &syn::Type,
    data_type: &syn::Type,
) -> TokenStream2 {
    quote! {
        let #modal_field_name: #modal_type = (*<#modal_inner_type as crate::form::ModalFormComponent<#data_type>>::run(context, &mut __component_data).await?).into();
    }
}

fn parse_on_finish(struct_attrs: &StructAttributes) -> Option<TokenStream2> {
    if let Some(on_finish) = &struct_attrs.on_finish {
        Some(quote! {
            async fn on_finish(self, context: crate::common::ApplicationContext<'_>) -> crate::error::Result<::std::boxed::Box<Self>> {
                #on_finish(context).into()
            }
        })
    } else {
        None
    }
}
