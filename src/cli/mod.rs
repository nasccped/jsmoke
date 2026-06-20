//! # Cli module
//!
//! Provides the path for all CLI stuff: [`App`] struct, it's subcommands and err parse enum
//! wrapper.
mod app;
pub mod error;

pub use app::App;
pub use error::AppParseFail;
