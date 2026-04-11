use super::{ArtifactError, MAXIMUM_ARTIFACT_LENGTH, MINIMUM_ARTIFACT_LENGTH};
use crate::utils::regex::NEW_REGEX_WITH;
use regex::Regex;
use std::fmt::Display;
use std::sync::LazyLock;

/// Regex matcher for [`Artifact::Allowed`].
static ALLOWED_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let pat = format!(
        "^[a-z][a-z0-9]{{{},{}}}$",
        MINIMUM_ARTIFACT_LENGTH - 1,
        MAXIMUM_ARTIFACT_LENGTH - 1
    );
    NEW_REGEX_WITH(pat.as_ref())
});

/// Type wrapper for project artifact.
#[derive(Debug, PartialEq)]
pub enum Artifact {
    /// When the passed artifact is already allowed.
    Allowed(Box<str>),
    /// When the passed artifact must be fixed to view (how its printed - with hyphens) and fix
    /// mode (hardcode - no hyphen) versions.
    Fixed {
        /// View designated version of artifact name.
        view: Box<str>,
        /// Code designated version of artifact name.
        fix: Box<str>,
    },
}

impl Artifact {
    /// Returns the [`Artifact`] name as fixed variant.
    pub fn get_fixed(&self) -> &str {
        match self {
            Self::Allowed(a) => a,
            Self::Fixed { fix, .. } => fix,
        }
    }

    /// Returns the [`Artifact`] name as view variant.
    pub fn get_view(&self) -> &str {
        match self {
            Self::Allowed(a) => a,
            Self::Fixed { view, .. } => view,
        }
    }

    /// Creates a [`Artifact::Allowed`] from the given string.
    fn new_allowed<T: AsRef<str>>(s: T) -> Self {
        Self::Allowed(s.as_ref().into())
    }

    /// Creates a [`Artifact::Fixed`] from the given string. Note that this function will replace
    /// all hyphens with empty string ("") for the [`Artifact::Fixed::fix`] field.
    fn new_fixed<T: AsRef<str>>(s: T) -> Self {
        let s = s.as_ref();
        let fix = s.replace("-", "").into_boxed_str();
        let view = s.into();
        Self::Fixed { view, fix }
    }
}

impl Display for Artifact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_view())
    }
}

/// Functions for [`ArtifactError`] empties checking.
const EMPTIES_CHECKING: [for<'a> fn(&'a str) -> Option<ArtifactError>; 2] = [
    |s| ArtifactError::empty_if_matches(s),
    |s| ArtifactError::whitespace_if_matches(s),
];

/// Functions for [`ArtifactError`] otherwise checking.
const OTHERWISE_CHECKING: [for<'a> fn(&'a str) -> Option<ArtifactError>; 3] = [
    |s| ArtifactError::tiny_if_matches(s),
    |s| ArtifactError::long_if_matches(s),
    |s| ArtifactError::unrecognizable_if_matches(s),
];

impl TryFrom<&str> for Artifact {
    type Error = ArtifactError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(err) = ArtifactError::reserved_if_true(value) {
            return Err(err);
        } else if is_allowed(value) {
            return Ok(Self::new_allowed(value));
        }
        let value = EMPTIES_CHECKING
            .iter()
            .try_for_each(|check| match check(value) {
                Some(e) => Err(e),
                None => Ok(()),
            })
            .map(|_| value.trim().replace("_", "-").to_lowercase())?;
        let s = OTHERWISE_CHECKING
            .iter()
            .try_for_each(|check| match check(value.as_str()) {
                Some(e) => Err(e),
                None => Ok(()),
            })
            .map(|_| value.as_str())?;
        let fix = if is_allowed(s) {
            Artifact::new_allowed(s)
        } else {
            Artifact::new_fixed(s)
        };
        if let Some(err) = ArtifactError::reserved_if_true(fix.get_fixed()) {
            Err(err)
        } else {
            Ok(fix)
        }
    }
}

