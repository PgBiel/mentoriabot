mod darling_types;

pub use darling_types::*;
use proc_macro2::Span;

/// Converts a string to ident (which can be inserted without quotes)
pub fn string_to_ident(s: &str) -> syn::Ident {
    syn::Ident::new(s, Span::call_site())
}

pub fn empty_tuple_type() -> syn::Type {
    syn::Type::Tuple(syn::TypeTuple {
        paren_token: Default::default(),
        elems: Default::default(),
    })
}

pub fn get_darling_attrs<T: darling::FromMeta>(
    attrs: &[syn::Attribute],
) -> Result<T, darling::Error> {
    let mapped_attrs = attrs
        .iter()
        .map(|attr| darling::ast::NestedMeta::Meta(attr.meta.clone()))
        .collect::<Vec<_>>();

    darling::FromMeta::from_list(&mapped_attrs)
}

pub fn get_darling_attrs_ref<T: darling::FromMeta>(
    attrs: &[&syn::Attribute],
) -> Result<T, darling::Error> {
    let mapped_attrs = attrs
        .iter()
        .map(|attr| darling::ast::NestedMeta::Meta(attr.meta.clone()))
        .collect::<Vec<_>>();

    darling::FromMeta::from_list(&mapped_attrs)
}

pub fn get_darling_attrs_filtered<T: darling::FromMeta>(
    attrs: &[syn::Attribute],
    filter_by: &[&str],
) -> Result<T, darling::Error> {
    get_darling_attrs_ref(
        &attrs
            .iter()
            .filter(|attr| {
                attr.path()
                    .segments
                    .first()
                    .map(|p| filter_by.contains(&&*p.ident.to_string()))
                    .unwrap_or(false)
            })
            .collect::<Vec<_>>(),
    )
}

// From poise

// transforms a type of form `OuterType<T>` into `T`
pub fn extract_type_parameter<'a>(outer_type: &str, t: &'a syn::Type) -> Option<&'a syn::Type> {
    if let syn::Type::Path(path) = t {
        if path.path.segments.len() == 1 {
            let path = &path.path.segments[0];
            if path.ident == outer_type {
                if let syn::PathArguments::AngleBracketed(generics) = &path.arguments {
                    if generics.args.len() == 1 {
                        if let syn::GenericArgument::Type(t) = &generics.args[0] {
                            return Some(t);
                        }
                    }
                }
            }
        }
    }
    None
}

/// Converts None => `None` and Some(x) => `Some(#x)`
#[allow(dead_code)]
pub fn wrap_option<T: quote::ToTokens>(literal: &Option<T>) -> syn::Expr {
    match literal {
        Some(literal) => syn::parse_quote! { Some(#literal) },
        None => syn::parse_quote! { None },
    }
}

/// Converts None => `None` and Some(x) => `Some(#x.into())`
pub fn wrap_option_into<T: quote::ToTokens>(literal: &Option<T>) -> syn::Expr {
    match literal {
        Some(literal) => syn::parse_quote! { Some(#literal.into()) },
        None => syn::parse_quote! { None },
    }
}

/// Converts None => `None` and Some(x) => `Some(Box::new(#x))`
pub fn wrap_option_box<T: quote::ToTokens>(literal: &Option<T>) -> syn::Expr {
    match literal {
        Some(literal) => syn::parse_quote! { Some(::std::boxed::Box::new(#literal)) },
        None => syn::parse_quote! { None },
    }
}

/// Utility macros for macro impls
pub(crate) mod macros {
    macro_rules! take_attribute_optional {
        ($attrs:expr; $attr_name:ident) => {
            if let Some($attr_name) = $attrs.$attr_name.as_ref() {
                quote! { #$attr_name.into() }
            } else {
                quote! { Default::default() }
            }
        };
    }

    #[allow(unused_macros)]
    macro_rules! take_attribute_or_its_function_required {
        ($attrs:expr; $attr_name:ident, $func_name:ident) => {
            if let Some($func_name) = $attrs.$func_name.as_ref() {
                quote! { #$func_name(context, data).into() }
            } else if let Some($attr_name) = $attrs.$attr_name.as_ref() {
                quote! { #$attr_name.into() }
            } else {
                panic!("Must specify one of #[$attr_name] or #[$func_name]")
            }
        };
    }

    #[allow(unused_macros)]
    macro_rules! take_attribute_or_its_function_optional {
        ($attrs:expr; $attr_name:ident, $func_name:ident) => {{
            if let Some($func_name) = $attrs.$func_name.as_ref() {
                quote! { #$func_name(context, data).into() }
            } else if let Some($attr_name) = $attrs.$attr_name.as_ref() {
                quote! { #$attr_name.into() }
            } else {
                quote! { Default::default() }
            }
        }};

        ($attrs:expr; $attr_name:ident, $func_name:ident; $or:expr) => {{
            if let Some($func_name) = $attrs.$func_name.as_ref() {
                quote! { #$func_name(context, data).into() }
            } else if let Some($attr_name) = $attrs.$attr_name.as_ref() {
                quote! { #$attr_name.into() }
            } else {
                quote! { $or }
            }
        }};
    }

    pub(crate) use take_attribute_optional;
    #[allow(unused_imports)]
    pub(crate) use take_attribute_or_its_function_optional;
    #[allow(unused_imports)]
    pub(crate) use take_attribute_or_its_function_required;
}
