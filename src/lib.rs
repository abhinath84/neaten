mod config;
mod engine;
mod error;
mod manager;

pub use config::{Config, Kind};
pub use engine::Engine;
pub use manager::Manager;

pub type Result<T> = std::result::Result<T, error::Error>;
