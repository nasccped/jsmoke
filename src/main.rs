mod cli;
mod exit;

use clap::Parser;
use std::process::ExitCode;

fn main() {
    let mut _app = cli::App::parse();
    // stop clippy arguing.
    exit::with_code(ExitCode::SUCCESS);
}
