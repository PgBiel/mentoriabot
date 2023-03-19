use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use chrono::Datelike;
use crate::error::{Error, Result};

/// Converts a certain duration to a human-readable String.
///
/// Displays, at most, the amount of days. Otherwise, uses smaller units (the largest possible).
pub fn convert_duration_to_string(duration: std::time::Duration) -> String {
    let dur = chrono::Duration::from_std(duration);
    if let Ok(dur) = dur {
        if dur.num_minutes() < 1 {
            format!("{} seconds", dur.num_seconds())
        } else if dur.num_hours() < 1 {
            format!("{} minutes", dur.num_minutes())
        } else if dur.num_days() < 1 {
            format!(
                "{} hours and {} minutes",
                dur.num_hours(),
                dur.num_minutes()
            )
        } else {
            format!("{} days", dur.num_days())
        }
    } else {
        format!("{} seconds", duration.as_secs())
    }
}

/// Represents a DateTime which can be parsed in a semi-human format.
/// It is parsed as UTC-3 and converted to UTC.
///
/// # Supported date/time formats
/// - %Y-%m-%d %H:%M:%S
/// - %Y-%m-%d %H:%M
/// - %d/%m/%d %H:%M:%S
/// - %d/%m/%d %H:%M
/// - %H:%M:%S
/// - %H:%M
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HumanParseableDateTime(pub chrono::DateTime<chrono::Utc>);

const HOUR: i32 = 3600;

fn brazil_timezone() -> Option<chrono::FixedOffset> {
    // -3 UTC
    chrono::FixedOffset::west_opt(3 * HOUR)
}

fn try_date_parse(s: &str, fmt: &str) -> Option<HumanParseableDateTime> {
    let date_time = chrono::NaiveDateTime::parse_from_str(s, fmt).ok()?;
    let date_time = date_time.and_local_timezone(
            brazil_timezone().expect("Brazil timezone was invalid.")
        )
        .single()?;
    let date_time = date_time.with_timezone(&chrono::Utc);
    Some(HumanParseableDateTime(date_time))
}

impl FromStr for HumanParseableDateTime {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();
        let curr_date = chrono::Utc::now()
            .with_timezone(&brazil_timezone().expect("Brazil timezone was invalid."));

        let today =
            format!("{}-{}-{}", curr_date.year(), curr_date.month(), curr_date.day());

        // in case date wasn't specified => default to today
        let string_with_today: &str = &format!("{today} {s}");

        let result = try_date_parse(s, "%Y-%m-%d %H:%M:%S")
            .or_else(|| try_date_parse(s, "%Y-%m-%d %H:%M"))
            .or_else(|| try_date_parse(s, "%d/%m/%Y %H:%M:%S"))
            .or_else(|| try_date_parse(s, "%d/%m/%Y %H:%M"))
            .or_else(|| try_date_parse(string_with_today, "%Y-%m-%d %H:%M:%S"))
            .or_else(|| try_date_parse(string_with_today, "%Y-%m-%d %H:%M"));

        result.ok_or(Error::DateTimeParse)
    }
}

impl<Tz: chrono::TimeZone> From<chrono::DateTime<Tz>> for HumanParseableDateTime {
    fn from(value: chrono::DateTime<Tz>) -> Self {
        Self(value.with_timezone(&chrono::Utc))
    }
}

impl Into<chrono::DateTime<chrono::Utc>> for HumanParseableDateTime {
    fn into(self) -> chrono::DateTime<chrono::Utc> {
        self.0
    }
}

impl Display for HumanParseableDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <chrono::DateTime<chrono::Utc> as Display>::fmt(&self.0, f)
    }
}

impl Deref for HumanParseableDateTime {
    type Target = chrono::DateTime<chrono::Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HumanParseableDateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
