use crate::utils::{Verbose, styler::Styler};
use clap::{
    builder::StyledStr,
    error::{ContextKind, ContextValue, Error as ClapError, ErrorKind, RichFormatter},
};
use std::{borrow::Cow, fmt::Display, ops::Deref};

/// When the parsing ([clap::Parser::try_parse]) fails.
///
/// There's two distinct variants:
/// 1. [`ActionCall`]
/// 2. [`SureError`]
///
/// By default, clap parsing treats `--help` and `--version` calls as parsing errors, so, the
/// [clap::Parser::parse] can detect this error, print the necessary stuff and exit the program
/// with success status code.
///
/// Since the error is being wrapped in a custom enum, we should differ between [`ActionCall`]
/// (it isn't a subcommand but must be handled normally) and  [`SureError`] (that's an error and
/// must be treated as an error).
pub enum AppParseFail {
    /// A common behavior that must be handled.
    ActionCall(ActionCall),

    /// Certainly an error.
    IsError(SureError),
}

/// Different kind of actions that are actually treated as errors but ain't, actually.
pub enum ActionCall {
    /// When `help` subcommand or flag (`--help`) is called.
    HelpCall { message: ActionCallMessage },

    /// When `--version`|`-V` flag is called.
    VersionCall { version: ActionCallMessage },
}

/// An wrapper for [`ActionCall`] inner messages. They can be either no-styled [`String`] or a
/// [`StyledStr`] also.
///
/// Join it in a single type to handle [`Display`] trait in an more simple way instead of boxing
/// values.
pub enum ActionCallMessage {
    /// When the provided message is an [`StyledStr`].
    StyledStr(StyledStr),

    /// When the provided message is literally [`String`].
    String(String),
}

/// Wrapper for [`clap::error::Error`] object. It's used to do printing stuff in a standard way
/// across other app's models parsing errors since [`Parser::parse`] does a `parse` +
/// [`ClapError::exit`] if fails (unable to custom printing).
#[derive(thiserror::Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum SureError {
    /// When non-valid subcommand found within [`super::app::App::try_parse`] function.
    #[error("`{}` isn't a valid subcommand", .0)]
    InvalidSubcommand(String),

    /// Same as [`AppParseError::InvalidSubcommand`] but the passed subcommand couldn't be caught
    /// (through [`ContextValue`] api).
    #[error("an invalid subcommand was provided")]
    UndefinedInvalidSubcommand,

    /// No subcommand passed.
    #[error("no subcommand passed")]
    MissingSubcommand,
}

impl From<ClapError> for AppParseFail {
    fn from(value: ClapError) -> Self {
        match value.kind() {
            ErrorKind::InvalidSubcommand => SureError::invalid_subcmd_from_err(&value).into(),
            ErrorKind::MissingSubcommand => SureError::MissingSubcommand.into(),
            ErrorKind::DisplayVersion => ActionCall::VersionCall {
                version: ActionCallMessage::String(value.render().to_string()),
            }
            .into(),
            ErrorKind::DisplayHelp => ActionCall::help_call_from_err(&value).into(),
            _ => SureError::MissingSubcommand.into(),
        }
    }
}

impl From<SureError> for AppParseFail {
    fn from(value: SureError) -> Self {
        Self::IsError(value)
    }
}

impl From<ActionCall> for AppParseFail {
    fn from(value: ActionCall) -> Self {
        Self::ActionCall(value)
    }
}

impl AppParseFail {
    /// If the inner variant is certainly an error.
    pub fn is_err(&self) -> bool {
        matches!(self, Self::IsError(_))
    }
}

impl Verbose for SureError {
    fn get_verbose_message(&self) -> Option<Cow<'_, str>> {
        Some(match self {
            Self::InvalidSubcommand(_)
            | Self::UndefinedInvalidSubcommand
            | Self::MissingSubcommand => Cow::Owned(format!(
                "Consider using {} to see available commands.",
                "jsmk --help".command_style()
            )),
        })
    }
}

