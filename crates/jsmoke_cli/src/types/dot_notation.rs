//! # Dot Notation module
//!
//! Provides access to the [`DotNotation`] struct + it's parsing.
use jsmoke_utils::{
    print_verbose::PrintVerbose,
    visuals::{color_highlights::ColorHighlights, tags::NOTE_TAG},
};
use regex::Regex;
use thiserror::Error;

/// Common syntax for dot notation matching.
const DOT_SYNTAX: &str = r"\s*([a-z][a-z0-9]*)(\.[a-z][a-z0-9]*)*\s*";

/// Represents search repetition in a regex pattern.
enum GroupTimeCount {
    /// At least this amount.
    MinimumOf(usize),
    /// Until this amount.
    MaximumOf(usize),
    /// Between this amount.
    Between(usize, usize),
    /// Means the same as `0 or more` in regex pattern (`a*`).
    Any,
}

/// Holds the `Dot Syntax` fields:
pub struct DotSyntaxFields {
    /// The regex pattern to be applied when looking for words.
    pattern: String,

    /// How many times the [`Self::pattern`] can repeat (separated by
    /// dot (`.`)).
    times: Option<GroupTimeCount>,

    /// Is leading spaces allowed.
    leading_spaces: Option<bool>,
}

impl<T: Iterator<Item = usize>> DotSyntaxFields {
    /// Creates a new [`DotSyntax`] by a given pattern. Note that
    fn new(pattern: impl ToString) -> Self {
        let (pattern, times, leading_spaces) = (pattern.to_string(), None, None);
        Self {
            pattern,
            times,
            leading_spaces,
        }
    }
    fn with_times(self, times: T) -> Self {
        let (pattern, times, leading_spaces) = (self.pattern, Some(times), self.leading_spaces);
        Self {
            pattern,
            times,
            leading_spaces,
        }
    }
    fn with_leading_spaces(self, leading_spaces: bool) -> Self {
        let (pattern, times, leading_spaces) = (self.pattern, self.times, Some(leading_spaces));
        Self {
            pattern,
            times,
            leading_spaces,
        }
    }
}

/// Dot notation for `pom.xml` like syntax. This struct shouldn't be
/// implement by hand. Instead, you should implement it by using its
/// wrappers (commonly disposed at [`crate::types::project`]).
#[derive(Debug)]
pub struct DotNotation(Vec<String>);

impl TryFrom<&str> for DotNotation {
    type Error = ParseError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(DOT_SYNTAX).expect("this is a valid regex, i swear!");
        let s = if re.find(s).map(|m| m.as_str() == s).unwrap_or(false) {
            Ok(s.trim())
        } else {
            Err(ParseError(s.into()))
        }?;
        Ok(Self(s.split(".").map(|word| word.to_string()).collect()))
    }
}

/// When the [`DotNotation`] parsing fails.
#[derive(Debug, Error)]
#[error("couldn't parse the provided WordNotation (`{0}`)")]
pub struct ParseError(String);

impl PrintVerbose for ParseError {
    fn print_verbose(&self) {
        let d_not = "`WordNotation`".type_highlight();
        println!();
        println!("This field is based on {} syntax:", d_not);
        println!("  - only lowercases");
        println!("  - dot separated words");
        println!("  - no leading spaces");
        println!("  - only alphanumeric (no emoji/complex glyphs)");
        println!("  - only alphabetic (a-z) word starting");
        println!();
        println!(
            "{}about leading spaces, left and right will be trimed",
            NOTE_TAG
        );
        println!("but the program can't handle middle whitespaces.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OK_SAMPLES: [&str; 3] = [
        "  this.is.a.group   ",
        "a.second.group.here",
        "group1.with2.number3",
    ];
    const ERR_SAMPLES: [&str; 5] = [
        "Uppercase.group",
        "underscore_separated",
        "middle. .whitespace",
        "cool.emoji| 😎",
        "starting.with.number.5",
    ];

    #[test]
    fn parse_success() {
        OK_SAMPLES
            .into_iter()
            .for_each(|s| assert!(DotNotation::try_from(s).is_ok()))
    }

    #[test]
    fn parse_fails() {
        ERR_SAMPLES
            .into_iter()
            .for_each(|s| assert!(DotNotation::try_from(s).is_err()))
    }
}
