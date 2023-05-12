use std::fmt::Display;

use chrono::{TimeZone, Timelike};
use once_cell::sync::Lazy;

pub mod parse;

pub use parse::HumanParseableDateTime;

const HOUR: i32 = 3600;

/// The UTC-3 timezone (the typical Brazilian timezone).
pub static BRAZIL_TIMEZONE: Lazy<chrono::FixedOffset> = Lazy::new(|| {
    // -3 UTC
    chrono::FixedOffset::west_opt(3 * HOUR).unwrap()
});

/// Returns the current time in the Brazil main timezone
pub fn brazil_now() -> chrono::DateTime<chrono::FixedOffset> {
    chrono::Utc::now().with_timezone(&*BRAZIL_TIMEZONE)
}

/// Displays some time as HOUR:MINUTE:SECOND.
pub fn hour_minute_second_display(time: chrono::NaiveTime) -> impl Display {
    time.format("%H:%M:%S")
}

/// Displays some time as HOUR:MINUTE.
pub fn hour_minute_display(time: chrono::NaiveTime) -> impl Display {
    time.format("%H:%M")
}

/// Displays a date as DAY/MONTH.
pub fn day_month_display(datetime: chrono::NaiveDate) -> impl Display {
    datetime.format("%d/%m")
}

pub fn parse_hour_minute(string: &str) -> chrono::ParseResult<chrono::NaiveTime> {
    chrono::NaiveTime::parse_from_str(string, "%H:%M")
}

/// Replaces the time in a [`chrono::DateTime`] with a [`chrono::NaiveTime`] object's time fields.
pub fn datetime_with_time<T: TimeZone>(
    datetime: chrono::DateTime<T>,
    time: chrono::NaiveTime,
) -> Option<chrono::DateTime<T>> {
    datetime
        .with_hour(time.hour())
        .and_then(|d| d.with_minute(time.minute()))
        .and_then(|d| d.with_second(time.second()))
}

/// Converts a [`chrono::DateTime`] object in a certain timezone to UTC.
pub fn datetime_as_utc<T: TimeZone>(
    datetime: &chrono::DateTime<T>,
) -> chrono::DateTime<chrono::Utc> {
    chrono::Utc.from_utc_datetime(&datetime.naive_utc())
}
