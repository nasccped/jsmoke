use crate::utils::{OptionallyFrom, Verbose};
use regex::{Error as RegexError, Regex};
use thiserror::Error as ThisError;

mod patterns {
    //! Private module to store regex patterns for **possible** errors.

    /// When no name is given (empty string).
    pub const NO_NAME: &str = r#"^$"#;
    /// When name is only whitespace.
    pub const WHITESPACE: &str = r#"^[\s]+$"#;
    /// When a non-valid character is used (accent and/or symbols).
    pub const INVALID_CHAR: &str = r#"[^a-z|A-Z|0-9|\s]"#;
    /// When fullname is a compound name.
    pub const COMPOUND_NAME: &str = r#"\S[\s]+\S"#;
    /// When first word of string starts with a number.
    pub const STARTS_WITH_NUMBER: &str = r#"^[\s]*[0-9]"#;
    /// When the name doesn't follow the CamelCase pattern.
    pub const NOT_CAMEL_CASE: &str = r#"^[\s]*[a-z]"#;
}

/// Error type when trying to parse the [`super::ProjectName`].
#[derive(ThisError, Debug, PartialEq)]
pub enum Error {
    /// When no project name is given.
    #[error("no project name provided/found")]
    NoProjectName,
    /// When project name is empty string.
    #[error("project name is whitespace only")]
    WhiteSpace,
    /// When project name starts with a number.
    #[error("project name starts with number: {0}")]
    StartsWithNumber(Box<str>),
    /// When project name is a compound name.
    #[error("project name is compound: {0}")]
    CompoundName(Box<str>),
    /// When an invalid char is passed.
    #[error("invalid char at projet name: {0}")]
    InvalidChar(Box<str>),
    /// When project name isn't a CamelCase pattern.
    #[error("project name isn't CamelCase: {0}")]
    NotCamelCase(Box<str>),
    /// [`regex::Error`] variant.
    #[error("unexpected error when regexing")]
    Unexpected(RegexError),
}

impl Verbose for Error {
    fn print_verbose(&self) {
        match self {
            Self::WhiteSpace | Self::NoProjectName => {}
            Self::StartsWithNumber(_) => eprintln!("That's not allowed by the Java syntax rule."),
            Self::InvalidChar(_) => eprintln!(
                "Java compiler expects only numerics (0-9) and alpha. chars as
class names."
            ),
            Self::CompoundName(_) | Self::NotCamelCase(_) => {
                eprintln!("This isn't allowed. Use CamelCase pattern instead!")
            }
            Self::Unexpected(err) => eprintln!(
                "That's an unexpected behavior. Error probably occured at
regex libray (or I wrote a bad code, as well). Look the
debug display: {:?}",
                err
            ),
        }
    }
}

// constructors shorthand. using this since I can't use the enum-variant generator due to type
// constraints...
impl Error {
    /// Create [`Error::NoProjectName`].
    fn no_name<T: Into<Box<str>>>(_: T) -> Self {
        Self::NoProjectName
    }
    /// Create [`Error::WhiteSpace`].
    fn white_space<T: Into<Box<str>>>(_: T) -> Self {
        Self::WhiteSpace
    }
    /// Create [`Error::StartsWithNumber`].
    fn starts_with_number<T: Into<Box<str>>>(value: T) -> Self {
        Self::StartsWithNumber(value.into())
    }
    /// Create [`Error::CompoundName`].
    fn compound_name<T: Into<Box<str>>>(value: T) -> Self {
        Self::CompoundName(value.into())
    }
    /// Create [`Error::InvalidChar`].
    fn invalid_char<T: Into<Box<str>>>(value: T) -> Self {
        Self::InvalidChar(value.into())
    }
    /// Create [`Error::NotCamelCase`].
    fn not_camel_case<T: Into<Box<str>>>(value: T) -> Self {
        Self::NotCamelCase(value.into())
    }
}

impl<T: AsRef<str>> OptionallyFrom<T> for Error {
    fn optionally_from(value: T) -> Option<Self>
    where
        Self: Sized,
    {
        type PatternMapping<'a> = (&'static str, fn(&'a str) -> Error);
        let mapping: [PatternMapping; 6] = [
            (patterns::NO_NAME, Error::no_name),
            (patterns::WHITESPACE, Error::white_space),
            (patterns::INVALID_CHAR, Error::invalid_char),
            (patterns::COMPOUND_NAME, Error::compound_name),
            (patterns::STARTS_WITH_NUMBER, Error::starts_with_number),
            (patterns::NOT_CAMEL_CASE, Error::not_camel_case),
        ];
        let valref = value.as_ref();
        for (pat, func) in mapping.into_iter() {
            match Regex::new(pat).map_err(Self::Unexpected) {
                Ok(r) => {
                    if r.is_match(valref) {
                        return Some(func(valref));
                    }
                }
                Err(e) => return Some(e),
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Converts a [`str`] slice into an [`Error`].
    fn str_into_error(input: &str) -> Option<Error> {
        Error::optionally_from(input)
    }

    #[test]
    fn test_variants() {
        assert_eq!(str_into_error(""), Some(Error::NoProjectName));
        assert!(matches!(str_into_error("    "), Some(Error::WhiteSpace)));
        assert!(matches!(str_into_error("??"), Some(Error::InvalidChar(_))));
        assert!(matches!(
            str_into_error("Some Name"),
            Some(Error::CompoundName(_))
        ));
        assert!(matches!(
            str_into_error("5k4t3"),
            Some(Error::StartsWithNumber(_))
        ));
        assert!(matches!(
            str_into_error("notCamelCase"),
            Some(Error::NotCamelCase(_))
        ));
    }
}
