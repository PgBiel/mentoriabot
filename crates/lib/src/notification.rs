//! Structs and functions related to notifying users through some channel;
//! e.g., calendar or e-mail.
mod calendar;
mod email;
mod google;

pub use calendar::CalendarManager;
pub use email::GmailManager;
pub use google::GoogleApiManager;
