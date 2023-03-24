use std::time;
use crate::{CustomId, HasCustomId};

/// Generates a Custom ID for Interactions,
/// using the current Unix timestamp.
pub fn generate_custom_id() -> String {
    let timestamp = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("Time is broken");

    format!("minirustbot-{}", timestamp.as_millis())
}

/// Extracts Custom IDs from a Vec
/// of types that implement [`HasCustomId`].
pub fn id_vec_from_has_custom_ids(vec: &Vec<impl HasCustomId>) -> Vec<&CustomId> {
    vec.iter()
        .map(HasCustomId::get_custom_id)
        .flat_map(|maybe_id| {
          if let Some(id) = maybe_id {
              vec![id]
          } else {
              Vec::new()
          }
        })
        .collect()
}
