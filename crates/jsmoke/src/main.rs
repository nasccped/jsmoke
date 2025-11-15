#![doc = include_str!("../README.md")]

use jsmoke_utils::exit;

const PROGRAM_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // unused var
    let _app = jsmoke_cli::parse(PROGRAM_VERSION);
    exit::exit_with_code(0);
}
