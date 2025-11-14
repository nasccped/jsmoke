//! # JSmoke CLI
//!
//! This module provides the JSmoke cli parser and application traits
//! implementer.

mod app;
pub mod prelude;
use clap::Parser;

/// Returns the parsed cli app ([`app::JsmkApp`]). This function is
/// just an wrapper for [`clap::Parser::parse`] function. Any error
/// or something else (`parse` call related) is treated within the
/// [`clap`] lib.
pub fn parse() -> app::JsmkApp {
    app::JsmkApp::parse()
}
