use super::{
    errors::ArtifactWrapperParseError as ParseError, input_checker::InputChecker as Checker,
};
use crate::{
    shared_models::reserveds::JavaReserveds,
    utils::{InputFix, MayFrom},
};
use std::str::FromStr;

/// Wrapper struct for project artifact name.
#[derive(Debug, Clone)]
pub enum ArtifactWrapper {
    /// When the string is already allowed by the java constraints.
    Allowed(String),
    /// When the provided string is 'displayable' but gonna be fixed to a `code version`.
    Fixed {
        /// Display version.
        display: String,
        /// Code version.
        literal: String,
    },
}

/// Type alias for [`ARTIFACT_ERR_MAPPING`].
type ErrMapping = (for<'a> fn(&'a str) -> bool, fn(String) -> ParseError);

const ARTIFACT_ERR_MAPPING: [ErrMapping; 2] = [
    (Checker::is_empty, |_: String| ParseError::Empty),
    (
        |x: &str| !Checker::is_allowed_length(x),
        |s: String| ParseError::Length(s.len()),
    ),
];

impl FromStr for ArtifactWrapper {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.input_fix(|s| {
            s.replace("_", "-")
                .to_lowercase()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        });
        for (check, conversor) in ARTIFACT_ERR_MAPPING {
            if check(&input) {
                return Err(conversor(input));
            }
        }
        input = self_string_or_reserved(input)?;
        match input {
            x if Checker::is_allowed_pattern(&x) => Ok(Self::Allowed(x)),
            x if Checker::is_fixable_pattern(&x) => {
                let literal = self_string_or_reserved(x.clone().input_fix(|s| s.replace("-", "")))?;
                Ok(Self::Fixed {
                    display: x,
                    literal,
                })
            }
            other => Err(ParseError::InvalidPattern(other)),
        }
    }
}

/// Returns a [`Result`] over `s` [`String`] and [`ParseError`]. Checks just the
/// [`ParseError::Reserved`] variant.
fn self_string_or_reserved(s: String) -> Result<String, ParseError> {
    if let Some(reserved) = JavaReserveds::may_from(&s) {
        Err(ParseError::Reserved(reserved))
    } else {
        Ok(s)
    }
}

#[cfg(test)]
mod test {
    use super::ArtifactWrapper;
    use crate::utils::SurelyUnwrap;
    use std::str::FromStr;

    #[test]
    fn allowed() {
        ["SomeArtifact", "CoolApp", "WillWork", "With10Number"]
            .into_iter()
            .for_each(|artifact| {
                let result = ArtifactWrapper::from_str(artifact).surely_unwrap();
                assert!(
                    matches!(result, ArtifactWrapper::Allowed(_)),
                    "'{}' was expected to return `ArtifactWrapper::Allowed` but returned {:?}",
                    artifact,
                    result
                );
            });
    }
}
