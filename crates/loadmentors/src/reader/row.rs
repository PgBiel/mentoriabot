//! Implements the Teacher Row type, a.k.a. the expected CSV format.
use csv::Result as CsvResult;

use crate::exports::lib::{
    error::{Error, Result},
    model::NewTeacher,
    util::Unvalidated,
};

lazy_static::lazy_static! {
    static ref EMAIL_REGEX: regex::Regex = regex::Regex::new("^.+@.+\\..{2,}$").unwrap();
}

/// Represents a single row in the teachers CSV.
#[derive(validator::Validate, serde::Serialize, serde::Deserialize)]
pub(crate) struct TeacherRow {
    /// The CSV is extracted from Google Forms.
    /// As such, each row starts with a timestamp.
    pub(crate) form_timestamp: String,

    /// The teacher's email.
    #[validate(regex = "EMAIL_REGEX")]
    pub(crate) email: String,

    /// The teacher's full name.
    pub(crate) name: String,

    /// The teacher's WhatsApp-compatible
    /// phone number.
    pub(crate) whatsapp: String,

    /// The teacher's Linkedin URL.
    pub(crate) linkedin: String,

    /// The teacher's academic course
    /// and the educational institution
    /// in which they study.
    pub(crate) course_info: String,

    /// The company the teacher works in,
    /// if any.
    pub(crate) company: String,

    /// The role the teacher has at their
    /// company, if any.
    pub(crate) company_role: String,

    /// The teacher's bio.
    pub(crate) bio: String,

    /// What the teacher is specialized at.
    pub(crate) specialty: String,

    /// Any general comments left by the teacher.
    pub(crate) comment_general: String,

    /// The experience comment left by the teacher,
    /// if any.
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
}

impl TryFrom<TeacherRow> for NewTeacher {
    type Error = Error;

    /// Try to parse the entries in the TeacherRow
    /// to construct a NewTeacher object, which
    /// can be inserted into the database.
    fn try_from(row: TeacherRow) -> Result<Self> {
        todo!()
    }
}
