//! Implements the #[derive(ModalFormComponent)] derive macro
use crate::util;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Representation of the struct attributes
#[derive(Debug, darling::FromMeta)]
#[darling(allow_unknown_fields)]
struct StructAttributes {
    // Path to the Form object holding this Modal.
    form: syn::Path
}

pub fn modal_component(input: syn::DeriveInput) -> Result<TokenStream, darling::Error> {
    let struct_attrs = input
        .attrs
        .iter()
        .map(|attr| attr.parse_meta().map(syn::NestedMeta::Meta))
        .collect::<Result<Vec<_>, _>>()?;

    let struct_attrs = <StructAttributes as darling::FromMeta>::from_list(&struct_attrs)?;
    let form = struct_attrs.form;

    let struct_ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        #[::async_trait::async_trait]
        impl #impl_generics ModalFormComponent for #struct_ident #ty_generics #where_clause {
            type Modal = Self;
            type Form = #form;
        }
    }.into())
}
