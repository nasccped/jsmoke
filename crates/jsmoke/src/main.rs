#![doc = include_str!("../README.md")]

use jsmoke_cli::prelude::app::JsmkApp;
use jsmoke_utils::exit;

/// # Override the program version
///
/// Since [`JsmkApp`] is generated within the [`jsmoke_cli`] lib,
/// it'll take the lib version when trying to parse the object.
/// Instead, we should call the [`jsmoke_cli::parse`] function and
/// provide the current crate version (same as the binary final
/// binary).
const PROGRAM_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let app: JsmkApp = jsmoke_cli::parse(PROGRAM_VERSION);
    if app.verbose {
        println!("Exiting error with verbose!");
        exit::exit_with_code(1);
    } else {
        println!("No verbose? No error!");
    }
}