impl SureError {
    /// This function builds a [`SureError`] variant based on the [`ContextValue`] returned when
    /// calling [`ClapError::get`]:
    /// - [`Some`] variant of [`ContextValue::String`] => [`SureError::InvalidSubcommand`] with the
    ///   inner name
    /// - [`None`] variant or a non [`ContextValue::String`] matching =>
    ///   [`SureError::UndefinedInvalidSubcommand`] since the inner name couldn't be caught.
    ///
    /// This function call must be handled at [`AppParseFail::try_from`] actually.
    fn invalid_subcmd_from_err(value: &ClapError<RichFormatter>) -> Self {
        match value.get(ContextKind::InvalidSubcommand) {
            Some(ContextValue::String(s)) => Self::InvalidSubcommand(s.clone()),
            _ => Self::UndefinedInvalidSubcommand,
        }
    }
}

impl Deref for ActionCall {
    type Target = ActionCallMessage;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::HelpCall { message } => message,
            Self::VersionCall { version } => version,
        }
    }
}

impl ActionCall {
    fn help_call_from_err(value: &ClapError<RichFormatter>) -> Self {
        Self::HelpCall {
            message: ActionCallMessage::StyledStr(value.render()),
        }
    }
}

impl Display for ActionCallMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StyledStr(s_str) => write!(f, "{}", s_str.ansi()),
            Self::String(strng) => write!(f, "{}", strng),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::App, *};
    use clap::{Parser, error::RichFormatter};

    /// Matches if a provided [`AppParseFail`] refers to [`SureError::MissingSubcommand`] variant.
    fn is_missing_subcommand(fail: AppParseFail) -> bool {
        matches!(fail, AppParseFail::IsError(SureError::MissingSubcommand))
    }

    /// Matches if a provided [`AppParseFail`] refers to [`SureError::InvalidSubcommand`] variant.
    fn is_invalid_subcommand(fail: AppParseFail) -> bool {
        matches!(fail, AppParseFail::IsError(SureError::InvalidSubcommand(_)))
    }

    /// Matches if a provided [`AppParseFail`] refers to [`ActionCall::HelpCall`] variant.
    fn is_help_call(fail: AppParseFail) -> bool {
        matches!(fail, AppParseFail::ActionCall(ActionCall::HelpCall { .. }))
    }

    /// Matches if a provided [`AppParseFail`] refers to [`ActionCall::VersionCall`] variant.
    fn is_version_call(fail: AppParseFail) -> bool {
        matches!(
            fail,
            AppParseFail::ActionCall(ActionCall::VersionCall { .. })
        )
    }

    /// Type alias for [`SURE_ERRORS`] and [`SURE_ACTION`]:
    ///
    /// > (<left>, <righ>) where:
    /// > - <left> means input
    /// > - <right> means output function matching
    type IOPair = (&'static [&'static str], fn(AppParseFail) -> bool);

    /// Values that certainly returns error:
    const SURE_ERRORS: [IOPair; 2] = [
        (&["jsmk"], is_missing_subcommand),
        (&["jsmk", "random-subcommand"], is_invalid_subcommand),
    ];

    const SURE_ACTION: [IOPair; 4] = [
        (&["jsmk", "--help"], is_help_call),
        (&["jsmk", "-h"], is_help_call),
        (&["jsmk", "--version"], is_version_call),
        (&["jsmk", "-V"], is_version_call),
    ];

    /// Gets the [`AppParseFail`] from a given [`Parser::try_parse`] result (this function) expects
    /// the result to be [`Err`], otherwise it'll panic.
    fn get_err(result: Result<App, ClapError<RichFormatter>>) -> AppParseFail {
        match result.map_err(AppParseFail::from) {
            Err(e) => e,
            Ok(val) => panic!(
                "the `result` value was expected to be error but\n`Ok({:?})` was returned",
                val
            ),
        }
    }

    #[test]
    fn expecting_error() {
        SURE_ERRORS.into_iter().for_each(|(input, match_func)| {
            assert!(match_func(get_err(App::try_parse_from(input))))
        });
    }

    #[test]
    fn expecting_action() {
        SURE_ACTION.into_iter().for_each(|(input, match_func)| {
            assert!(match_func(get_err(App::try_parse_from(input))))
        });
    }
}
