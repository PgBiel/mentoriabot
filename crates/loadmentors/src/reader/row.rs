//! Implements the Teacher Row type, a.k.a. the expected CSV format.
use chrono::TimeZone;
use csv::Result as CsvResult;
use mentoriabot_lib::{
    model::{PartialAvailability, Weekday},
    util::{self, BRAZIL_TIMEZONE},
};

use crate::exports::lib::{
    error::{Error, Result},
    model::NewTeacher,
    util::Unvalidated,
};

lazy_static::lazy_static! {
    // "a@b.co"
    pub static ref EMAIL_REGEX: regex::Regex = regex::Regex::new("^.+@.+\\..{2,}$").unwrap();

    // "09:00, 10:00, 13:00"  /  "11:00"
    pub static ref AVAILABILITY_REGEX: regex::Regex = regex::Regex::new("^(\\d{1,2}:\\d{1,2}, )*(\\d{1,2}:\\d{1,2})?$").unwrap();

    // "11/12/2023 13:34:20"
    pub static ref TIMESTAMP_REGEX: regex::Regex =
        regex::Regex::new("^(\\d{2})/(\\d{2})/(\\d{4}) (\\d{2}):(\\d{2}):(\\d{2})$").unwrap();
}

/// Represents a single row in the teachers CSV.
#[derive(validator::Validate, serde::Serialize, serde::Deserialize)]
pub(crate) struct TeacherRow {
    /// The CSV is extracted from Google Forms.
    /// As such, each row starts with a timestamp.
    #[validate(regex = "TIMESTAMP_REGEX")]
    pub(crate) form_timestamp: String,

    /// The teacher's email.
    #[validate(regex = "EMAIL_REGEX")]
    #[validate(length(min = 2, max = 512))]
    pub(crate) email: String,

    /// The teacher's full name.
    #[validate(length(min = 1, max = 512))]
    pub(crate) name: String,

    /// The teacher's WhatsApp-compatible
    /// phone number.
    #[validate(length(min = 0, max = 512))]
    pub(crate) whatsapp: String,

    /// The teacher's Linkedin URL.
    #[validate(length(min = 0, max = 512))]
    pub(crate) linkedin: String,

    /// The teacher's academic course
    /// and the educational institution
    /// in which they study.
    #[validate(length(min = 0, max = 512))]
    pub(crate) course_info: String,

    /// The company the teacher works in,
    /// if any.
    #[validate(length(min = 0, max = 512))]
    pub(crate) company: String,

    /// The role the teacher has at their
    /// company, if any.
    #[validate(length(min = 1, max = 512))]
    pub(crate) company_role: String,

    /// The teacher's bio.
    #[validate(length(min = 0, max = 1024))]
    pub(crate) bio: String,

    /// What the teacher is specialized at.
    #[validate(length(min = 0, max = 512))]
    pub(crate) specialty: String,

    #[validate(regex = "AVAILABILITY_REGEX")]
    #[validate(length(min = 0, 512))]
    pub(crate) availability_monday: String,

    #[validate(regex = "AVAILABILITY_REGEX")]
    #[validate(length(min = 0, 512))]
    pub(crate) availability_tuesday: String,

    #[validate(regex = "AVAILABILITY_REGEX")]
    #[validate(length(min = 0, 512))]
    pub(crate) availability_wednesday: String,
    #[validate(regex = "AVAILABILITY_REGEX")]
    #[validate(length(min = 0, 512))]
    pub(crate) availability_thursday: String,

    #[validate(regex = "AVAILABILITY_REGEX")]
    #[validate(length(min = 0, 512))]
    pub(crate) availability_friday: String,

    #[validate(regex = "AVAILABILITY_REGEX")]
    #[validate(length(min = 0, 512))]
    pub(crate) availability_saturday: String,

    /// Any general comments left by the teacher.
    #[validate(length(min = 0, max = 8192))]
    pub(crate) comment_general: String,

