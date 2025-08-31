use super::subcommands;
use crate::config::app_fields::*;
use clap::Command;

/// Provides app's basics features.
///
/// ## Warning
///
/// This trait holds a generic since the app's output type can change
/// during the program development.
pub trait App<OutputType> {
    /// Default app constructor
    fn generate() -> Command {
        Command::new(APP_NAME)
            .version(APP_VERSION)
            .about(APP_ABOUT)
            .author(APP_AUTHOR)
            .styles(APP_STYLE)
            .subcommands([subcommands::new()])
    }

    /// Execute the app object based on a given input
    /// ([`Vec<String>`]).
    ///
    /// ## Warning
    ///
    /// This function is implemented for the [`Command`] type and
    /// should be ran only by the [`Command`] object generated from
    /// the [`App::generate`] function. Otherwise, it'll panic at
    /// runtime!
    fn execute_app(&self, input: Vec<String>) -> OutputType;

    /// Converts app run's output into machine's exit code ([`i32`])
    fn into_exit_code(output: OutputType) -> i32;
}

impl App<AppOutput> for Command {
    fn execute_app(&self, input: Vec<String>) -> AppOutput {
        let _input = input;
        todo!("implement the execution logic");
    }

    fn into_exit_code(output: AppOutput) -> i32 {
        match output {
            AppOutput::Ok => 0,
            AppOutput::NotOk => 1,
        }
    }
}

/// A common output type to replace [`Result<Ok(_), Err(_)>`] one.
///
/// We don't care about values since all error reports are
/// done during the lib run time (the user shouldn't handle the final
/// value).
pub enum AppOutput {
    Ok,
    NotOk,
}
