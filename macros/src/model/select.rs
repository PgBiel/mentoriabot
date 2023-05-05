use darling::util::Flag;
use proc_macro2::TokenStream;
use quote::quote;
use crate::model::select_option::SelectOptionSpecRepr;
use crate::model::ToSpec;

use crate::util;

/// Representation of a select menu component spec.
#[derive(Debug, Default, Clone, darling::FromDeriveInput)]
#[darling(allow_unknown_fields, attributes(select))]
pub(crate) struct SelectSpecRepr {
    /// The menu's fixed custom ID; if unspecified,
    /// it is auto-generated.
    #[darling(map = "util::parse_option_expr")]
    custom_id: Option<syn::Expr>,

    /// Minimum amount of options the user must select, or 1.
    min_values: Option<syn::Expr>,

    /// Maximum amount of options the user can select, or `min_values`.
    max_values: Option<syn::Expr>,

    /// If this menu is disabled and cannot be clicked
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

impl ToSpec for SelectSpecRepr {
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
            ::minirustbot_forms::SelectMenuSpec {
                custom_id: #custom_id,
                options: #options,
                min_values: #min_values,
                max_values: #max_values,
                disabled: #disabled,
            }
        }
    }
}
