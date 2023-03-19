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
        convert_chrono_duration_to_string(dur)
    } else {
        let seconds = duration.as_secs();
        format!("{} second{}", seconds, if seconds != 1 { "s" } else { "" })
    }
}

/// Converts a Chrono duration to a string.
pub fn convert_chrono_duration_to_string(duration: chrono::Duration) -> String {
    if duration.num_minutes() < 1 {
        let seconds = duration.num_seconds();
        format!("{} second{}", seconds, if seconds != 1 { "s" } else { "" })
    } else if duration.num_hours() < 1 {
        let minutes = duration.num_minutes();
        format!("{} minute{}", minutes, if minutes != 1 { "s" } else { "" })
    } else if duration.num_days() < 1 {
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;
        format!(
            "{} hour{}{}",
            hours,
            if hours != 1 { "s" } else { "" },
            if minutes > 0 {
                format!(
                    " and {} minute{}",
                    minutes,
                    if minutes != 1 { "s" } else { "" }
                )
            } else {
                String::new()
            }
        )
    } else {
        let days = duration.num_days();
        format!("{} day{}", days, if days != 1 { "s" } else { "" })
    }
}

/// Converts a Chrono duration to a string in Brazilian Portuguese.
pub fn convert_chrono_duration_to_brazilian_string(duration: chrono::Duration) -> String {
    if duration.num_minutes() < 1 {
        let seconds = duration.num_seconds();
        format!("{} segundo{}", seconds, if seconds != 1 { "s" } else { "" })
    } else if duration.num_hours() < 1 {
        let minutes = duration.num_minutes();
        format!("{} minuto{}", minutes, if minutes != 1 { "s" } else { "" })
    } else if duration.num_days() < 1 {
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;
        format!(
            "{} hora{}{}",
            hours,
            if hours != 1 { "s" } else { "" },
            if minutes > 0 {
                format!(
                    " e {} minuto{}",
                    minutes,
                    if minutes != 1 { "s" } else { "" }
                )
            } else {
                String::new()
            }
        )
    } else {
        let days = duration.num_days();
        format!("{} dia{}", days, if days != 1 { "s" } else { "" })
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

/// Obtains the UTC-3 timezone (the typical Brazilian timezone).
pub fn brazil_timezone() -> Option<chrono::FixedOffset> {
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

    /// Attempts to parse a human datetime string **as UTC-3**, and converts it
    /// to a **UTC DateTime**.
    ///
    /// # Supported date/time formats
    /// - %Y-%m-%d %H:%M:%S
    /// - %Y-%m-%d %H:%M
    /// - %d/%m/%d %H:%M:%S
    /// - %d/%m/%d %H:%M
    /// - %H:%M:%S
    /// - %H:%M
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::util::HumanParseableDateTime;
    /// # use chrono::TimeZone;
    /// let today = chrono::Utc::now();
    /// let expected_date1 =
    ///     today.with_ymd_and_hms(2023, 03, 19, 14, 29, 30).single().unwrap();
    /// let expected_date2 =
    ///     today.with_ymd_and_hms(2023, 03, 19, 14, 29, 0).single().unwrap();
    /// let expected_date3 =
    ///     today.with_ymd_and_hms(today.year(), today.month(), today.day(), 14, 29, 30)
    ///     .single().unwrap();
    /// let expected_date4 =
    ///     today.with_ymd_and_hms(today.year(), today.month(), today.day(), 14, 29, 0)
    ///     .single().unwrap();
    ///
    /// let parsed1: HumanParseableDateTime = "2023-03-19 11:29:30".parse().unwrap();
    /// let parsed2: HumanParseableDateTime = "2023-03-19 11:29".parse().unwrap();
    /// let parsed3: HumanParseableDateTime = "19/03/2023 11:29:30".parse().unwrap();
    /// let parsed4: HumanParseableDateTime = "19/03/2023 11:29".parse().unwrap();
    /// let parsed5: HumanParseableDateTime = "11:29:30".parse().unwrap();
    /// let parsed6: HumanParseableDateTime = "11:29".parse().unwrap();
    ///
    /// assert_eq!(parsed1.0, expected_date1);
    /// assert_eq!(parsed2.0, expected_date2);
    /// assert_eq!(parsed3.0, expected_date1);
    /// assert_eq!(parsed4.0, expected_date2);
    /// assert_eq!(parsed5.0, expected_date3);
    /// assert_eq!(parsed6.0, expected_date4);
    /// ```
    fn from_str(s: &str) -> Result<Self> {
        let s = &s.trim()
            .replace(", ", "")  // some common little mistakes/changes
            .replace("; ", "")
            .replace("   ", " ")
            .replace("  ", " ")
            .replace(",", "")
            .replace(";", "");

        let curr_date = chrono::Utc::now()
            .with_timezone(&brazil_timezone().expect("Brazil timezone was invalid."));

        let today =
            format!("{}-{}-{}", curr_date.year(), curr_date.month(), curr_date.day());

        // in case date wasn't specified => default to today
        let string_with_today: &str = &format!("{today} {s}");

        let string_with_year: &str = &format!("{}; {s}", curr_date.year());

        let result = try_date_parse(s, "%Y-%m-%d %H:%M:%S")
            .or_else(|| try_date_parse(s, "%Y-%m-%d %H:%M"))
            .or_else(|| try_date_parse(s, "%d/%m/%Y %H:%M:%S"))
            .or_else(|| try_date_parse(s, "%d/%m/%Y %H:%M"))
            .or_else(|| try_date_parse(string_with_year, "%Y; %d/%m %H:%M:%S"))
            .or_else(|| try_date_parse(string_with_year, "%Y; %d/%m %H:%M"))
            .or_else(|| try_date_parse(string_with_today, "%Y-%m-%d %H:%M:%S"))
            .or_else(|| try_date_parse(string_with_today, "%Y-%m-%d %H:%M"))
            // inverted (time then date)
            .or_else(|| try_date_parse(s, "%H:%M:%S %Y-%m-%d"))
            .or_else(|| try_date_parse(s, "%H:%M %Y-%m-%d"))
            .or_else(|| try_date_parse(s, "%H:%M:%S %d/%m/%Y"))
            .or_else(|| try_date_parse(s, "%H:%M %d/%m/%Y"))
            .or_else(|| try_date_parse(string_with_year, "%Y; %H:%M:%S %d/%m"))
            .or_else(|| try_date_parse(string_with_year, "%Y; %H:%M %d/%m"));

        result.ok_or(Error::DateTimeParse)
    }
}

impl<Tz: chrono::TimeZone> From<chrono::DateTime<Tz>> for HumanParseableDateTime {
    /// Converts a [`chrono::DateTime`] with any timezone to a HumanParseableDateTime
    /// by converting the DateTime to UTC and wrapping it.
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
    /// Uses [`chrono::DateTime`]'s Display implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <chrono::DateTime<chrono::Utc> as Display>::fmt(&self.0, f)
    }
}

impl Deref for HumanParseableDateTime {
    type Target = chrono::DateTime<chrono::Utc>;

    /// Allows easy access to the wrapped [`chrono::DateTime`]
    /// inside.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HumanParseableDateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod date_time {
        use super::*;
        use chrono::TimeZone;

        #[test]
        fn parses_yyyy_mm_dd_hh_mm_ss_correctly() {
            let parsed: HumanParseableDateTime = "2023-03-19 11:29:30".parse().unwrap();

            assert_eq!(
                chrono::Utc.with_ymd_and_hms(2023, 03, 19, 14, 29, 30).unwrap(),
                parsed.0
            )
        }

        #[test]
        fn parses_yyyy_mm_dd_hh_mm_with_zero_seconds() {
            let parsed: HumanParseableDateTime = "2023-03-19 11:29".parse().unwrap();

            assert_eq!(
                chrono::Utc.with_ymd_and_hms(2023, 03, 19, 14, 29, 0).unwrap(),
                parsed.0
            )
        }

        #[test]
        fn parses_dd_mm_yyyy_hh_mm_ss_correctly() {
            let parsed: HumanParseableDateTime = "19/03/2023 11:29:30".parse().unwrap();

            assert_eq!(
                chrono::Utc.with_ymd_and_hms(2023, 03, 19, 14, 29, 30).unwrap(),
                parsed.0
            )
        }

        #[test]
        fn parses_dd_mm_yyyy_hh_mm_with_zero_seconds() {
            let parsed: HumanParseableDateTime = "19/03/2023 11:29".parse().unwrap();

            assert_eq!(
                chrono::Utc.with_ymd_and_hms(2023, 03, 19, 14, 29, 0).unwrap(),
                parsed.0
            )
        }

        #[test]
        fn parses_dd_mm_hh_mm_ss_with_current_year() {
            let year = chrono::Utc::now().year();

            let parsed: HumanParseableDateTime = "19/03 11:29:30".parse().unwrap();

            assert_eq!(
                chrono::Utc.with_ymd_and_hms(year, 03, 19, 14, 29, 30).unwrap(),
                parsed.0
            )
        }

        #[test]
        fn parses_dd_mm_hh_mm_with_current_year_and_zero_seconds() {
            let year = chrono::Utc::now().year();

            let parsed: HumanParseableDateTime = "19/03 11:29".parse().unwrap();

            assert_eq!(
                chrono::Utc.with_ymd_and_hms(year, 03, 19, 14, 29, 0).unwrap(),
                parsed.0
            )
        }

        // -- inverted tests --

        #[test]
        fn parses_hh_mm_ss_yyyy_mm_dd_correctly() {
            let parsed: HumanParseableDateTime = "11:29:30 2023-03-19".parse().unwrap();

            assert_eq!(
                chrono::Utc.with_ymd_and_hms(2023, 03, 19, 14, 29, 30).unwrap(),
                parsed.0
            )
        }

        #[test]
        fn parses_hh_mm_yyyy_mm_dd_with_zero_seconds() {
            let parsed: HumanParseableDateTime = "11:29 2023-03-19".parse().unwrap();

            assert_eq!(
                chrono::Utc.with_ymd_and_hms(2023, 03, 19, 14, 29, 0).unwrap(),
                parsed.0
            )
        }

        #[test]
        fn parses_hh_mm_ss_dd_mm_yyyy_correctly() {
            let parsed: HumanParseableDateTime = "11:29:30 19/03/2023".parse().unwrap();

            assert_eq!(
                chrono::Utc.with_ymd_and_hms(2023, 03, 19, 14, 29, 30).unwrap(),
                parsed.0
            )
        }

        #[test]
        fn parses_hh_mm_dd_mm_yyyy_with_zero_seconds() {
            let parsed: HumanParseableDateTime = "11:29 19/03/2023".parse().unwrap();

            assert_eq!(
                chrono::Utc.with_ymd_and_hms(2023, 03, 19, 14, 29, 0).unwrap(),
                parsed.0
            )
        }

        #[test]
        fn parses_hh_mm_ss_dd_mm_with_current_year() {
            let year = chrono::Utc::now().year();

            let parsed: HumanParseableDateTime = "11:29:30 19/03".parse().unwrap();

            assert_eq!(
                chrono::Utc.with_ymd_and_hms(year, 03, 19, 14, 29, 30).unwrap(),
                parsed.0
            )
        }

        #[test]
        fn parses_hh_mm_dd_mm_with_current_year_and_zero_seconds() {
            let year = chrono::Utc::now().year();

            let parsed: HumanParseableDateTime = "11:29 19/03".parse().unwrap();

            assert_eq!(
                chrono::Utc.with_ymd_and_hms(year, 03, 19, 14, 29, 0).unwrap(),
                parsed.0
            )
        }

        #[test]
        fn parses_hh_mm_ss_with_current_date() {
            let today = chrono::Utc::now();
            let (year, month, day) = (today.year(), today.month(), today.day());

            let parsed: HumanParseableDateTime = "11:29:30".parse().unwrap();

            assert_eq!(
                chrono::Utc.with_ymd_and_hms(year, month, day, 14, 29, 30).unwrap(),
                parsed.0
            )
        }

        #[test]
        fn parses_hh_mm_with_current_date_and_zero_seconds() {
            let today = chrono::Utc::now();
            let (year, month, day) = (today.year(), today.month(), today.day());

            let parsed: HumanParseableDateTime = "11:29".parse().unwrap();

            assert_eq!(
                chrono::Utc.with_ymd_and_hms(year, month, day, 14, 29, 0).unwrap(),
                parsed.0
            )
        }

        #[test]
        fn parses_yyyy_mm_dd_hh_mm_ss_with_commas_and_multiple_spaces() {
            let parsed: HumanParseableDateTime = "2023-03-19,,;;;  11:29:30".parse().unwrap();

            assert_eq!(
                chrono::Utc.with_ymd_and_hms(2023, 03, 19, 14, 29, 30).unwrap(),
                parsed.0
            )
        }
    }
}
