mod cli;
mod runtime;
mod shared_models;
mod utils;

use clap::Parser;
use runtime::AppRunner;

fn main() {
    let app = cli::App::parse();
    let output = AppRunner::run(app);
    AppRunner::exit_with_code(output);
}
