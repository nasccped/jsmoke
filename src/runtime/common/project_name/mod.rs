//! # Project Name module
//!
//! Provides project name parsing + it's utilities.
mod error;

use crate::utils::{OptionallyFrom, SurelyUnwrap};
pub use error::Error;
use regex::Regex;
use std::fmt::Display;

/// The project name regex pattern.
const PROJECT_NAME_PATTERN: &str = r#"[A-Z][a-z|0-9|A-Z]*"#;

/// A struct that wraps the project name data type.
#[derive(Eq, PartialEq, Debug)]
pub struct ProjectName(Box<str>);

impl ProjectName {
    /// Private [`From`] trait implementing.
    ///
    /// This function must be used only within this module, since all external construction should
    /// be built under [`TryFrom`] trait.
    fn from<T: AsRef<str>>(value: T) -> Self {
        let re = Regex::new(PROJECT_NAME_PATTERN).surely_unwrap();
        Self(re.find(value.as_ref()).surely_unwrap().as_str().into())
    }
}

impl<T: AsRef<str> + Clone> TryFrom<Option<T>> for ProjectName {
    type Error = Error;
    fn try_from(value: Option<T>) -> Result<Self, Self::Error> {
        let value = value.ok_or(Error::NoProjectName)?;
        if let Some(e) = Error::optionally_from(value.clone()) {
            Err(e)
        } else {
            Ok(ProjectName::from(value))
        }
    }
}

impl Display for ProjectName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Fast project name function.
    fn fpn<T: Into<Box<str>>>(value: T) -> ProjectName {
        ProjectName(value.into())
    }

    #[test]
    #[should_panic]
    fn panick_it() {
        ProjectName::from("  ");
    }

    #[test]
    fn dont_panick_it() {
        let pairs = [
            ("    SomeName ", fpn("SomeName")),
            ("Other", fpn("Other")),
            ("\tNumbers55Too", fpn("Numbers55Too")),
        ];
        for (input, expected) in pairs.into_iter() {
            assert_eq!(ProjectName::try_from(Some(input)), Ok(expected));
        }
    }
}
