macro_rules! tr {
    // t!("foo")
    ($key:expr) => {
        rust_i18n::t!($key)
    };

    // t!("foo", locale="en")
    ($key:expr, locale=$locale:expr) => {
        rust_i18n::t!($key, locale=$crate::util::default_locale(&$locale))
    };

    // tr!("key", ctx=ctx)
    ($key:expr, ctx=$ctx:expr) => {
        rust_i18n::t!($key, locale=$crate::util::get_defaulted_locale($ctx.into()))
    };

    // t!("foo", locale="en", a=1, b="Foo")
    ($key:expr, locale=$locale:expr, $($var_name:tt = $var_val:expr),+ $(,)?) => {
        rust_i18n::t!($key, locale=$crate::util::default_locale(&$locale), $($var_name = $var_val),+)
    };

    // t!("foo", ctx=ctx, a=1, b="Foo")
    ($key:expr, ctx=$ctx:expr, $($var_name:tt = $var_val:expr),+ $(,)?) => {
        rust_i18n::t!($key, locale=$crate::util::get_defaulted_locale($ctx.into()), $($var_name = $var_val),+)
    };

    // t!("foo %{a} %{b}", a="bar", b="baz")
    ($key:expr, $($var_name:tt = $var_val:expr),+ $(,)?) => {
         rust_i18n::t!($key, $($var_name = $var_val),*)
    };

    // t!("foo %{a} %{b}", locale = "en", "a" => "bar", "b" => "baz")
    ($key:expr, locale = $locale:expr, $($var_name:expr => $var_val:expr),+ $(,)?) => {
        {
            tr!($key, locale = $locale, $($var_name = $var_val),*)
        }
    };

    // tr!("foo %{a} %{b}", ctx = ctx, "a" => "bar", "b" => "baz")
    ($key:expr, ctx = $ctx:expr, $($var_name:expr => $var_val:expr),+ $(,)?) => {
        {
            tr!($key, locale = $crate::util::get_defaulted_locale($ctx.into()), $($var_name = $var_val),*)
        }
    };

    // t!("foo %{a} %{b}", "a" => "bar", "b" => "baz")
    ($key:expr, $($var_name:expr => $var_val:expr),+ $(,)?) => {
        {
            tr!($key, locale = &rust_i18n::locale(), $($var_name = $var_val),*)
        }
    };
}

pub(crate) use tr;
