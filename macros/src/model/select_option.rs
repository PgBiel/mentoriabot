use darling::util::Flag;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    model::{ToSpec, ValidateAttrs},
    util,
};

/// Representation of a select menu option spec.
#[derive(Debug, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
pub(crate) struct SelectOptionSpecRepr {
    /// The option's literal label.
    #[darling(map = "util::parse_expr")]
    label: syn::Expr,

    /// This option's unique value key.
    #[darling(map = "util::parse_expr")]
    value_key: syn::Expr,

    /// The option's description (small explanation text), optional.
    #[darling(map = "util::parse_option_expr")]
    description: Option<syn::Expr>,

    /// An optional single emoji to display near the label
    #[darling(map = "util::parse_option_expr")]
    emoji: Option<syn::Expr>,

    /// If this is the default selection option.
    is_default: Flag,
}

impl ValidateAttrs for SelectOptionSpecRepr {
    fn validate_attrs(&self) -> Result<(), syn::Error> {
        Ok(())
    }
}

impl ToSpec for SelectOptionSpecRepr {
    fn to_spec(&self) -> TokenStream {
        let Self {
            label,
            value_key,
            description,
            emoji,
            is_default,
        } = self;

        let description = util::wrap_option_into(description);
        let emoji = util::wrap_option_into(emoji);
        let is_default = is_default.is_present();

        quote! {
            ::minirustbot_forms::SelectMenuOptionSpec {
                label: (#label).into(),
                value_key: ::minirustbot_forms::interaction::SelectValue(#value_key.into()),
                description: (#description).into(),
                emoji: #emoji,
                is_default: #is_default,
            }
        }
    }
}
