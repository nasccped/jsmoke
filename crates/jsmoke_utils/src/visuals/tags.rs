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
        match self {
            TagKind::ErrorKind => ERROR,
            TagKind::WarningKind => WARNING,
            TagKind::NoteKind => NOTE,
            TagKind::OkKind => OK,
        }
    }
}

/// private available kinds (used to print within
/// [`Verbose::print_verbose`] function).
const AVAILABLE_KINDS: [TagKind; 4] = [
    TagKind::ErrorKind,
    TagKind::WarningKind,
    TagKind::NoteKind,
    TagKind::OkKind,
];

impl std::fmt::Display for TagKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: ",
            match self {
                Self::ErrorKind => ERROR.bright_red(),
                Self::WarningKind => WARNING.bright_yellow(),
                Self::NoteKind => NOTE.bright_cyan(),
                Self::OkKind => OK.bright_green(),
            }
        )
    }
}

impl<'a> TryFrom<&'a str> for TagKind {
    type Error = UnparseableTagKind;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value.to_lowercase().trim() {
            ERROR => Ok(Self::ErrorKind),
            WARNING => Ok(Self::WarningKind),
            NOTE => Ok(Self::NoteKind),
            OK => Ok(Self::OkKind),

            // pass any other string (even empty) to `from` method
            // since they all are invalid
            other => Err(UnparseableTagKind::from(other)),
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

impl From<&str> for UnparseableTagKind {
    fn from(value: &str) -> Self {
        if value.is_empty() {
            Self::EmptyKind
        } else {
            Self::InvalidKind(value.to_string())
        }
    }
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
            tk,
            str_slc,
            tk,
            available,
            TagKind::NoteKind,
            cur_mod
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
            ("error", TagKind::ErrorKind),
            ("warn", TagKind::WarningKind),
            ("note", TagKind::NoteKind),
            ("ok", TagKind::OkKind),
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
