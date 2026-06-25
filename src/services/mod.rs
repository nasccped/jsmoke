pub mod error;
mod new_service;
mod service_variant;

use crate::{
    cli::{App, AppParseFail, subcommands::AppSubcommands},
    services::error::ServiceParseError,
    utils::notifier::Notifier,
};
use new_service::NewService;
use service_variant::AppServiceVariant;
use std::{
    ops::Deref,
    process::{self, ExitCode},
};

/// Does the [`crate::cli::App`] runtime stuff.
pub struct AppService<'a> {
    /// If the operation is being verbose.
    verbose: bool,

    /// If the operation is being forced.
    force: bool,

    /// Service variant to run.
    service: AppServiceVariant<'a>,
}

impl<'a> TryFrom<&'a App> for AppService<'a> {
    type Error = ServiceParseError<'a>;

    fn try_from(value: &'a App) -> Result<Self, Self::Error> {
        let verbose = value.is_verbose();
        let force = value.is_forced();
        let service = match value.subcommand() {
            AppSubcommands::New(cmd) => NewService::try_from(cmd),
        };
        service
            .map(|srvc| Self {
                verbose,
                force,
                service: AppServiceVariant::from(srvc),
            })
            .map_err(|err| {
                let mut spe = ServiceParseError::from(err);
                spe.set_verbose(verbose);
                spe
            })
    }
}

impl<'a> AppService<'a> {
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
