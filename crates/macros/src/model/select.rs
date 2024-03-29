use darling::util::Flag;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Error;

use crate::{
    model::{macros::validate_attr, select_option::SelectOptionSpecRepr, ToSpec, ValidateAttrs},
    util,
};

/// Representation of a select menu component spec.
#[derive(Debug, Default, Clone, darling::FromMeta)]
#[darling(allow_unknown_fields)]
pub(crate) struct SelectMenuSpecRepr {
    /// The menu's fixed custom ID; if unspecified,
    /// it is auto-generated.
    #[darling(map = "util::parse_option_expr")]
    custom_id: Option<syn::Expr>,

    /// Minimum amount of options the user must select, or 1.
    min_values: Option<syn::Expr>,

    /// Maximum amount of options the user can select, or `min_values`.
    max_values: Option<syn::Expr>,

    /// If this menu is disabled and cannot be clicked (exclusive with `disabled_expr`)
    disabled: Flag,

    /// Expression that determines if this menu is disabled
    disabled_expr: Option<syn::Expr>,

    /// This select menu's options as an expression.
    /// Either specify this, or specify multiple `option` attributes.
    #[darling(map = "util::parse_option_expr")]
    options: Option<syn::Expr>,

    /// Describe each select option.
    #[darling(multiple)]
    option: Vec<SelectOptionSpecRepr>,
}

impl ValidateAttrs for SelectMenuSpecRepr {
    fn validate_attrs(&self) -> Result<(), Error> {
        validate_attr!(self (selectmenu): @not_both_some(@vec option, options));
        validate_attr!(self (selectmenu): @not_both_some(@flag disabled, disabled_expr));

        Ok(())
    }
}

impl ToSpec for SelectMenuSpecRepr {
    fn to_spec(&self) -> TokenStream {
        let custom_id = util::wrap_option_into(&self.custom_id);
        let min_values = util::wrap_option_into(&self.min_values);
        let max_values = util::wrap_option_into(&self.max_values);

        let options = if let Some(options) = self.options.as_ref() {
            quote! { #options }
        } else {
            let options = self.option.iter().map(ToSpec::to_spec).collect::<Vec<_>>();
            quote! { ::std::vec![#(#options),*] }
        };

        let disabled = if let Some(disabled_expr) = self.disabled_expr.as_ref() {
            quote! { #disabled_expr }
        } else {
            let disabled = self.disabled.is_present();
            quote! { #disabled }
        };

        quote! {
            ::mentoriabot_forms::SelectMenuSpec {
                custom_id: #custom_id,
                options: #options,
                min_values: #min_values,
                max_values: #max_values,
                disabled: #disabled,
            }
        }
    }
}
