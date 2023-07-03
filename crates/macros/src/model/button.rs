use darling::util::Flag;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    model::{impl_to_tokens_from_spec, macros::validate_attr, ToSpec, ValidateAttrs},
    util::{self, macros::take_attribute_optional},
};

/// Representation of a button component spec.
#[derive(Debug, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
pub(crate) struct ButtonSpecRepr {
    /// The button's literal label (required)
    #[darling(map = "util::parse_expr")]
    label: syn::Expr,

    /// The button's fixed custom ID; if unspecified,
    /// it is auto-generated.
    #[darling(map = "util::parse_option_expr")]
    custom_id: Option<syn::Expr>,

    primary: Flag,
    secondary: Flag,
    success: Flag,
    danger: Flag,

    /// Makes the button lead to the given link
    /// NOTE: Such a button cannot be awaited for.
    #[darling(map = "util::parse_option_expr")]
    link: Option<syn::Expr>,

    /// An optional single emoji to display near the label
    #[darling(map = "util::parse_option_expr")]
    emoji: Option<syn::Expr>,

    /// If this button is disabled and cannot be clicked
    disabled: Flag,

    /// Function that determines if this button is disabled
    /// (takes &Data, returns bool)
    disabled_function: Option<syn::Path>,
}

impl ValidateAttrs for ButtonSpecRepr {
    fn validate_attrs(&self) -> Result<(), syn::Error> {
        validate_attr!(self (button): @not_both_some(@flag disabled, disabled_function));

        Ok(())
    }
}

impl ToSpec for ButtonSpecRepr {
    fn to_spec(&self) -> TokenStream {
        let label = &self.label;
        let custom_id = take_attribute_optional!(self; custom_id);
        let link = take_attribute_optional!(self; link);
        let emoji = take_attribute_optional!(self; emoji);

        let disabled = if let Some(disabled_function) = &self.disabled_function {
            quote! { #disabled_function(context, data).into() }
        } else {
            let disabled = self.disabled.is_present();
            quote! { #disabled.into() }
        };

        let style = if self.primary.is_present() {
            Some(quote! { ::poise::serenity_prelude::ButtonStyle::Primary })
        } else if self.secondary.is_present() {
            Some(quote! { ::poise::serenity_prelude::ButtonStyle::Secondary })
        } else if self.danger.is_present() {
            Some(quote! { ::poise::serenity_prelude::ButtonStyle::Danger })
        } else if self.success.is_present() {
            Some(quote! { ::poise::serenity_prelude::ButtonStyle::Success })
        } else if self.link.is_some() {
            Some(quote! { ::poise::serenity_prelude::ButtonStyle::Link })
        } else {
            None
        };

        let style = util::wrap_option_into(&style);

        quote! {
            ::mentoriabot_forms::ButtonSpec {
                label: #label,
                custom_id: #custom_id,
                style: #style,
                link: #link,
                emoji: #emoji,
                disabled: #disabled,
            }
        }
    }
}

impl_to_tokens_from_spec!(ButtonSpecRepr);
