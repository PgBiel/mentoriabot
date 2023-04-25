use std::fmt::Display;

use once_cell::sync::Lazy;

pub mod parse;

pub use parse::HumanParseableDateTime;

const HOUR: i32 = 3600;

/// The UTC-3 timezone (the typical Brazilian timezone).
pub static BRAZIL_TIMEZONE: Lazy<chrono::FixedOffset> = Lazy::new(|| {
    // -3 UTC
    chrono::FixedOffset::west_opt(3 * HOUR).unwrap()
});

/// Displays some time as HOUR:MINUTE:SECOND.
pub fn hour_minute_second_display(time: chrono::NaiveTime) -> impl Display {
    time.format("%H:%M:%S")
}

/// Displays some time as HOUR:MINUTE.
pub fn hour_minute_display(time: chrono::NaiveTime) -> impl Display {
    time.format("%H:%M")
}

pub fn parse_hour_minute(string: &str) -> chrono::ParseResult<chrono::NaiveTime> {
    chrono::NaiveTime::parse_from_str(string, "%H:%M")
}
