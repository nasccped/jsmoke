use crate::{cli::AppParseFail, utils::notifier::Notifier};
use std::{
    ops::Deref,
    process::{self, ExitCode},
};

/// Does the [`crate::cli::App`] runtime stuff.
pub struct AppService;

impl AppService {
    /// Handle the [`AppParseFail`] returned from [`clap::Parser::try_parse`] function.
    ///
    /// The reason of this error handling is explained at [`AppParseFail`] documentation.
    ///
    /// An [`ExitCode`] value is returned at the end of program.
    pub fn handle_parse_fail(fail: AppParseFail) -> ExitCode {
        // auto set is_err to true if necessary.
        let mut notifier = Notifier::from(&fail);
        match fail {
            AppParseFail::IsError(error) => {
                notifier.notify_failure(&error);
                notifier.notify_verbose(&error);
                ExitCode::FAILURE
            }
            AppParseFail::ActionCall(action) => {
                notifier.notify_simple(action.deref());
                ExitCode::SUCCESS
            }
        }
    }

    /// Exits the current process with the provided [`ExitCode`]:
    /// - `SUCCESS` means `0`
    /// - `<other>` means `1`
    pub fn exit_with_code(code: ExitCode) {
        let code = match code {
            ExitCode::SUCCESS => 0,
            _ => 1,
        };
        process::exit(code);
    }
}
