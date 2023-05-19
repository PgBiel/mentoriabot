//! Main library crate for all the inner machinery the bot uses,
//! including database, errors, utilities, notification systems, and more.
pub mod connection;
pub mod error;
pub mod model;
pub mod notification;
pub mod repository;
mod schema;
pub mod util;
