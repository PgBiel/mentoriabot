//! Module responsible for loading mentors from a CSV file in a certain format
//! into the database.

mod reader;

use mentoriabot_lib::{error::Error, model::NewAvailability};
use reader::ReaderResult;

/// Dependency re-exports.
pub mod exports {
    /// Re-exports the bot's library crate.
    pub mod lib {
        pub use mentoriabot_lib::*;
    }
}

use exports::lib::{self, db::Repository};

/// Reads teachers from a CSV file and inserts them
/// in the database.
pub async fn load_teachers_to_db(
    csv_contents: &str,
    database_url: &str,
) -> lib::error::Result<ReaderResult<(lib::model::Teacher, Vec<lib::model::Availability>)>> {
    let csv_data = match reader::read_teacher_csv(csv_contents)? {
        Ok(new_teachers) => new_teachers,
        Err(line_errors) => return Ok(Err(line_errors)),
    };

    Ok({
        let db = lib::db::DatabaseManager::new(database_url)?;
        let mut results = Vec::new();
        let mut errors = Vec::new();
        results.reserve(csv_data.len());

        for (i, (new_teacher, availabilities)) in csv_data.into_iter().enumerate() {
            if errors.len() > 5 {
                break; // 5 errors max is good enough
            }

            let teacher = match db.teacher_repository().insert(&new_teacher).await {
                Ok(teacher) => teacher,
                Err(db_err) => {
                    errors.push((i + 1, db_err));
                    continue;
                }
            };

            let mut inserted_availabilities = Vec::new();

            for availability in availabilities {
                let unexpected_availability_structure =
                    || Error::Other("unexpected availability structure");
                let new_availability = NewAvailability {
                    teacher_id: teacher.id,
                    weekday: availability
                        .weekday
                        .ok_or_else(unexpected_availability_structure)?,
                    time_start: availability
                        .time_start
                        .ok_or_else(unexpected_availability_structure)?,
                    expired: availability
                        .expired
                        .ok_or_else(unexpected_availability_structure)?,
                    duration: availability
                        .duration
                        .ok_or_else(unexpected_availability_structure)?,
                };

                match db.availability_repository().insert(&new_availability).await {
                    Ok(availability) => inserted_availabilities.push(availability),
                    Err(db_err) => {
                        errors.push((i + 1, db_err));
                        continue;
                    }
                }
            }

            results.push((teacher, inserted_availabilities));
        }

        if errors.is_empty() {
            Ok(results)
        } else {
            Err(errors)
        }
    })
}
