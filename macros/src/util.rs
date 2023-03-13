pub fn empty_tuple_type() -> syn::Type {
    syn::Type::Tuple(syn::TypeTuple {
        paren_token: Default::default(),
        elems: Default::default(),
    })
}

pub fn get_darling_attrs<T: darling::FromMeta>(
    attrs: &Vec<syn::Attribute>,
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

/// Syn Fold to make all lifetimes 'static. Used to access trait items of a type without having its
/// concrete lifetime available
pub struct AllLifetimesToStatic;
impl syn::fold::Fold for AllLifetimesToStatic {
    fn fold_lifetime(&mut self, _: syn::Lifetime) -> syn::Lifetime {
        syn::parse_quote! { 'static }
    }
}

/// Darling utility type that accepts a list of things, e.g. `#[attr(thing1, thing2...)]`
#[derive(Debug)]
pub struct List<T>(pub Vec<T>);
impl<T: darling::FromMeta> darling::FromMeta for List<T> {
    fn from_list(items: &[::syn::NestedMeta]) -> darling::Result<Self> {
        items
            .iter()
            .map(|item| T::from_nested_meta(item))
            .collect::<darling::Result<Vec<T>>>()
            .map(Self)
    }
}
impl<T> Default for List<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

/// Darling utility type that accepts a 2-tuple list of things, e.g. `#[attr(thing1, thing2)]`
#[derive(Debug)]
pub struct Tuple2<T>(pub T, pub T);
impl<T: darling::FromMeta> darling::FromMeta for Tuple2<T> {
    fn from_list(items: &[::syn::NestedMeta]) -> darling::Result<Self> {
        Ok(match items {
            [a, b] => Self(T::from_nested_meta(a)?, T::from_nested_meta(b)?),
            _ => {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "expected two items `(\"a\", \"b\")`",
                )
                .into())
            }
        })
    }
}

pub fn vec_tuple_2_to_hash_map(v: Vec<Tuple2<String>>) -> proc_macro2::TokenStream {
    let (keys, values): (Vec<String>, Vec<String>) = v.into_iter().map(|x| (x.0, x.1)).unzip();
    quote::quote! {
        std::collections::HashMap::from([
            #( (#keys.to_string(), #values.to_string()) ),*
        ])
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
                quote! { #$func_name(context, data).await?.into() }
            } else if let Some($attr_name) = $attrs.$attr_name.as_ref() {
                quote! { #$attr_name.into() }
            } else {
                panic!("Must specify one of #[$attr_name] or #[$func_name]")
            }
        };
    }

    macro_rules! take_attribute_or_its_function_optional {
        ($attrs:expr; $attr_name:ident, $func_name:ident) => {
            if let Some($func_name) = $attrs.$func_name.as_ref() {
                quote! { #$func_name(context, data).await?.into() }
            } else if let Some($attr_name) = $attrs.$attr_name.as_ref() {
                quote! { #$attr_name.into() }
            } else {
                quote! { Default::default() }
            }
        };
    }

    pub(crate) use take_attribute_optional;
    pub(crate) use take_attribute_or_its_function_optional;
    pub(crate) use take_attribute_or_its_function_required;
}
