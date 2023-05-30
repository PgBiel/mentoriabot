use super::{calendar::CalendarManager, email::GmailManager};
use crate::error::Result;

/// Manages Google API-related structs.
#[derive(Clone)]
pub struct GoogleApiManager {
    pub calendar: CalendarManager,
    pub email: GmailManager,
}

impl GoogleApiManager {
    /// Connects to the Google API with the given authenticator; more specifically, to Google
    /// Calendar (with the given calendar ID) and to Gmail (with the given user ID).
    pub async fn connect(
        auth: impl google_apis_common::GetToken + Clone + 'static,
        calendar_id: &str,
        user_id: &str,
    ) -> Result<Self> {
        Ok(Self {
            calendar: CalendarManager::connect(auth.clone(), calendar_id).await?,
            email: GmailManager::connect(auth, user_id).await?,
        })
    }
}
