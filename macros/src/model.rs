mod button;
mod macros;
mod select;
mod select_option;
mod reply;

pub(crate) use button::ButtonSpecRepr;
use proc_macro2::TokenStream;
pub(crate) use select::SelectMenuSpecRepr;
pub(crate) use select_option::SelectOptionSpecRepr;
pub(crate) use reply::ReplySpecRepr;

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
