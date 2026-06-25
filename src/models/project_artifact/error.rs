use std::borrow::Cow;

/// Possible fails when trying to parse a [`super::ProjectArtifact`] from a string slice.
#[derive(thiserror::Error, Debug)]
pub enum ProjectArtifactParseError<'a> {
    /// When a single name is expected but a compound one was passed.
    #[error("the project artifact carries a compound name")]
    CompoundName,

    /// When an empty/whitespace string is passed as artifact name.
    #[error("the project artifact was expected but not provided")]
    NoNameProvided,

    /// When the given artifact name doesn't follows the [`super::PROJECT_ARTIFACT_REGEX`] rules.
    #[error("the project artifact contains an invalid pattern {}", .0)]
    InvalidPattern(&'a str),

    /// When the artifact name result in a reserved word.
    ///
    /// This variant takes a [`Cow`] since it's type is used to check if the artifact name was
    /// fixed.
    #[error(
        "the {} a reserved name ({})",
        match .0 {
            Cow::Owned(_) => "fixed project artifact results in",
            _ => "project artifact is",
        },
        .0
    )]
    Reserved(Cow<'a, str>),

    /// When the name is too short (less than [`super::PROJECT_ARTIFACT_MINIMUM_LENGTH`] chars
    /// length).
    #[error("the project artifact is too short ({})", .0)]
    ShortName(Cow<'a, str>),

    /// When the name is too long (greater than [`super::PROJECT_ARTIFACT_MINIMUM_LENGTH`] chars
    /// length).
    #[error("the project artifact is too long ({})", .0)]
    LongName(Cow<'a, str>),
}

#[cfg(test)]
mod tests {
    use super::{super::ProjectArtifact, ProjectArtifactParseError};

    #[test]
    fn err_testing() {
        type E = ProjectArtifactParseError<'static>;
        type TestType = (&'static str, fn(E) -> bool);
        let test_sequence: [TestType; 9] = [
            ("a b", |x: E| matches!(x, E::CompoundName)),
            ("abc def", |x: E| matches!(x, E::CompoundName)),
            ("", |x: E| matches!(x, E::NoNameProvided)),
            ("  ", |x: E| matches!(x, E::NoNameProvided)),
            ("in.valid", |x: E| matches!(x, E::InvalidPattern(_))),
            ("0numberstart", |x: E| {
                matches!(x, E::InvalidPattern(_))
            }),
            ("-startswithdash", |x: E| matches!(x, E::InvalidPattern(_))),
            ("endswithdash-", |x: E| matches!(x, E::InvalidPattern(_))),
            ("non-letter?", |x: E| matches!(x, E::InvalidPattern(_))),
        ];
        test_sequence.into_iter().for_each(|(input, err_match)| {
            assert!(ProjectArtifact::try_from(input).is_err_and(err_match))
        });
    }
}
