//! Autocompletion methods.

use crate::common::Context;

/// Autocomplete a user's session IDs as student.
pub async fn autocomplete_student_sessions(
    ctx: Context<'_>,
    partial: &str,
    only_active: bool,
) -> Vec<u32> {
    let Ok(sessions) = ctx.data()
        .db.session_repository()
        .find_student_autocomplete(ctx.author().id.into(), partial, only_active)
        .await
        .map_err(|err| tracing::warn!("Student session autocomplete couldn't talk with the DB: {err}."))
    else {
        return Vec::new();
    };

    sessions
        .into_iter()
        .map(|session| session.id as u32)
        .collect()
}

/// Autocomplete a user's active or inactive session IDs as student.
pub async fn autocomplete_any_student_sessions(ctx: Context<'_>, partial: &str) -> Vec<u32> {
    autocomplete_student_sessions(ctx, partial, true).await
}

/// Autocomplete a user's active only (i.e. which weren't yet started) session IDs as student.
pub async fn autocomplete_active_student_sessions(ctx: Context<'_>, partial: &str) -> Vec<u32> {
    autocomplete_student_sessions(ctx, partial, false).await
}
