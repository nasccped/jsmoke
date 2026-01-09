//! # Group submodule
//!
//! Provides the [`ProjectGroup`] parsing features.
use crate::types::dot_notation;
use jsmoke_utils::print_verbose::PrintVerbose;
use thiserror::Error;

/// The project group object. Holds the content as a
/// [`dot_notation::DotNotation`] value (similar to `pom.xml` values)
/// which items means a subgroup (separated by a dot (`.`)).
///
/// ## Parsing
///
/// This object should be parsed from a [`str`] slice using the
/// [`FromStr`] trait. The output value will be
/// [`Result<Self, ParseError>`], which [`Err`] holds an instance of
/// [`dot_notation::ParseError`] for verbose printing.
#[derive(Debug)]
pub struct ProjectGroup(dot_notation::DotNotation);

impl TryFrom<&str> for ProjectGroup {
    type Error = ParseError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        dot_notation::DotNotation::try_from(s)
            .map(Self)
            .map_err(|e| ParseError {
                provided: s.into(),
                parse_error: e,
            })
    }
}

/// Error to return when [`ProjectGroup`]'s parse fails. Even if the
/// [`ParseError::print_verbose`] function does the same as
/// [`dot_notation::ParseError::print_verbose`] one, the error title
/// changes by context, so is necessary to holds the parsing error
/// within a new type.
#[derive(Debug, Error)]
#[error("fail to parse the provided project group (`{}`)", .provided)]
pub struct ParseError {
    /// The provided project group.
    provided: String,
    /// The [`dot_notation::ParseError`] instance for the provided
    /// input.
    parse_error: dot_notation::ParseError,
}

impl PrintVerbose for ParseError {
    fn print_verbose(&self) {
        self.parse_error.print_verbose();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERR_SAMPLES: [&str; 4] = ["some. .empty", "no..space", "cool.group.😎", "lets.test.1t"];
    const OK_SAMPLES: [&str; 3] = ["input.number1", "input.number.two", "cool.thing"];

    #[test]
    fn parse_error() {
        ERR_SAMPLES
            .into_iter()
            .for_each(|input| assert!(ProjectGroup::try_from(input).is_err()));
    }

    #[test]
    fn parse_ok() {
        OK_SAMPLES
            .into_iter()
            .for_each(|input| assert!(ProjectGroup::try_from(input).is_ok()));
    }
}
