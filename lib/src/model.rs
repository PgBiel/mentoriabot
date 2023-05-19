//! Model structs used in the bot, usually interfacing with the DB.
mod availability;
mod discordid;
mod session;
mod teacher;
mod user;
mod weekday;

pub use availability::{Availability, NewAvailability, PartialAvailability};
pub use discordid::DiscordId;
pub use session::{NewSession, PartialSession, Session};
pub use teacher::{NewTeacher, PartialTeacher, Teacher};
pub use user::{NewUser, PartialUser, User};
pub use weekday::Weekday;
