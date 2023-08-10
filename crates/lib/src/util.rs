//! Utility functions for the bot.
pub mod bases;
pub mod forms;
pub mod iter;
pub mod locale;
pub mod macros;
pub mod string;
pub mod time;
mod validation;

pub use forms::{apply_limits_to_select_menu_spec, apply_limits_to_select_option_spec};
pub use macros::tr;
pub use string::limit_string_len;
pub use time::{HumanParseableDateTime, BRAZIL_TIMEZONE};
pub use validation::Unvalidated;
