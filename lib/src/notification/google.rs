use google_calendar3::oauth2;

use super::{calendar::CalendarManager, email::GmailManager};
use crate::error::Result;

/// Manages Google API-related structs.
pub struct GoogleApiManager {
    calendar: CalendarManager,
    email: GmailManager,
}

impl GoogleApiManager {
    /// Connects to the Google API with the given secret; more specifically, to Google Calendar
    /// (with the given calendar ID) and to Gmail (with the given user ID).
    pub async fn connect(
        secret: oauth2::ApplicationSecret,
        calendar_id: &str,
        user_id: &str,
    ) -> Result<Self> {
        Ok(Self {
            calendar: CalendarManager::connect(secret.clone(), calendar_id).await?,
            email: GmailManager::connect(secret, user_id).await?,
        })
    }
}
