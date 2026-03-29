use crate::utils::OptionallyFrom;
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
pub enum Error<'a> {
    /// When no project name is given.
    #[error("no project name provided")]
    NoProjectName,
    /// When project name is empty string.
    #[error("project name is whitespace only")]
    WhiteSpace,
    /// When project name starts with a number.
    #[error("project name starts with number: {0}")]
    StartsWithNumber(&'a str),
    /// When project name is a compound name.
    #[error("project name is compound: {0}")]
    CompoundName(&'a str),
    /// When an invalid char is passed.
    #[error("invalid char at projet name: {0}")]
    InvalidChar(&'a str),
    /// When project name isn't a CamelCase pattern.
    #[error("project name isn't CamelCase: {0}")]
    NotCamelCase(&'a str),
    /// [`regex::Error`] variant.
    #[error("unexpected error when regexing: {0:?}")]
    Unexpected(RegexError),
}

// constructors shorthand. using this since I can't use the enum-variant generator due to type
// constraints...
impl<'a> Error<'a> {
    /// Create [`Error::NoProjectName`].
    fn no_name(_: &'a str) -> Self {
        Self::NoProjectName
    }
    /// Create [`Error::WhiteSpace`].
    fn white_space(_: &'a str) -> Self {
        Self::WhiteSpace
    }
    /// Create [`Error::StartsWithNumber`].
    fn starts_with_number(value: &'a str) -> Self {
        Self::StartsWithNumber(value)
    }
    /// Create [`Error::CompoundName`].
    fn compound_name(value: &'a str) -> Self {
        Self::CompoundName(value)
    }
    /// Create [`Error::InvalidChar`].
    fn invalid_char(value: &'a str) -> Self {
        Self::InvalidChar(value)
    }
    /// Create [`Error::NotCamelCase`].
    fn not_camel_case(value: &'a str) -> Self {
        Self::NotCamelCase(value)
    }
}

impl<'a> OptionallyFrom<&'a str> for Error<'a> {
    fn optionally_from(value: &'a str) -> Option<Self>
    where
        Self: Sized,
    {
        type PatternMapping<'b> = (&'static str, fn(&'b str) -> Error<'b>);
        let mapping: [PatternMapping<'a>; 6] = [
            (patterns::NO_NAME, Error::no_name),
            (patterns::WHITESPACE, Error::white_space),
            (patterns::INVALID_CHAR, Error::invalid_char),
            (patterns::COMPOUND_NAME, Error::compound_name),
            (patterns::STARTS_WITH_NUMBER, Error::starts_with_number),
            (patterns::NOT_CAMEL_CASE, Error::not_camel_case),
        ];
        for (pat, func) in mapping.into_iter() {
            match Regex::new(pat).map_err(Self::Unexpected) {
                Ok(r) => {
                    if r.is_match(value) {
                        return Some(func(value));
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
    fn str_into_error<'a>(input: &'a str) -> Option<Error<'a>> {
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
