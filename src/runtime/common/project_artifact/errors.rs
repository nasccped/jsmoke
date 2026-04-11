use super::{
    super::reserved_words::ReservedWords, MAXIMUM_ARTIFACT_LENGTH, MINIMUM_ARTIFACT_LENGTH,
};
use crate::utils::{
    Verbose,
    regex::{
        ASCII_ALPHANUMERIC_REGEX, EMPTY_REGEX, ENDS_WITH_REGEX, NEW_REGEX_WITH, STARTS_WITH_REGEX,
        WHITESPACE_REGEX,
    },
    visuals::style::{NumberStyle, SuggestionStyle},
};
use thiserror::Error as ThisError;

/// Suggestion for artifact names.
const ARTIFACT_NAME_SUGGESTION: [&str; 2] = ["my-app", "singleword"];

/// Error when trying to check a [`str`] like type for [`super::ProjectArtifact`].
#[derive(ThisError, Debug, PartialEq)]
pub enum ArtifactError {
    /// When the artifact is an empty string.
    #[error("artifact name can't be empty")]
    Empty,
    /// When the artifact is whitespace string.
    #[error("artifact name can't be whitespace")]
    WhiteSpace,
    /// When the artifact name is tiny.
    #[error("artifact name can't be this tiny ({0})")]
    Tiny(Box<str>),
    /// When the artifact name is long.
    #[error("artifact name can't be this long ({0})")]
    Long(Box<str>),
    /// When the artifact name is obviously unallowed (starts with hyphen, contains accent, ...)
    #[error("the provided artifact isn't recognized as a valid pattern ({0})")]
    Unrecognizable(Box<str>),
    /// When the artifact name is already reserved by Java (like keyword or type name).
    #[error("the provided artifact is reserved by Java itself ({0})")]
    Reserved(&'static str),
}

impl ArtifactError {
    /// Returns [`Some`] variant of [`ArtifactError::Empty`] if it matches for regex rules.
    pub fn empty_if_matches<T: AsRef<str>>(s: T) -> Option<Self> {
        EMPTY_REGEX.is_match(s.as_ref()).then_some(Self::Empty)
    }

    /// Returns [`Some`] variant of [`ArtifactError::WhiteSpace`] if it matches for regex rules.
    pub fn whitespace_if_matches<T: AsRef<str>>(s: T) -> Option<Self> {
        WHITESPACE_REGEX
            .is_match(s.as_ref())
            .then_some(Self::WhiteSpace)
    }

    /// Returns [`Some`] variant of [`ArtifactError::Tiny`] if it matches for regex rules.
    pub fn tiny_if_matches<T: AsRef<str>>(s: T) -> Option<Self> {
        let s = s.as_ref();
        (ASCII_ALPHANUMERIC_REGEX.find_iter(s).count() < MINIMUM_ARTIFACT_LENGTH)
            .then(|| Self::Tiny(s.into()))
    }

    /// Returns [`Some`] variant of [`ArtifactError::Long`] if it matches for regex rules.
    pub fn long_if_matches<T: AsRef<str>>(s: T) -> Option<Self> {
        let s = s.as_ref();
        (ASCII_ALPHANUMERIC_REGEX.find_iter(s).count() > MAXIMUM_ARTIFACT_LENGTH)
            .then(|| Self::Long(s.into()))
    }

    /// Returns [`Some`] variant of [`ArtifactError::Unrecognizable`] if it matches for regex
    /// rules.
    pub fn unrecognizable_if_matches<T: AsRef<str>>(s: T) -> Option<Self> {
        let s = s.as_ref();
        let conditions = [
            NEW_REGEX_WITH("[-][-]").is_match(s),
            STARTS_WITH_REGEX("[-]").is_match(s),
            ENDS_WITH_REGEX("[-]").is_match(s),
            STARTS_WITH_REGEX("[0-9]").is_match(s),
            NEW_REGEX_WITH("[-][0-9]").is_match(s),
            s.chars().any(|c| !c.is_ascii_alphanumeric() && c != '-'),
        ];
        conditions
            .into_iter()
            .any(|c| c)
            .then(|| Self::Unrecognizable(s.into()))
    }

