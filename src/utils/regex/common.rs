use crate::utils::SurelyUnwrap;
use regex::Regex;
use std::sync::LazyLock;

/// Regex for empty strings detection.
pub static EMPTY_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^$"#).surely_unwrap());

/// Regex for whitespace strings detection.
pub static WHITESPACE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^[\s]+$"#).surely_unwrap());

/// Regex for ascii char detection.
pub static ASCII_CHAR_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[a-zA-Z]"#).surely_unwrap());
