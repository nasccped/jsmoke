//! # JSmoke CLI
//!
//! This module provides the JSmoke CLI parser and application traits
//! implementer.

mod app;
pub mod prelude;
use clap::Parser;

/// Returns the parsed CLI app ([`app::JsmkApp`]). This function is
/// just an wrapper for [`clap::Parser::parse`] function. Errors or
/// something else (`parse` call related) is treated within the
/// [`clap`] lib.
pub fn parse() -> app::JsmkApp {
    app::JsmkApp::parse()
}