    /// Returns [`Some`] variant of [`ArtifactError::Reserved`] if it's a reserved Java word. Note
    /// that this function doesn't works with [`Regex`], but with string comparing actually. So,
    /// the `s` input is expected to be already trimmed + lowercase.
    pub fn reserved_if_true<T: AsRef<str>>(s: T) -> Option<Self> {
        let s = s.as_ref();
        ReservedWords::get_if_contained(s).map(Self::Reserved)
    }
}

impl Verbose for ArtifactError {
    fn print_verbose(&self) {
        if let Self::Tiny(_) = self {
            eprintln!(
                "Consider using an artifact at least {} chars long!",
                MINIMUM_ARTIFACT_LENGTH.number_style()
            );
        } else if let Self::Long(_) = self {
            eprintln!(
                "Consider using an artifact of no more than {} chars long!",
                MAXIMUM_ARTIFACT_LENGTH.number_style()
            );
        } else if let Self::Unrecognizable(_) = self {
            eprintln!("Consider using a simpler artifact name, such as:");
            ARTIFACT_NAME_SUGGESTION
                .iter()
                .for_each(|name| eprintln!("- {}", name.suggestion_style()));
        } else if let Self::Reserved(_) = self {
            eprintln!(
                "Note that {} is an valid artifact but it'll be",
                "pri-vate".suggestion_style()
            );
            eprintln!("reduced to {} anyway.\n", "private".suggestion_style());
            eprintln!("Avoid this by using an artifact that isn't (and can't be");
            eprintln!("converted to) a reserved word!");
        }
    }
}

#[cfg(test)]
mod test {
    use super::{ArtifactError as Error, MAXIMUM_ARTIFACT_LENGTH};

    const WHITESPACES: [&str; 4] = [" ", "\t", "\n  \n", "        "];
    const TINIES: [&str; 3] = ["abc", "d", "e-f"];
    const UNRECOGNIZABLES: [&str; 6] = [
        "-startswithdash",
        "endswithdash-",
        "contains--double--dash",
        "0startswithnumber",
        "wordstarts-0withnumber",
        "with-áccênt",
    ];
    const RESERVEDS: [&str; 5] = ["package", "java", "public", "null", "integer"];

    #[test]
    fn empty() {
        let e = Error::empty_if_matches("")
            .expect("`Error::empty_if_matches(\"\")` was expected to return `Some`!");
        assert_eq!(e, Error::Empty);
    }

    #[test]
    fn whitespace() {
        WHITESPACES.into_iter().for_each(|name| {
            let e = Error::whitespace_if_matches(name)
                .unwrap_or_else(|| panic!("\"{}\" name was expected to return `Some`!", name,));
            assert_eq!(e, Error::WhiteSpace);
        });
    }

    #[test]
    fn tiny() {
        TINIES.into_iter().for_each(|name| {
            let e = Error::tiny_if_matches(name)
                .unwrap_or_else(|| panic!("\"{}\" name was expected to return `Some`!", name));
            assert!(
                matches!(e, Error::Tiny(_)),
                "\"{}\" name was expected to be `ArtifactError::Tiny(_)`, but `ArtifactError::{:?}` was returned!",
                name,
                e
            );
        });
    }

    #[test]
    fn long() {
        let name = "a".repeat(MAXIMUM_ARTIFACT_LENGTH + 1);
        let e = Error::long_if_matches(name.as_str()).unwrap_or_else(|| {
            panic!("\"{}\" name was expected to return `Some`!", name.as_str());
        });
        assert!(
            matches!(e, Error::Long(_)),
            "\"{}\" name was expected to be `ArtifactError::Long(_)`, but `ArtifactError::{:?}` was returned!",
            name,
            e
        );
    }

    #[test]
    fn unrecognizable() {
        UNRECOGNIZABLES.into_iter().for_each(|name| {
            let e = Error::unrecognizable_if_matches(name).unwrap_or_else(|| {
            panic!("\"{}\" name was expected to return `Some`!", name);
        });
            assert!(
                matches!(e, Error::Unrecognizable(_)),
                "\"{}\" name was expected to be `ArtifactError::Unrecognizable(_)`, but `ArtifactError::{:?}` was returned!",
                name,
                e
            );
        })
    }

    #[test]
    fn reserved() {
        RESERVEDS.into_iter().for_each(|name| {
            let e = Error::reserved_if_true(name).unwrap_or_else(|| {
                panic!("\"{}\" name was expected to return `Some`!", name);
            });
            assert!(
                matches!(e, Error::Reserved(_)),
                "\"{}\" name was expected to be `ArtifactError::Reserved(_)`, but `ArtifactError::{:?}` was returned!",
                name,
                e
            );
        });
    }
}
