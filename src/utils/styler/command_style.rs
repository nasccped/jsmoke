#![allow(clippy::borrow_interior_mutable_const)]
use crate::utils::StringUtils;
use colored::Colorize;
use regex::Regex;
use std::{fmt::Display, ops::Deref, sync::LazyLock};

/// Style behavior for command-like prompting, just like:
/// ```sh
/// cd some/dir
/// ```
/// And others...
pub struct CommandStyle<'a>(Vec<Kind<'a>>);

/// Different kind of [`CommandStyle`] pieces.
#[derive(Debug, PartialEq, Clone)]
enum Kind<'a> {
    /// Any piece that starts the prompt.
    Command(&'a str),

    /// Any piece that follows a command and starts with a dash (`-`).
    Flag(&'a str),

    /// Any piece that is around quoted (`'` or `"`).
    String(&'a str),

    /// Means the `||` or `|` operator.
    Or(&'a str),

    /// Means the `&&` or `&` operator.
    And(&'a str),

    /// Any piece that follows a command or flag and isn't a string (quoted).
    Value(&'a str),
}

/// A simple regex build entirely for [`Kind`] matching.
static SIMPLE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let string_pattern = r#"(?:'[^']*'|"[^"]*")"#;
    let flag_pattern = r"(?:\-+\S+)";
    let cmd_pattern = r"(?:\S+)";
    let and_pattern = r"(?:\&+)";
    let or_pattern = r"(?:\|+)";
    let res = format!(
        "{}|{}|{}|{}|{}",
        string_pattern, flag_pattern, cmd_pattern, and_pattern, or_pattern
    );
    Regex::new(res.as_str()).unwrap()
});

impl<'a> From<&'a str> for CommandStyle<'a> {
    fn from(value: &'a str) -> Self {
        let mut v = Vec::new();
        let expecting_cmd = |v: &[Kind<'_>]| {
            v.last()
                .map(|kind| matches!(kind, Kind::Or(_) | Kind::And(_)))
                .unwrap_or(true)
        };
        for m in SIMPLE_REGEX.captures_iter(value) {
            let kind = match m.get_match().as_str() {
                x if x.is_quoted() => Kind::String(x),
                x if expecting_cmd(&v) => Kind::Command(x),
                x if x.starts_with("-") => Kind::Flag(x),
                x if x.is_repetition_of("|") => Kind::Or(x),
                x if x.is_repetition_of("&") => Kind::And(x),
                x => Kind::Value(x),
            };
            v.push(kind);
        }
        Self(v)
    }
}

impl<'a> Display for CommandStyle<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = super::TICK.deref();
        let cmd = self
            .0
            .iter()
            .map(|kind| kind.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{}{}{}", t, cmd, t)
    }
}

impl<'a> Display for Kind<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let k = match self {
            Kind::Command(c) => c.bright_yellow(),
            Kind::Value(v) => v.white(),
            Kind::Flag(f) => f.bright_black(),
            Kind::String(s) => s.cyan(),
            Kind::Or(o) => o.white(),
            Kind::And(a) => a.white(),
        };
        write!(f, "{}", k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::IntoIter;

    /// Test purpose struct.
    #[derive(Debug, Clone)]
    struct KindIter<'a> {
        input: &'a str,
        iter: IntoIter<Kind<'a>>,
    }

    impl<'a> KindIter<'a> {
        /// Creates a new [`KindIter`] from a given string input.
        fn new(input: &'a str) -> Self {
            let iter = CommandStyle::from(input).0.into_iter();
            Self {
                input,
                iter: iter.clone(),
            }
        }

        /// Asserts the next item.
        fn assert_next(&mut self, expected: Option<Kind<'a>>) {
            assert_eq!(
                self.iter.next(),
                expected,
                "Assertion failed for '{}'",
                self.input
            );
        }
    }

    #[test]
    fn simple_command() {
        let mut iter = KindIter::new("simple-command");
        iter.assert_next(Some(Kind::Command("simple-command")));
        iter.assert_next(None);
    }

    #[test]
    fn or_and() {
        let mut asrt: fn(&str);
        asrt = |separator: &str| {
            let input = format!("cmd {} other-cmd", separator);
            let mut iter = KindIter::new(input.as_str());
            iter.assert_next(Some(Kind::Command("cmd")));
            iter.assert_next(Some(Kind::Or(separator)));
            iter.assert_next(Some(Kind::Command("other-cmd")));
        };
        asrt("|");
        asrt("||");
        asrt = |separator: &str| {
            let input = format!("cmd {} other-cmd", separator);
            let mut iter = KindIter::new(input.as_str());
            iter.assert_next(Some(Kind::Command("cmd")));
            iter.assert_next(Some(Kind::And(separator)));
            iter.assert_next(Some(Kind::Command("other-cmd")));
        };
        asrt("&");
        asrt("&&");
    }

    #[test]
    fn command_and_value() {
        let mut iter = KindIter::new("command value1 value2");
        iter.assert_next(Some(Kind::Command("command")));
        iter.assert_next(Some(Kind::Value("value1")));
        iter.assert_next(Some(Kind::Value("value2")));
        iter.assert_next(None);
    }

    #[test]
    fn string() {
        let mut iter = KindIter::new("command 'and string'");
        iter.assert_next(Some(Kind::Command("command")));
        iter.assert_next(Some(Kind::String("'and string'")));
        iter.assert_next(None);
        let mut iter = KindIter::new("cmd \"and double quote string\"");
        iter.assert_next(Some(Kind::Command("cmd")));
        iter.assert_next(Some(Kind::String("\"and double quote string\"")));
        iter.assert_next(None);
        let mut iter = KindIter::new("\"string\" | piped");
        iter.assert_next(Some(Kind::String("\"string\"")));
        iter.assert_next(Some(Kind::Or("|")));
        iter.assert_next(Some(Kind::Command("piped")));
        iter.assert_next(None);
    }
}
