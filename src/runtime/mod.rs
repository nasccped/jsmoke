use crate::cli::App;
use std::process::{self, ExitCode};

/// Does the runtime for the entire [`App`].
pub struct AppRunner;

impl AppRunner {
    /// Runs based on the provided [`App`] item.
    pub fn run(app: App) -> ExitCode {
        // silence the compiler.
        let _ = app;
        ExitCode::SUCCESS
    }

    /// Exits the current program with the provided [`ExitCode`].
    pub fn exit_with_code(code: ExitCode) {
        process::exit(match code {
            ExitCode::SUCCESS => 0,
            _ => 1,
        });
    }
}
