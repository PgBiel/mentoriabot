//! Module for all things database in the bot.
mod connection;
mod repository;
pub mod schema;

pub use connection::DatabaseManager;
pub use repository::{
    AvailabilityRepository, Repository, SessionRepository, TeacherRepository, UpdatableRepository,
    UserRepository,
};
