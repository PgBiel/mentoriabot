//! Module responsible for loading mentors from a CSV file in a certain format
//! into the database.

mod reader;

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
) -> lib::error::Result<ReaderResult<lib::model::Teacher>> {
    let new_teachers = match reader::read_teacher_csv(csv_contents)? {
        Ok(new_teachers) => new_teachers,
        Err(line_errors) => return Ok(Err(line_errors)),
    };

    Ok({
        let db = lib::db::DatabaseManager::new(database_url)?;
        let mut teachers = Vec::new();
        let mut errors = Vec::new();
        teachers.reserve(new_teachers.len());

        for (i, new_teacher) in new_teachers.into_iter().enumerate() {
            if errors.len() > 5 {
                break; // 5 errors max is good enough
            }
            match db.teacher_repository().insert(&new_teacher).await {
                Ok(teacher) => teachers.push(teacher),
                Err(db_err) => errors.push((i + 1, db_err.into())),
            }
        }

        if errors.is_empty() {
            Ok(teachers)
        } else {
            Err(errors)
        }
    })
}
