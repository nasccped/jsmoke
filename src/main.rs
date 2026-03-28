mod cli;
mod exit;
mod runtime;
mod utils;

use clap::Parser;

fn main() {
    let app = cli::App::parse();
    let output = runtime::run(app);
    exit::with_code(output);
}
