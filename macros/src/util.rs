mod darling_types;
pub use darling_types::*;

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
        .map(|attr| attr.parse_meta().map(syn::NestedMeta::Meta))
        .collect::<Result<Vec<_>, _>>()?;

    <T as darling::FromMeta>::from_list(&mapped_attrs)
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
    pub(crate) use take_attribute_or_its_function_optional;
    pub(crate) use take_attribute_or_its_function_required;
}
