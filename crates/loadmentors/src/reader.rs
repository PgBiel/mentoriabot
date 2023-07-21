//! Implements the task of reading teacher rows from CSV.
mod row;

use csv::Result as CsvResult;

use self::row::TeacherRow;
use crate::exports::lib::{error::Error, model::NewTeacher};

/// If all rows were successfully read and converted to 'T', returns a vector of those.
/// Otherwise, returns a vector with each error and the corresponding line number
/// for the error.
pub(crate) type ReaderResult<T> = std::result::Result<Vec<T>, Vec<(usize, Error)>>;

/// Reads a CSV file for teachers.
/// The outermost Result will error if the CSV failed to parse.
pub(crate) fn read_teacher_csv(csv_contents: &str) -> CsvResult<ReaderResult<NewTeacher>> {
    let rows = TeacherRow::from_csv(csv_contents);
    let mut res_vec: Vec<NewTeacher> = Vec::new();
    res_vec.reserve(rows.len());
    let mut errs_vec = Vec::new();

    for (i, row_res) in rows.into_iter().enumerate() {
        let validated_row = row_res?.validate();
        match validated_row {
            // only consider a valid row if there are no errors already,
            // since we only return valid rows if no errors occurred.
            Ok(row) if !errs_vec.is_empty() => {
                match row.try_into() {
                    Ok(teacher) => res_vec.push(teacher),

                    // error line is i + 1 as we are skipping the header line
                    Err(conversion_err) => errs_vec.push((i + 1, conversion_err)),
                }
            }
            Err(validation_err) => errs_vec.push((i + 1, validation_err.into())),
            _ => {}
        }
    }

    Ok(if !errs_vec.is_empty() {
        Err(errs_vec)
    } else {
        Ok(res_vec)
    })
}
