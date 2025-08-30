//! # `cli` module
//!
//! This module provides the cli build features, such as default the
//! [`clap::Command`] (through [`App::generate`]) with their
//! subcommands (from [`subcommands`] module) as well.

mod app;
mod subcommands;
pub use app::App;
pub use app::AppOutput;
