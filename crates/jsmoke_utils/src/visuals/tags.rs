//! # Tags module
//!
//! Provide tag kinds (`error`, `warning`, ...), printing/parsing &
//! its features.

use super::color_highlights::ColorHighlights;
use crate::verbose::Verbose;
use colored::Colorize;
use thiserror::Error;

/// Represents [`TagKind`] as string.
const TAGKIND_REPR: &str = "`TagKind`";

/// Represents [`self`] module as string.
const TAGSMOD_REPR: &str = "`jsmoke_utils::visuals::tags`";

/// Represents [`str`] slice as string.
const STRING_SLICE_REPR: &str = "`&str`";

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
            "{}:",
            match self {
                Self::ErrorKind => "error".bright_red(),
                Self::WarningKind => "warn".bright_yellow(),
                Self::NoteKind => "note".bright_cyan(),
                Self::OkKind => "ok".bright_green(),
            }
        )
    }
}

impl<'a> TryFrom<&'a str> for TagKind {
    type Error = UnparseableTagKind;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value.to_lowercase().trim() {
            "error" => Ok(Self::ErrorKind),
            "warn" => Ok(Self::WarningKind),
            "note" => Ok(Self::NoteKind),
            "ok" => Ok(Self::OkKind),

            // pass any other string (even empty) to `from` method
            // since they all are invalid
            other => Err(UnparseableTagKind::from(other)),
        }
    }
}

/// Error obj. when TagKind parse fails.
#[derive(Error, Debug, PartialEq)]
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
        let mut s = format!(
            "This occurs when trying to generate a {} from an invalid {} value.\n",
            TAGKIND_REPR.type_highlight(),
            STRING_SLICE_REPR.type_highlight()
        );
        s.push_str(
            format!(
                "Current available {} values: \n",
                TAGKIND_REPR.type_highlight()
            )
            .as_str(),
        );
        AVAILABLE_KINDS
            .iter()
            .for_each(|kind| s.push_str(format!("  - {}\n", kind).as_str()));
        s.push_str(
            format!(
                "{} {} referes to {} module.",
                TagKind::NoteKind,
                TAGKIND_REPR.type_highlight(),
                TAGSMOD_REPR.module_highlight()
            )
            .as_str(),
        );
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_parsing() {
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
