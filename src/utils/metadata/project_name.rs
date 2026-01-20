use crate::utils::{
    printer::Printer,
    verbose::Verbose,
    visuals::list::{Bullet, List},
};
use colored::Colorize;
use regex::Regex;
use thiserror::Error;

/// Fast way to generate a regex from a surely safe pattern (avoid `Regex::new(...).unwrap()`
/// repetition).
fn fast_regex(pattern: &str) -> Regex {
    Regex::new(pattern)
        .unwrap_or_else(|e| panic!("this was expected to be `Ok(_)` but returned: {:?}", e))
}

/// Expected string pattern for the [`ProjectName`].
const PROJECT_NAME_PATTERN: &str = r#"[A-Z][a-z|0-9|A-Z]*"#;
/// String pattern for empty project name.
const ERR_IS_EMPTY_PATTERN: &str = r#"^\s*$"#;
/// String pattern for project names that starts with a number.
const ERR_STARTS_WITH_NUMBER_PATTERN: &str = r#"^\s*[0-9]"#;
/// String pattern for project names that starts with a number.
const ERR_STARTS_WITH_LOWERCASE_PATTERN: &str = r#"^\s*[a-z]"#;
/// String pattern for not allowed chars.
const ERR_NOT_ALLOWED_CHAR_PATTERN: &str = r#"[^a-z|A-Z|\s|0-9]"#;
/// String pattern for compound project names.
const ERR_COMPOUND_NAME_PATTERN: &str = r#"\S+\s+\S+"#;

/// Project name wrapper. Provides shortcuts for project name data and actions.
pub struct ProjectName(String);

/// When [`ProjectName`] building fails due to an invalid pattern.
#[derive(Debug, Error)]
pub enum ProjectNameError {
    /// When passing an empty string as project name.
    #[error("project name can't be empty")]
    IsEmpty,
    /// When passing a string that starts with a number.
    #[error("project name can't start with a number ({0})")]
    StartsWithNumber(String),
    /// When passing a string that starts with a lowercase character.
    #[error("project name can't start with a number ({0})")]
    StartsWithLowercase(String),
    /// When passing more than 1 word as project name.
    #[error("project name can't be compound ({0})")]
    CompoundName(String),
    /// When passing a string with a not allowed char.
    #[error("project name contains not allowed chars ({0})")]
    NotAllowedChar(String),
}

/// `class-like` valid names that are mentioned at [`ProjectName::print_verbose`] function.
const CLASS_LIKE_VALID_NAMES: [&str; 3] = ["MyClass", "Main", "App"];

impl Verbose for ProjectNameError {
    type LocalPrinter<'a> = &'a mut Printer;
    fn print_verbose(&self, printer: &'_ mut Printer) {
        printer.use_stderr();
        printer.println("Use a class-like valid name:");
        let color = |s: &str| s.bright_blue();
        let mut list: List<&str, &str> = List::new();
        list.with_items(CLASS_LIKE_VALID_NAMES)
            .with_bullet(Bullet::Symbol("*"))
            .with_bullet_color(color);
        for item in list.into_iter() {
            printer.println(item);
        }
    }
}

impl ProjectNameError {
    fn new(sample: &str) -> Option<Self> {
        let rules = [
            |s: &str| {
                fast_regex(ERR_COMPOUND_NAME_PATTERN)
                    .find(s)
                    .map(|_| Self::CompoundName(s.into()))
            },
            |s: &str| {
                fast_regex(ERR_IS_EMPTY_PATTERN)
                    .find(s)
                    .map(|_| Self::IsEmpty)
            },
            |s: &str| {
                fast_regex(ERR_STARTS_WITH_NUMBER_PATTERN)
                    .find(s)
                    .map(|_| Self::StartsWithNumber(s.into()))
            },
            |s: &str| {
                fast_regex(ERR_NOT_ALLOWED_CHAR_PATTERN)
                    .find(s)
                    .map(|_| Self::NotAllowedChar(s.into()))
            },
            |s: &str| {
                fast_regex(ERR_STARTS_WITH_LOWERCASE_PATTERN)
                    .find(s)
                    .map(|_| Self::StartsWithLowercase(s.into()))
            },
        ];
        for r in rules {
            match r(sample) {
                None => {}
                x => return x,
            }
        }
        None
    }
}

impl ProjectName {
    fn try_new<S: AsRef<str>>(value: S) -> Result<Self, ProjectNameError> {
        let re = fast_regex(PROJECT_NAME_PATTERN);
        let s = value.as_ref();
        match ProjectNameError::new(s) {
            Some(err) => Err(err),
            _ => {
                let name = re
                    .find(s)
                    .unwrap_or_else(|| panic!("`{}` was expected to be `Some` since err variants was already checked!", s))
                    .as_str()
                    .into();
                Ok(Self(name))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_names() {
        let valids = [
            "   Some  ",
            " Valid    ",
            " Name     ",
            "I",
            " WithNumber50  ",
        ];
        for v in valids {
            assert!(ProjectName::try_new(v).is_ok_and(|pn| pn.0.eq(v.trim())))
        }
    }

    #[test]
    fn invalid_names() {
        assert!(
            ProjectNameError::new("name")
                .is_some_and(|e| matches!(e, ProjectNameError::StartsWithLowercase(_)))
        );
        assert!(
            ProjectNameError::new("5Five  ")
                .is_some_and(|e| matches!(e, ProjectNameError::StartsWithNumber(_)))
        );
        assert!(
            ProjectNameError::new("   \t  \n ")
                .is_some_and(|e| matches!(e, ProjectNameError::IsEmpty))
        );
        assert!(
            ProjectNameError::new("Big Name")
                .is_some_and(|e| matches!(e, ProjectNameError::CompoundName(_)))
        );
        assert!(
            ProjectNameError::new("GoodChar^.^")
                .is_some_and(|e| matches!(e, ProjectNameError::NotAllowedChar(_)))
        );
    }
}
