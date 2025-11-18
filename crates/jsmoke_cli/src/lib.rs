#![doc = include_str!("../README.md")]

mod app;
pub mod common;
pub mod prelude;
use clap::{CommandFactory, FromArgMatches};

const EXPECTED_MESSAGE: &str =
    "this is intended to be Some since it's constructed from the self object";

/// Returns the parsed CLI app ([`app::JsmkApp`]). This function is
/// just an wrapper for [`clap::Parser::parse`] function. Errors or
/// something else (`parse` call related) is treated within the
/// [`clap`] lib.
///
/// A new version value should be given (use binary version instead
/// of lib version).
pub fn parse(version: &'static str) -> app::JsmkApp {
    let app_cmd = app::JsmkApp::command().version(version);
    app::JsmkApp::from_arg_matches(&app_cmd.get_matches()).expect(EXPECTED_MESSAGE)
}
