use crate::utils::SurelyUnwrap;
use regex::Regex;
use std::sync::LazyLock;

/// Regex for empty strings detection.
pub static EMPTY_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^$"#).surely_unwrap());

/// Regex for whitespace strings detection.
pub static WHITESPACE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^[\s]+$"#).surely_unwrap());

/// Regex for ascii alphabetic detection. Works both for lowercase and uppercase.
pub static ASCII_ALPHABETIC_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[a-zA-Z]"#).surely_unwrap());

/// Regex for ascii alphanumeric detection. Works both for lowercase and uppercase.
pub static ASCII_ALPHANUMERIC_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[a-zA-Z0-9]"#).surely_unwrap());

/// Function to `startswith` detection. Note that this variable returns a function that results
/// into a [`Regex`] (unwrap at initialization), so, it can [`panic`] if invalid regex syntax is
/// used.
///
/// Regex's begin symbol (`^`) isn't required since it's pushed at initialization.
pub static STARTS_WITH_REGEX: LazyLock<fn(&str) -> Regex> = LazyLock::new(|| starts_with_regex);

/// Function to `endswith` detection. Note that this variable returns a function that results
/// into a [`Regex`] (unwrap at initialization), so, it can [`panic`] if invalid regex syntax is
/// used.
///
/// Regex's begin symbol (`^`) isn't required since it's pushed at initialization.
pub static ENDS_WITH_REGEX: LazyLock<fn(&str) -> Regex> = LazyLock::new(|| ends_with_regex);

/// Creates a new [`Regex`] with the provided regex pattern. Can [`panic`] since the
/// [`SurelyUnwrap::surely_unwrap`] is used before returning.
pub static NEW_REGEX_WITH: LazyLock<fn(&str) -> Regex> = LazyLock::new(|| new_regex_with);

/// Private function wrapper for [`STARTS_WITH_REGEX`] lock.
///
/// It takes a str slice as pattern and produces a [`Regex`] over it. Can [`panic`] since it uses
/// [`SurelyUnwrap::surely_unwrap`] before returning.
fn starts_with_regex(pattern: &str) -> Regex {
    let mut s = String::from("^");
    s.push_str(pattern);
    Regex::new(s.as_str()).surely_unwrap()
}

/// Private function wrapper for [`ENDS_WITH_REGEX`] lock.
///
/// It takes a str slice as pattern and produces a [`Regex`] over it. Can [`panic`] since it uses
/// [`SurelyUnwrap::surely_unwrap`] before returning.
fn ends_with_regex(pattern: &str) -> Regex {
    let mut s = String::from(pattern);
    s.push('$');
    Regex::new(s.as_str()).surely_unwrap()
}

/// Private function wrapper for [`NEW_REGEX_WITH`] lock.
///
/// It takes a str slice as pattern and produces a [`Regex`] over it. Can [`panic`] since it uses
/// [`SurelyUnwrap::surely_unwrap`] before returning.
fn new_regex_with(pattern: &str) -> Regex {
    Regex::new(pattern).surely_unwrap()
}
