use std::time;

/// Generates a custom ID for Interactions,
/// using the current Unix timestamp.
pub fn generate_custom_id() -> String {
    let timestamp = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("Time is broken");

    format!("minirustbot-{}", timestamp.as_millis())
}
