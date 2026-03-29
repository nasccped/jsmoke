mod common;

use crate::cli::App;
use std::process::ExitCode;

/// Type that mimics the a [`Result`] enum since I can't implement `Into<ExitCode>` for foreign
/// types.
pub enum Output {
    /// Ok variant.
    Ok(OkVariant),
    /// Error variant.
    Err(ErrVariant),
}

/// Temp type for Ok shadowing...
pub struct OkVariant;
/// Temp type for Err shadowing...
pub struct ErrVariant;

#[allow(clippy::from_over_into)]
impl Into<ExitCode> for Output {
    fn into(self) -> ExitCode {
        match self {
            Output::Ok(_) => ExitCode::SUCCESS,
            _ => ExitCode::FAILURE,
        }
    }
}

/// Run the jsmoke program based on the [`App`] inner fields.
pub fn run(app: App) -> Output {
    if app.verbose {
        println!("verbose is enabled");
    }
    if app.force {
        println!("forcing action...");
        Output::Ok(OkVariant)
    } else {
        eprintln!("no action being forced! Returning an err");
        Output::Err(ErrVariant)
    }
}
