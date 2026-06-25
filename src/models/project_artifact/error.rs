use crate::utils::{
    Verbose,
    styler::{ListStyle, Styler, StylerOutput},
};
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
    #[error("the project artifact contains an invalid pattern ({})", .0)]
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

impl<'a> Verbose for ProjectArtifactParseError<'a> {
    fn get_verbose_message(&self) -> Option<Cow<'_, str>> {
        let message = match self {
            Self::CompoundName | Self::NoNameProvided => format!(
                "Consider passing a simple name such as {}. \n\
                You can also use {} flag to skip artifact setting.",
                "my-app".suggestion_style(),
                "--no-artifact".command_style()
            ),
            Self::InvalidPattern(_) => {
                let mut conditions = ListStyle::new_unordered();
                conditions.set_left_padding(1);
                conditions.push_items([
                    "starts with a letter",
                    "contain dashes/capitalized",
                    "contain numbers",
                ]);
                let styled_out: StylerOutput<String> = conditions.into();
                format!(
                    "A valid artifact pattern can be expressed\n\
                    by the {} regex. In short:\n\
                    {}",
                    super::PROJECT_ARTIFACT_REGEX.as_str().term_style(),
                    styled_out
                )
            }
            Self::Reserved(res) => {
                let styled: StylerOutput<String> = {
                    let mut l = ListStyle::new_ordered();
                    l.set_left_padding(1);
                    l.push_items(["keywords", "common types", "well know names", "etc"]);
                    l.into()
                };
                let mut message = format!(
                    "To avoid {} or management undefined behavior, java\n\
                    environment reserved words aren't allowed. This refers to:\n\
                    {}",
                    "(run/comp)time".term_style(),
                    styled
                );
                if matches!(res, Cow::Owned(_)) {
                    let note = format!(
                        "this check is done after artifact fixing, so\n\
                        the {} can be fixed to {} and turn into a reserved\n\
                        word.",
                        "ja-va".special_style(),
                        "java".special_style()
                    )
                    .note_style();
                    message.push_str(format!("\n\n{}", note).as_str())
                }
                message
            }
            Self::ShortName(_) => format!(
                "The project artifact must be at least {} char long!",
                super::PROJECT_ARTIFACT_MINIMUM_LENGTH.number_style(),
            ),
            Self::LongName(_) => format!(
                "The project artifact must contains less than {} chars!",
                (super::PROJECT_ARTIFACT_MAXIMUM_LENGTH + 1).number_style()
            ),
        };
        Some(Cow::Owned(message))
    }
}

#[cfg(test)]
mod tests {
    use super::{super::ProjectArtifact, ProjectArtifactParseError};

    /// Test purpose struct.
    struct ErrTester(ProjectArtifactParseError<'static>);

    impl ErrTester {
        /// Creates a new [`ErrTester`] with the inner value.
        fn new(value: &'static str) -> Self {
            match ProjectArtifact::try_from(value) {
                Err(err) => Self(err),
                Ok(val) => panic!("`Err` was expected but got {:?}", val),
            }
        }

        /// Asserts if the provided match is true.
        fn assert_match<F>(&self, matching: F)
        where
            F: FnOnce(&ProjectArtifactParseError<'static>) -> bool,
        {
            assert!(matching(&self.0));
        }
    }

    #[test]
    fn err_testing() {
        type E = ProjectArtifactParseError<'static>;
        ErrTester::new("a b").assert_match(|err| matches!(err, E::CompoundName));
        ErrTester::new("abc def").assert_match(|err| matches!(err, E::CompoundName));
        ErrTester::new("").assert_match(|err| matches!(err, E::NoNameProvided));
        ErrTester::new("  ").assert_match(|err| matches!(err, E::NoNameProvided));
        ErrTester::new("in.valid").assert_match(|err| matches!(err, E::InvalidPattern(_)));
        ErrTester::new("0numberstart").assert_match(|err| matches!(err, E::InvalidPattern(_)));
        ErrTester::new("-startswithdash").assert_match(|err| matches!(err, E::InvalidPattern(_)));
        ErrTester::new("endswithdash-").assert_match(|err| matches!(err, E::InvalidPattern(_)));
        ErrTester::new("non-letter?").assert_match(|err| matches!(err, E::InvalidPattern(_)));
    }
}
