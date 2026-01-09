use std::process::ExitCode;

mod exit;

fn main() {
    println!("jsmoke rebooting...");
    exit::with_code(ExitCode::SUCCESS);
}
