use std::ops::Add;

use darling::util::Flag;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    model::{impl_to_tokens_from_spec, macros::validate_attr, ToSpec, ValidateAttrs},
    util,
    util::macros::{
        take_attribute_optional, take_attribute_or_its_function_optional,
        take_attribute_or_its_function_required,
    },
};

/// Representation of button attributes.
#[derive(Debug, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
pub struct ButtonSpecRepr {
    /// The button's literal label (please specify this, or a `label_function`)
    label: Option<String>,

    /// Path to a function (accepting &Data, as specified by `#[data = "Data"]`)
    /// that returns the button's label as a Into<String> (specify this or `label`).
    label_function: Option<syn::Path>,

    /// The button's fixed custom ID; if unspecified,
    /// it is auto-generated.
    custom_id: Option<String>,

    primary: Flag,
    secondary: Flag,
    success: Flag,
    danger: Flag,

    /// Makes the button lead to the given link
    /// NOTE: Such a button cannot be awaited for.
    link: Option<String>,

    /// Function takes &Data, and returns the link the button leads to (Into<String>).
    link_function: Option<syn::Path>,

    /// An optional single emoji to display near the label
    emoji: Option<char>,

    /// Function that returns the emoji to display near the button label
    /// (takes &Data, returns Into<ReactionType>)
    emoji_function: Option<syn::Path>,

    /// If this button is disabled and cannot be clicked
    disabled: Flag,

    /// Function that determines if this button is disabled
    /// (takes &Data, returns bool)
    disabled_function: Option<syn::Path>,
}

impl ValidateAttrs for ButtonSpecRepr {
    fn validate_attrs(&self) -> Result<(), syn::Error> {
        validate_attr!(self (button): @not_both_some(label, label_function));
        validate_attr!(self (button): @not_both_none(label, label_function));
        validate_attr!(self (button): @not_both_some(emoji, emoji_function));
        validate_attr!(self (button): @not_both_some(link, link_function));
        validate_attr!(self (button): @not_both_some(@flag disabled, disabled_function));

        Ok(())
    }
}

impl ToSpec for ButtonSpecRepr {
    fn to_spec(&self) -> TokenStream {
        let label = take_attribute_or_its_function_required!(self; label, label_function);
        let custom_id = take_attribute_optional!(self; custom_id);
        let link = take_attribute_or_its_function_optional!(self; link, link_function);
        let emoji = take_attribute_or_its_function_optional!(self; emoji, emoji_function);

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
            ::minirustbot_forms::ButtonSpec {
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
