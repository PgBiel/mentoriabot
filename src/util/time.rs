use once_cell::sync::Lazy;

pub mod parse;

pub use parse::HumanParseableDateTime;

const HOUR: i32 = 3600;

/// The UTC-3 timezone (the typical Brazilian timezone).
pub static BRAZIL_TIMEZONE: Lazy<chrono::FixedOffset> = Lazy::new(|| {
    // -3 UTC
    chrono::FixedOffset::west_opt(3 * HOUR).unwrap()
});
