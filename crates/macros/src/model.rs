mod button;
mod macros;
mod reply;
mod select;
mod select_option;

pub(crate) use button::ButtonSpecRepr;
use proc_macro2::TokenStream;
pub(crate) use reply::ReplySpecRepr;

#[allow(unused_imports)]
pub(crate) use select::SelectMenuSpecRepr;
#[allow(unused_imports)]
pub(crate) use select_option::SelectOptionSpecRepr;

/// Allows for validation of attributes in model structs
pub(crate) trait ValidateAttrs {
    fn validate_attrs(&self) -> Result<(), syn::Error>;
}

/// For attribute-parsing classes which are converted to a Spec constructor.
pub(crate) trait ToSpec {
    fn to_spec(&self) -> TokenStream;
}

macro_rules! impl_to_tokens_from_spec {
    ($structname:ident) => {
        impl ::quote::ToTokens for $structname {
            fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
                *tokens = <Self as $crate::model::ToSpec>::to_spec(self);
            }
        }
    };
}

pub(crate) use impl_to_tokens_from_spec;
