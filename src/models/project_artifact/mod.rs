mod error;

use crate::utils::Reserveds;
pub use error::ProjectArtifactParseError;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

/// The [`Regex`] used for valid [`ProjectArtifact`] matching.
static PROJECT_ARTIFACT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^[a-z|A-Z][\w]*(?:-[a-z|A-Z][\w]+)*$"#).unwrap());

/// The minimum str length for a [`ProjectArtifact`] item (counts after-fix chars only).
const PROJECT_ARTIFACT_MINIMUM_LENGTH: usize = 4;

/// The maximum str length for a [`ProjectArtifact`] item (counts after-fix chars only).
const PROJECT_ARTIFACT_MAXIMUM_LENGTH: usize = 30;

/// Project artifact wrapper.
#[derive(Debug)]
pub enum ProjectArtifact<'a> {
    /// When the provided string is an already valid artifact (for _'code version'_).
    Valid(&'a str),

    /// When the provided string must be fixed to a _'code version'_:
    /// - `my-art` to `myart`
    /// - `UpperCased` to `uppercased`
    /// - etc
    Fixed {
        /// Original string.
        original: &'a str,

        /// Fixed string.
        fixed: String,
    },
}

impl<'a> TryFrom<&'a str> for ProjectArtifact<'a> {
    type Error = ProjectArtifactParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let s = value.trim();
        match (s, PROJECT_ARTIFACT_REGEX.is_match(s)) {
            // if empty string
            ("", _) => Err(ProjectArtifactParseError::NoNameProvided),

            // if compound name
            (s, _) if s.chars().any(|c| c.is_ascii_whitespace()) => {
                Err(ProjectArtifactParseError::CompoundName)
            }

            // if not a match
            (_, false) => Err(ProjectArtifactParseError::InvalidPattern(s)),

            // other
            (s, true) => {
                if Self::str_must_be_fixed(s) {
                    let fixed: Cow<'a, str> = Cow::Owned(Self::fix_str(s));
                    let clone: String = fixed.clone().into_owned();
                    match Self::potential_error(fixed) {
                        Some(err) => Err(err),
                        None => Ok(Self::Fixed {
                            original: s,
                            fixed: clone,
                        }),
                    }
                } else {
                    match Self::potential_error(Cow::Borrowed(s)) {
                        Some(err) => Err(err),
                        None => Ok(Self::Valid(s)),
                    }
                }
            }
        }
    }
}

impl<'a> ProjectArtifact<'a> {
    /// If the string slice value must be fixed to a _'valid code'_ format.
    fn str_must_be_fixed(value: &'a str) -> bool {
        value.chars().any(|c| c.is_ascii_uppercase() || c == '-')
    }

    /// Takes a string slice and fix it to a _'valid code'_ format.
    fn fix_str(value: &'a str) -> String {
        let mut s = String::new();
        for c in value.chars() {
            match c {
                '-' => {}
                x if x.is_ascii_uppercase() => s.push(x.to_ascii_lowercase()),
                x => s.push(x),
            }
        }
        s
    }

    /// Takes a [`Cow`] over string slice and checks if it's a potential error (must be
    /// called before returning the [`ProjectArtifact`] on [`ProjectArtifact::try_from`] function
    /// block).
    ///
    /// There are three possible error situations:
    /// 1. the provided string is a reserved word (checked through [`Reserveds::is_reserved`]
    ///    function)
    /// 2. the provided string is less than the [`PROJECT_ARTIFACT_MINIMUM_LENGTH`]
    /// 3. the provided string is greater than the [`PROJECT_ARTIFACT_MAXIMUM_LENGTH`]
    ///
    /// Any of cases above returns a [`Some`] variant of the appropriate
    /// [`ProjectArtifactParseError`], otherwise, [`None`] is returned.
    ///
    /// **Note:**
    /// 1. this function takes a [`Cow`] type since it's variant is used to define if the artifact
    ///    input was fixed or not (field required by the [`ProjectArtifactParseError::Reserved`]
    ///    variant)
    /// 2. this function doesn't detects previous checks done at [`ProjectArtifact::try_from`]
    ///    function such as empty string, not matching string, etc...
    fn potential_error(value: Cow<'a, str>) -> Option<ProjectArtifactParseError<'a>> {
        match value.as_ref() {
            // reserved
            s if Reserveds::is_reserved(s) => Some(ProjectArtifactParseError::Reserved(value)),
            // minimum length
            s if s.len() < PROJECT_ARTIFACT_MINIMUM_LENGTH => {
                Some(ProjectArtifactParseError::ShortName(value))
            }
            // maximum length
            s if s.len() > PROJECT_ARTIFACT_MAXIMUM_LENGTH => {
                Some(ProjectArtifactParseError::LongName(value))
            }
            // no errors
            _ => None,
        }
    }
}

impl<'a> ProjectArtifact<'a> {
    /// Returns the inner artifact name as fixed string (code valid).
    fn fixed_str(&self) -> &str {
        match self {
            Self::Valid(s) => s,
            Self::Fixed { fixed, .. } => fixed,
        }
    }

    /// Returns the inner artifact name as it original string (as passed).
    fn original_str(&self) -> &str {
        match self {
            Self::Valid(s) => s,
            Self::Fixed { original, .. } => original,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProjectArtifact;
    use std::borrow::Cow;

    /// Test specific for [`ProjectArtifact`].
    struct PATest<'a>(ProjectArtifact<'a>);

    impl<'a> PATest<'a> {
        /// Creates a new [`PATest`] by the given string slice.
        fn new(artifact: &'a str) -> Self {
            Self(ProjectArtifact::try_from(artifact).unwrap_or_else(|err| {
                panic!("`Ok` expected but '{}' input returned {:?}", artifact, err)
            }))
        }

        /// Asserts if the [`ProjectArtifact::fixed_str`] returns the same value as the `expected`
        /// param.
        fn assert_fixed(&self, expected: &str) -> &Self {
            assert_eq!(self.0.fixed_str(), expected);
            self
        }

        /// Asserts if the [`ProjectArtifact::original_str`] returns the same value as the `expected`
        /// param.
        fn assert_original(&self, expected: &str) -> &Self {
            assert_eq!(self.0.original_str(), expected);
            self
        }
    }

    #[test]
    fn expected_result() {
        PATest::new("normalname")
            .assert_original("normalname")
            .assert_fixed("normalname");
        PATest::new("my-project")
            .assert_original("my-project")
            .assert_fixed("myproject");
        PATest::new("MyProject")
            .assert_original("MyProject")
            .assert_fixed("myproject");
    }

    #[test]
    fn potential_error() {
        [
            ("java", true),
            ("Integer", true),
            ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", true),
            ("aa", true),
            ("other", false),
        ]
        .into_iter()
        .for_each(|(art, expecting_err)| {
            assert!(ProjectArtifact::potential_error(Cow::Borrowed(art)).is_some() == expecting_err)
        });
    }
}