    /// The experience comment left by the teacher,
    /// if any.
    #[validate(length(min = 0, max = 8192))]
    pub(crate) comment_experience: String,
}

impl TeacherRow {
    /// Reads unvalidated teacher rows from the CSV file.
    pub(crate) fn from_csv(csv_contents: &str) -> Vec<CsvResult<Unvalidated<Self>>> {
        let mut reader = csv::Reader::from_reader(csv_contents.as_bytes());
        reader
            .deserialize()
            .map(|res| res.map(Unvalidated::new))
            .collect::<Vec<_>>()
    }

    /// Try to parse this row into a teacher and partial availabilities.
    pub(crate) fn try_parse(&self) -> Result<(NewTeacher, Vec<PartialAvailability>)> {
        let teacher = NewTeacher {
            name: self.name.clone(),
            email: self.email.clone(),
            specialty: self.specialty.clone(),
            applied_at: Some(util::time::datetime_as_utc(
                &BRAZIL_TIMEZONE
                    .datetime_from_str(&self.form_timestamp, "%d/%m/%Y %H:%M:%S")
                    .map_err(|_| Error::DateTimeParse)?,
            )),
            bio: wrap_string_option_based_on_emptiness(self.bio.clone()),
            company: wrap_string_option_based_on_emptiness(self.company.clone()),
            company_role: wrap_string_option_based_on_emptiness(self.company_role.clone()),
            whatsapp: wrap_string_option_based_on_emptiness(self.whatsapp.clone()),
            linkedin: wrap_string_option_based_on_emptiness(self.linkedin.clone()),
        };

        let mut availabilities = Vec::new();

        for (i, availability_text) in [
            &self.availability_monday,
            &self.availability_tuesday,
            &self.availability_wednesday,
            &self.availability_thursday,
            &self.availability_friday,
            &self.availability_saturday,
        ]
        .into_iter()
        .enumerate()
        .filter(|(_, text)| !text.is_empty())
        {
            // sunday is zero => add 1 to start from monday
            let weekday_number = i16::try_from(i).unwrap() + 1;

            // six elements => 0-5 + 1 => 1-6 (within range)
            let weekday = Weekday::try_from(weekday_number).unwrap();

            let matches = AVAILABILITY_REGEX
                .captures(&availability_text)
                .ok_or_else(|| Error::Other("could not parse available schedule specification"))?;

            // see the regex for reference
            // e.g. for "19:00, 20:00, 21:00":
            // -> first_availabilities is "19:00, 20:00, "
            // -> second_availabilities is "21:00"
            let first_availabilities = matches.get(1).unwrap().as_str();
            let last_availability = matches.get(2).unwrap().as_str();

            let availability_strings = first_availabilities
                .split(", ")
                .into_iter()
                .filter(|s| !s.is_empty()) // last one is empty (due to trailing ", ")
                .chain([last_availability]);

            let new_availabilities = availability_strings
                .map(|avail| chrono::NaiveTime::parse_from_str(avail, "%H:%M:%S"))
                .map(|time| PartialAvailability {
                    id: None,
                    teacher_id: None,
                    weekday: Some(weekday),
                    time_start: Some(time.unwrap()),
                    expired: Some(false),
                    duration: Some(1i16),
                });

            availabilities.extend(new_availabilities);
        }

        Ok((teacher, availabilities))
    }
}

/// Returns `None` if the given string is empty.
/// Otherwise, `Some(string)`.
fn wrap_string_option_based_on_emptiness(string: String) -> Option<String> {
    Some(string).filter(|string| !string.is_empty())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wrap_string_based_on_emptiness_returns_none_for_empty_string() {
        assert_eq!(wrap_string_option_based_on_emptiness("".into()), None);
    }

    #[test]
    fn test_wrap_string_based_on_emptiness_wraps_non_empty_string_in_some() {
        assert_eq!(wrap_string_option_based_on_emptiness("abc".into()), Some("abc".into()));
    }
}
