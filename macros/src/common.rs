use syn::NestedMeta;

use crate::util::{self, parse, Parsed2};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FormContextInfo {
    pub(crate) data: syn::Type,
    pub(crate) error: syn::Type,
}

#[derive(Debug, Clone, PartialEq, Eq, darling::FromMeta)]
pub(crate) struct FormData {
    /// Type of the Data object to be passed to components.
    /// By default, ()
    #[darling(default = "util::empty_tuple_type", map = "parse")]
    pub(crate) data: syn::Type,

    /// Context's Data and Error types.
    pub(crate) ctx: FormContextInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, darling::FromMeta)]
#[darling(allow_unknown_fields)]
pub(crate) struct FormDataAttr {
    pub(crate) form_data: FormData,
}

// --- impls ---

impl darling::FromMeta for FormContextInfo {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let Parsed2(data, error) = Parsed2::from_list(items)?;

        Ok(Self { data, error })
    }
}
