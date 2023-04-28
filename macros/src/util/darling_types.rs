//! Darling utility parsing types.

use quote::ToTokens;

/// Darling utility type for a parsed 1-tuple attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Parsed1<T>(pub T);

impl<T: syn::parse::Parse> darling::FromMeta for Parsed1<T> {
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        Ok(match items {
            [a] => Self(syn::parse2(a.into_token_stream())?),
            _ => {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "expected one single item for attribute",
                )
                .into())
            }
        })
    }
}

/// Darling utility type for a parsed 1-tuple optional attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct OptionParsed1<T>(pub Option<T>);

impl<T: syn::parse::Parse> darling::FromMeta for OptionParsed1<T> {
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        Ok(match items {
            [a] => Self(Some(syn::parse2::<T>(a.into_token_stream())?)),
            _ => Self(None),
        })
    }
}

/// Darling utility type for a parsed 2-tuple attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Parsed2<T, U = T>(pub T, pub U);

impl<T, U> darling::FromMeta for Parsed2<T, U>
where
    T: syn::parse::Parse,
    U: syn::parse::Parse,
{
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        Ok(match items {
            [a, b] => Self(
                syn::parse2(a.into_token_stream())?,
                syn::parse2(b.into_token_stream())?,
            ),
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

/// Utility function to add to `#[darling(map = ...)]`
/// to parse attribute arguments as `#[attr(argument_without_quotes)]`
/// instead of `#[attr = "argument_must_have_quotes_which_is_annoying"]`
pub(crate) fn parse<T>(parsed: Parsed1<T>) -> T {
    parsed.0
}

/// Same as [parse] but for optional attributes
pub(crate) fn parse_option<T>(parsed: OptionParsed1<T>) -> Option<T> {
    parsed.0
}

/// Same as [parse] but for tuples of two arguments
pub(crate) fn parse2<T, U, R: From<(T, U)>>(parsed: Parsed2<T, U>) -> R {
    (parsed.0, parsed.1).into()
}

/// Holds either a type or `Self`.
pub(crate) enum TypeOrSelf {
    Ty(syn::Type),
    SelfType(syn::Token![Self]),
}

impl From<syn::Type> for TypeOrSelf {
    fn from(value: syn::Type) -> Self {
        Self::Ty(value)
    }
}

impl From<syn::Token![Self]> for TypeOrSelf {
    fn from(value: syn::Token!(Self)) -> Self {
        Self::SelfType(value)
    }
}

/// Syn Fold to make all lifetimes 'static. Used to access trait items of a type without having its
/// concrete lifetime available
#[allow(dead_code)]
pub(crate) struct AllLifetimesToStatic;
impl syn::fold::Fold for AllLifetimesToStatic {
    fn fold_lifetime(&mut self, _: syn::Lifetime) -> syn::Lifetime {
        syn::parse_quote! { 'static }
    }
}

/// Darling utility type that accepts a list of things, e.g. `#[attr(thing1, thing2...)]`
#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct List<T>(pub Vec<T>);
impl<T: darling::FromMeta> darling::FromMeta for List<T> {
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tuple2<T, U = T>(pub T, pub U);
impl<T, U> darling::FromMeta for Tuple2<T, U>
where
    T: darling::FromMeta,
    U: darling::FromMeta,
{
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        Ok(match items {
            [a, b] => Self(T::from_nested_meta(a)?, U::from_nested_meta(b)?),
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

impl<T, U> From<Tuple2<T, U>> for (T, U) {
    fn from(value: Tuple2<T, U>) -> Self {
        (value.0, value.1)
    }
}

impl<T, U> From<(T, U)> for Tuple2<T, U> {
    fn from(value: (T, U)) -> Self {
        Tuple2(value.0, value.1)
    }
}

#[allow(dead_code)]
pub fn vec_tuple_2_to_hash_map(v: Vec<Tuple2<String>>) -> proc_macro2::TokenStream {
    let (keys, values): (Vec<String>, Vec<String>) = v.into_iter().map(|x| (x.0, x.1)).unzip();
    quote::quote! {
        std::collections::HashMap::from([
            #( (#keys.to_string(), #values.to_string()) ),*
        ])
    }
}
