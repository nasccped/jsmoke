use std::process::{self, ExitCode};

/// Exit's the current program with the provided exit code.
pub fn with_code<T: Into<ExitCode>>(code: T) -> ! {
    match code.into() {
        ExitCode::SUCCESS => process::exit(0),
        _ => process::exit(1),
    }
}
