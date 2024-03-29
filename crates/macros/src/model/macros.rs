macro_rules! validate_attr {
    ($self:ident ($name:ident): @not_both_some($attr1:ident, $attr2:ident)) => {
        if $self.$attr1.is_some() && $self.$attr2.is_some() {
            return ::core::result::Result::Err(
                ::syn::Error::new(
                    ::proc_macro2::Span::call_site(),
                    concat!(
                        stringify!($name),
                        ": Cannot specify attributes '",
                        stringify!($attr1),
                        "' and '",
                        stringify!($attr2),
                        "' at the same time."
                    ),
                )
                .into(),
            );
        }
    };
    ($self:ident ($name:ident): @not_both_none($attr1:ident, $attr2:ident)) => {
        if $self.$attr1.is_none() && $self.$attr2.is_none() {
            return ::core::result::Result::Err(
                ::syn::Error::new(
                    ::proc_macro2::Span::call_site(),
                    concat!(
                        stringify!($name),
                        ": Must specify at least one of '",
                        stringify!($attr1),
                        "' and '",
                        stringify!($attr2),
                        "'."
                    ),
                )
                .into(),
            );
        }
    };
    ($self:ident ($name:ident): @not_both_some(@flag $attr1:ident, $attr2:ident)) => {
        if $self.$attr1.is_present() && $self.$attr2.is_some() {
            return ::core::result::Result::Err(
                ::syn::Error::new(
                    ::proc_macro2::Span::call_site(),
                    concat!(
                        stringify!($name),
                        ": Cannot specify attributes '",
                        stringify!($attr1),
                        "' and '",
                        stringify!($attr2),
                        "' at the same time."
                    ),
                )
                .into(),
            );
        }
    };
    ($self:ident ($name:ident): @not_both_some(@vec $attr1:ident, $attr2:ident)) => {
        if !$self.$attr1.is_empty() && $self.$attr2.is_some() {
            return ::core::result::Result::Err(
                ::syn::Error::new(
                    ::proc_macro2::Span::call_site(),
                    concat!(
                        stringify!($name),
                        ": Cannot specify attributes '",
                        stringify!($attr1),
                        "' and '",
                        stringify!($attr2),
                        "' at the same time."
                    ),
                )
                .into(),
            );
        }
    };
}

pub(crate) use validate_attr;
