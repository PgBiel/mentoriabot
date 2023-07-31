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
        assert_eq!(
            wrap_string_option_based_on_emptiness("abc".into()),
            Some("abc".into())
        );
    }

    #[test]
    fn test_row_teacher_parsing_works_correctly() {
        let row = TeacherRow {
            form_timestamp: "11/05/2023 18:43:55".into(),
            email: "email@email.com".into(),
            name: "José Silva".into(),
            whatsapp: "(41)912345678".into(),
            linkedin: "https://linkedin.com/sus".into(),
            course_info: "Engenharia da Computação, USP".into(),
            company: "Empadas & Cia.".into(),
            company_role: "Gerente de Software".into(),
            bio: "Gosto do meu trabalho".into(),
            specialty: "Álgebra Linear".into(),
            availability_monday: "09:00, 10:00".into(),
            availability_tuesday: "20:00, 21:00".into(),
            availability_wednesday: "12:00, 13:00".into(),
            availability_thursday: "".into(),
            availability_friday: "10:00".into(),
            availability_saturday: "17:00, 18:00, 19:00, 20:00, 21:00".into(),
            comment_general: "".into(),
            comment_experience: "Nada a declarar".into(),
        };

        let (teacher, _) = row.try_parse().unwrap();

        assert_eq!(
            teacher,
            NewTeacher {
                name: "José Silva".into(),
                email: "email@email.com".into(),
                specialty: "Álgebra Linear".into(),

                // 21:43:55 in UTC (18:43:55 in UTC-3)
                applied_at: Some(
                    chrono::Utc
                        .with_ymd_and_hms(2023, 05, 11, 21, 43, 55)
                        .single()
                        .unwrap()
                ),
                bio: Some("Gosto do meu trabalho".into()),
                company: Some("Empadas & Cia.".into()),
                company_role: Some("Gerente de Software".into()),
                whatsapp: Some("(41)912345678".into()),
                linkedin: Some("https://linkedin.com/sus".into()),
            }
        );
    }

    #[test]
    fn test_row_availabilities_parsing_works_correctly() {
        let row = TeacherRow {
            form_timestamp: "11/05/2023 18:43:55".into(),
            email: "email@email.com".into(),
            name: "José Silva".into(),
            whatsapp: "(41)912345678".into(),
            linkedin: "https://linkedin.com/sus".into(),
            course_info: "Engenharia da Computação, USP".into(),
            company: "Empadas & Cia.".into(),
            company_role: "Gerente de Software".into(),
            bio: "Gosto do meu trabalho".into(),
            specialty: "Álgebra Linear".into(),
            availability_monday: "09:00, 10:00".into(),
            availability_tuesday: "20:00, 21:00".into(),
            availability_wednesday: "12:00, 13:00".into(),
            availability_thursday: "".into(),
            availability_friday: "10:00".into(),
            availability_saturday: "17:00, 18:00, 19:00, 20:00, 21:00".into(),
            comment_general: "".into(),
            comment_experience: "Nada a declarar".into(),
        };

        let (_, availabilities) = row.try_parse().unwrap();

        let time_hm = |hour, min| chrono::NaiveTime::from_hms_opt(hour, min, 0).unwrap();
        assert_eq!(
            availabilities,
            vec![
                PartialAvailability {
                    id: None,
                    teacher_id: None,
                    weekday: Some(Weekday::Monday),
                    time_start: Some(time_hm(9, 0)),
                    expired: Some(false),
                    duration: Some(1i16),
                },
                PartialAvailability {
                    id: None,
                    teacher_id: None,
                    weekday: Some(Weekday::Monday),
                    time_start: Some(time_hm(10, 0)),
                    expired: Some(false),
                    duration: Some(1i16),
                },
                PartialAvailability {
                    id: None,
                    teacher_id: None,
                    weekday: Some(Weekday::Tuesday),
                    time_start: Some(time_hm(20, 0)),
                    expired: Some(false),
                    duration: Some(1i16),
                },
                PartialAvailability {
                    id: None,
                    teacher_id: None,
                    weekday: Some(Weekday::Tuesday),
                    time_start: Some(time_hm(21, 0)),
                    expired: Some(false),
                    duration: Some(1i16),
                },
                PartialAvailability {
                    id: None,
                    teacher_id: None,
                    weekday: Some(Weekday::Wednesday),
                    time_start: Some(time_hm(12, 0)),
                    expired: Some(false),
                    duration: Some(1i16),
                },
                PartialAvailability {
                    id: None,
                    teacher_id: None,
                    weekday: Some(Weekday::Wednesday),
                    time_start: Some(time_hm(13, 0)),
                    expired: Some(false),
                    duration: Some(1i16),
                },
                PartialAvailability {
                    id: None,
                    teacher_id: None,
                    weekday: Some(Weekday::Friday),
                    time_start: Some(time_hm(10, 0)),
                    expired: Some(false),
                    duration: Some(1i16),
                },
                PartialAvailability {
                    id: None,
                    teacher_id: None,
                    weekday: Some(Weekday::Saturday),
                    time_start: Some(time_hm(17, 0)),
                    expired: Some(false),
                    duration: Some(1i16),
                },
                PartialAvailability {
                    id: None,
                    teacher_id: None,
                    weekday: Some(Weekday::Saturday),
                    time_start: Some(time_hm(18, 0)),
                    expired: Some(false),
                    duration: Some(1i16),
                },
                PartialAvailability {
                    id: None,
                    teacher_id: None,
                    weekday: Some(Weekday::Saturday),
                    time_start: Some(time_hm(19, 0)),
                    expired: Some(false),
                    duration: Some(1i16),
                },
                PartialAvailability {
                    id: None,
                    teacher_id: None,
                    weekday: Some(Weekday::Saturday),
                    time_start: Some(time_hm(20, 0)),
                    expired: Some(false),
                    duration: Some(1i16),
                },
                PartialAvailability {
                    id: None,
                    teacher_id: None,
                    weekday: Some(Weekday::Saturday),
                    time_start: Some(time_hm(21, 0)),
                    expired: Some(false),
                    duration: Some(1i16),
                },
            ]
        );
    }
}
