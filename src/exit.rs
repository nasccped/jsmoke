use std::process::{self, ExitCode};

/// Exit's the current program with the provided exit code.
pub fn with_code(code: ExitCode) -> ! {
    match code {
        ExitCode::SUCCESS => process::exit(0),
        _ => process::exit(1),
    }
}
