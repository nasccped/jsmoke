mod exit;

use std::process::ExitCode;

fn main() {
    println!("jsmoke is about to run :^D");
    exit::with_code(ExitCode::SUCCESS);
}
