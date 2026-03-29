mod error;

use crate::utils::{OptionallyFrom, SurelyUnwrap};
pub use error::Error;
use regex::Regex;

/// The project name regex pattern.
const PROJECT_NAME_PATTERN: &str = r#"[A-Z][a-z|0-9|A-Z]*"#;

/// A struct that wraps the project name data type.
#[derive(Eq, PartialEq, Debug)]
pub struct ProjectName<'a>(&'a str);

impl<'a> ProjectName<'a> {
    /// Private [`From`] trait implementing.
    ///
    /// This function must be used only within this module, since all external construction should
    /// be built under [`TryFrom`] trait.
    fn from(value: &'a str) -> Self {
        let re = Regex::new(PROJECT_NAME_PATTERN).surely_unwrap();
        Self(re.find(value).surely_unwrap().as_str())
    }
}

impl<'a> TryFrom<Option<&'a str>> for ProjectName<'a> {
    type Error = Error<'a>;
    fn try_from(value: Option<&'a str>) -> Result<Self, Self::Error> {
        let value = value.ok_or(Error::NoProjectName)?;
        if let Some(e) = Error::optionally_from(value) {
            Err(e)
        } else {
            Ok(ProjectName::from(value))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn panick_it() {
        ProjectName::from("  ");
    }

    #[test]
    fn dont_panick_it() {
        let pairs = [
            ("    SomeName ", ProjectName("SomeName")),
            ("Other", ProjectName("Other")),
            ("\tNumbers55Too", ProjectName("Numbers55Too")),
        ];
        for (input, expected) in pairs.into_iter() {
            assert_eq!(ProjectName::try_from(Some(input)), Ok(expected));
        }
    }
}
