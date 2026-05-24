mod cli;
mod exit;
mod shared_models;
mod utils;

use clap::Parser;
use std::process::ExitCode;

fn main() {
    let _app = cli::App::parse();
    exit::with_code(ExitCode::SUCCESS);
}
