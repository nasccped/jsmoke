use crate::utils::{SurelyUnwrap, TrimAndBox};
use regex::Regex;
use std::{cmp::Ordering, sync::LazyLock};
use thiserror::Error;

/// Examples of [`VersionLiteral`] inputs.
const VERSION_LITERAL_INPUT_EXAMPLES: [&str; 2] = ["10", "13.2"];

/// Same as [`VERSION_LITERAL_REGEX_STR`] but with grouped fields.
static GROUPED_VERSION_LITERAL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\s*(?<major>\d+)(?:\.(?<minor>\d+))?(?:\.(?<patch>\d+))?\s*$"#).surely_unwrap()
});

/// Representation of a version, with major, optional minor and optional patch fields.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VersionLiteral {
    major: u16,
    minor: Option<u16>,
    patch: Option<u16>,
}

impl VersionLiteral {
    /// Returns examples of valid [`VersionLiteral`] inputs.
    pub fn get_examples() -> Vec<&'static str> {
        VERSION_LITERAL_INPUT_EXAMPLES.into_iter().collect()
    }

    /// Returns if all fields are (or represents - such as [`None`]) zero.
    pub fn is_all_zero(&self) -> bool {
        [
            self.major,
            self.minor.unwrap_or_default(),
            self.patch.unwrap_or_default(),
        ]
        .iter()
        .all(|x| *x == 0)
    }
}

impl PartialOrd for VersionLiteral {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VersionLiteral {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            self.major,
            self.minor.unwrap_or_default(),
            self.patch.unwrap_or_default(),
        )
            .cmp(&(
                other.major,
                other.minor.unwrap_or_default(),
                other.patch.unwrap_or_default(),
            ))
    }
}

// NOTE: allow clippy macro, this trait gonna be used later...
#[allow(clippy::to_string_trait_impl)]
impl ToString for VersionLiteral {
    fn to_string(&self) -> String {
        let mut buf = self.major.to_string();
        [self.minor, self.patch].iter().for_each(|val| {
            if let Some(v) = *val {
                buf.push_str(format!(".{}", v).as_str());
            }
        });
        buf
    }
}

/// Error when trying to parse a [`VersionLiteral`] string.
#[derive(Debug, Error, PartialEq)]
pub enum VersionLiteralError {
    /// When passed version isn't recognized as valid pattern.
    #[error("couldn't recognized the version as valid ({0})")]
    NotRecognized(Box<str>),
    /// When version field can't be parsed due to overflow.
    #[error("couldn't parse version due to overflow ({0})")]
    Overflow(Box<str>),
    /// When passed version is zero (`0.0.0`).
    #[error("version can't be zero")]
    IsZero,
}

impl TryFrom<&str> for VersionLiteral {
    type Error = VersionLiteralError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let captures = GROUPED_VERSION_LITERAL_REGEX
            .captures(value)
            .ok_or(VersionLiteralError::NotRecognized(value.trim_and_box()))?;
        let get_group = |name: &str| match captures.name(name) {
            None => Ok(None),
            Some(m) => {
                let s = m.as_str();
                s.parse::<u16>()
                    .map(Some)
                    .map_err(|_| VersionLiteralError::Overflow(s.into()))
            }
        };
        let major: u16 = get_group("major")?.surely_unwrap();
        let minor: Option<u16> = get_group("minor")?;
        let patch: Option<u16> = get_group("patch")?;
        let result = Self {
            major,
            minor,
            patch,
        };
        (!result.is_all_zero())
            .then_some(result)
            .ok_or(VersionLiteralError::IsZero)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok_cases() {
        let values = [
            ("1", (1, None, None)),
            ("12", (12, None, None)),
            ("23.1", (23, Some(1), None)),
            ("1.212.32", (1, Some(212), Some(32))),
        ];
        values.into_iter().for_each(|(input, exp)| {
            let version = VersionLiteral::try_from(input)
                .unwrap_or_else(|e| panic!("expecting Ok but got Err({:?})", e));
            assert_eq!(version.major, exp.0);
            assert_eq!(version.minor, exp.1);
            assert_eq!(version.patch, exp.2);
        });
    }

    #[test]
    fn err_cases() {
        let values = ["1.", ".12", "2..31", "1.2 2.32"];
        values.into_iter().for_each(|input| {
            assert!(
                VersionLiteral::try_from(input).is_err(),
                "{} input returned Ok",
                input
            );
        });
    }

    #[test]
    fn comp() {
        let greater = [("2", "1"), ("2.1", "1.0.0"), ("0.1", "0.0.1")];
        greater.into_iter().for_each(|(this, other)| {
            let ver = VersionLiteral::try_from(this).surely_unwrap();
            let other = VersionLiteral::try_from(other).surely_unwrap();
            assert!(
                ver.cmp(&other) == Ordering::Greater,
                "{:?} was expected to be greater than {:?}",
                ver,
                other
            );
        });
        let equals = [("1", "1"), ("1.0", "1")];
        equals.into_iter().for_each(|(this, other)| {
            let ver = VersionLiteral::try_from(this).surely_unwrap();
            let other = VersionLiteral::try_from(other).surely_unwrap();
            assert!(
                ver.cmp(&other) == Ordering::Equal,
                "{:?} was expected to be equals than {:?}",
                ver,
                other
            );
        });
        // NOTE: less assertions are unnecessary since it's the inverse of `greater` assertion.
    }

    #[test]
    fn zero() {
        assert!(VersionLiteral::try_from("0").is_err_and(|e| e == VersionLiteralError::IsZero));
    }
}
