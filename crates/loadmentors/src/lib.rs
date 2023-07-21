//! Module responsible for loading mentors from a CSV file in a certain format
//! into the database.

mod reader;

/// Dependency re-exports.
pub mod exports {
    /// Re-exports the bot's library crate.
    pub mod lib {
        pub use mentoriabot_lib::*;
    }
}

pub fn load(
    csv_content: &str,
    database_url: &str,
) -> exports::lib::error::Result<Vec<exports::lib::model::Teacher>> {
    todo!()
}
