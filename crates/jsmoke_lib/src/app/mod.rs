//! # `app` module
//!
//! This module is intended to provide the app's basic features, such
//! as the default `Command` object (from [`clap`] crate) and its
//! subcommands + their struct env. contexts as well.
//!
//! The app is built through a trait (called [`App`]) function that
//! generates the schema. All the subcommands can be found at
//! `cli::subcommands` private module.
//!
//! The subcommand's context dependecies can be found at
//! [`contexts`] crate.
//!
//! Each context type can implement a [`crate::utils`] trait/feature
//! to help on input handling.

mod cli;
mod contexts;

pub use cli::App;
pub use cli::AppOutput;
