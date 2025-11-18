//! # Tags module
//!
//! Provide tag kinds (`error`, `warning`, ...), printing/parsing &
//! its features.

use super::color_highlights::ColorHighlights;
use crate::{
    printing::error_print::ErrorPrint,
    strings::repr::{STRING_SLICE, TAG_KIND, TAGS_MODULE},
    verbose::Verbose,
};
use colored::Colorize;
use jsmoke_macros::ErrorPrint;
use thiserror::Error;

const ERROR: &str = "error";
const WARNING: &str = "warn";
const NOTE: &str = "note";
const OK: &str = "ok";

/// Public [`TagKind::ErrorKind`] variant.
pub const ERROR_TAG: TagKind = TagKind::ErrorKind;
/// Public [`TagKind::WarningKind`] variant.
pub const WARNING_TAG: TagKind = TagKind::WarningKind;
/// Public [`TagKind::NoteKind`] variant.
pub const NOTE_TAG: TagKind = TagKind::NoteKind;
/// Public [`TagKind::OkKind`] variant.
pub const OK_TAG: TagKind = TagKind::OkKind;

/// Tag kind when reporting messages to user.
#[derive(Debug, PartialEq)]
pub enum TagKind {
    /// Something went wrong.
    ErrorKind,
    /// The job was partially done/other essential info.
    WarningKind,
    /// Just note.
    NoteKind,
    /// The job was done successfully!
    OkKind,
}

impl TagKind {
    /// Converts the provided [`TagKind`] into a simple string (no
    /// color/colon).
    fn as_simple_str(&self) -> &'static str {
        match *self {
            ERROR_TAG => ERROR,
            WARNING_TAG => WARNING,
            NOTE_TAG => NOTE,
            OK_TAG => OK,
        }
    }
}

/// private available kinds (used to print within
/// [`Verbose::print_verbose`] function).
const AVAILABLE_KINDS: [TagKind; 4] = [ERROR_TAG, WARNING_TAG, NOTE_TAG, OK_TAG];

impl std::fmt::Display for TagKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: ",
            match *self {
                ERROR_TAG => ERROR.bright_red(),
                WARNING_TAG => WARNING.bright_yellow(),
                NOTE_TAG => NOTE.bright_cyan(),
                OK_TAG => OK.bright_green(),
            }
        )
    }
}

impl<'a> TryFrom<&'a str> for TagKind {
    type Error = UnparseableTagKind;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value.to_lowercase().trim() {
            // Ok cases:
            ERROR => Ok(Self::ErrorKind),
            WARNING => Ok(Self::WarningKind),
            NOTE => Ok(Self::NoteKind),
            OK => Ok(Self::OkKind),

            // Err cases:
            "" => Err(UnparseableTagKind::EmptyKind),
            other => Err(UnparseableTagKind::InvalidKind(other.into())),
        }
    }
}

/// Error obj. when TagKind parse fails.
#[derive(Error, Debug, PartialEq, ErrorPrint)]
pub enum UnparseableTagKind {
    /// The provide [`str`] isn't a valid TagKind.
    #[error("couldn't parse the provided kind value (`{0}`)")]
    InvalidKind(String),
    /// The provide [`str`] is empty.
    #[error("couldn't parse the kind value (empty string)")]
    EmptyKind,
}

impl Verbose for UnparseableTagKind {
    fn get_verbose(&self) -> String {
        let tk = TAG_KIND.type_highlight();
        let str_slc = STRING_SLICE.type_highlight();
        let available = AVAILABLE_KINDS
            .iter()
            .map(|k| format!("  - {}", k.as_simple_str()))
            .collect::<Vec<_>>()
            .join("\n");
        let cur_mod = TAGS_MODULE.module_highlight();

        format!(
            "\
            This occurs when trying to generate a {} from an\n\
            invalid {} value.\n\
            \n\
            Currently available {} values:\n\
            {}\n\
            \n\
            {}refers to {} module.",
            tk, str_slc, tk, available, NOTE_TAG, cur_mod
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_parsing() {
        // use literals to avoid accidental const changes.
        [
            ("error", ERROR_TAG),
            ("warn", WARNING_TAG),
            ("note", NOTE_TAG),
            ("ok", OK_TAG),
        ]
        .into_iter()
        .for_each(|(s, k)| assert_eq!(TagKind::try_from(s), Ok(k)))
    }

    #[test]
    fn err_parsing() {
        [
            ("   ", UnparseableTagKind::EmptyKind),
            (
                " not valid ",
                UnparseableTagKind::InvalidKind("not valid".into()),
            ),
            ("Bob", UnparseableTagKind::InvalidKind("bob".into())),
        ]
        .into_iter()
        .for_each(|(s, e)| assert_eq!(TagKind::try_from(s), Err(e)));
    }
}
