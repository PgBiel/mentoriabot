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
            format!("{} hours and {} minutes", dur.num_hours(), dur.num_minutes())
        } else {
            format!("{} days", dur.num_days())
        }
    } else {
        format!("{} seconds", duration.as_secs())
    }
}