/// Returns if a given string is an acceptable [`Artifact::Allowed`] artifact.
fn is_allowed<T: AsRef<str>>(s: T) -> bool {
    ALLOWED_REGEX.is_match(s.as_ref())
}

#[cfg(test)]
mod test {
    use super::*;

    const ALLOWED_ARTIFACTS: [&str; 5] = [
        "myartifact",
        "  leadingleft",
        "leadright  ",
        "  bothleading  ",
        "withnumber50",
    ];
    const FIXED_EXPECTS: [(&str, &str); 4] = [
        ("my-artifact", "myartifact"),
        ("Camel-Case", "camelcase"),
        ("number01-On-it", "number01onit"),
        ("  leading-Left01", "leadingleft01"),
    ];

    #[test]
    fn allowed() {
        ALLOWED_ARTIFACTS.into_iter().for_each(|art| {
            let res = Artifact::try_from(art);
            assert!(
                matches!(res, Ok(Artifact::Allowed(_))),
                "expecting \"{}\" to result into `Ok(Artifact::Allowed(_))`, but got `{:?}`",
                art,
                res
            );
        })
    }

    #[test]
    fn fixed() {
        FIXED_EXPECTS.into_iter().for_each(|(name, _)| {
            let res = Artifact::try_from(name).expect("this was expected to be Ok");
            assert!(
                matches!(res, Artifact::Fixed { .. }),
                "expecting \"{}\" to result into `Artifact::Fixed(_)`, but got `Artifact::{:?}`",
                res,
                res
            );
        });
    }

    #[test]
    fn expecting() {
        ALLOWED_ARTIFACTS.into_iter().for_each(|name| {
            let res = Artifact::try_from(name).expect("this was expected to be Ok");
            assert_eq!(res.get_fixed(), res.get_view());
        });
        FIXED_EXPECTS.into_iter().for_each(|(name, exp)| {
            let res = Artifact::try_from(name).expect("this was expected to be Ok");
            assert_eq!(res.get_fixed(), exp);
        });
    }

    #[test]
    fn redo() {
        let a = |x: &str| Artifact::try_from(x);
        let empty = || Err(ArtifactError::Empty);
        let white = || Err(ArtifactError::WhiteSpace);
        let tiny = |x: &str| Err::<Artifact, ArtifactError>(ArtifactError::Tiny(x.into()));
        let long = |x: &str| Err::<Artifact, ArtifactError>(ArtifactError::Long(x.into()));
        let unrec =
            |x: &str| Err::<Artifact, ArtifactError>(ArtifactError::Unrecognizable(x.into()));
        let rese = |x: &str| Err::<Artifact, ArtifactError>(ArtifactError::Reserved(x.into()));
        assert_eq!(a(""), empty());
        assert_eq!(a("  "), white());
        assert_eq!(a("\t"), white());
        assert_eq!(a("\n"), white());
        assert_eq!(a("Abc"), tiny("abc"));
        assert_eq!(a("a-bc"), tiny("a-bc"));
        assert_eq!(
            a("a".repeat(MAXIMUM_ARTIFACT_LENGTH + 1).as_str()),
            long("a".repeat(MAXIMUM_ARTIFACT_LENGTH + 1).as_str())
        );
        assert_eq!(a("-project"), unrec("-project"));
        assert_eq!(a("project-"), unrec("project-"),);
        assert_eq!(a("the--project"), unrec("the--project"),);
        assert_eq!(a("00numbers"), unrec("00numbers"));
        assert_eq!(a("late-00number"), unrec("late-00number"));
        assert_eq!(a("áccênt"), unrec("áccênt"));
        assert_eq!(a("emoji-😁"), unrec("emoji-😁"));
        assert_eq!(a("punctuation!"), unrec("punctuation!"));
        assert_eq!(a("  pri-vate  "), rese("private"));
        assert_eq!(a("java"), rese("java"));
        assert_eq!(a("c-l-a-s-s  "), rese("class"));
    }
}
