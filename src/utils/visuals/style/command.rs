use super::{dollar::DollarIt, ticks::TickIt};
use crate::utils::SurelyUnwrap;
use colored::{ColoredString, Colorize};
use regex::{Captures, Match, Regex};
use std::sync::LazyLock;

mod groups {
    //! # Regex groups
    //!
    //! Separate matching by groups (easy tokenize).

    /// [`super::Token::OrOper`] token group.
    pub const OROPER: &str = "oroper";

    /// [`super::Token::AndOper`] token group.
    pub const ANDOPER: &str = "andoper";

    /// [`super::Token::Command`] token group.
    pub const COMMAND: &str = "command";

    /// [`super::Token::Flag`] token group.
    pub const FLAG: &str = "flag";

    /// [`super::Token::String`] token group.
    pub const STRING: &str = "string";

    /// [`super::Token::Subcommand`] token group.
    pub const SUBCOMMAND: &str = "subcommand";

    /// [`super::Token::Number`] token group.
    pub const NUMBER: &str = "number";

    /// [`super::Token::Other`] token group.
    pub const OTHER: &str = "other";
}

/// Command [`Regex`] pattern. Also holds the [`groups::OROPER`] and [`groups::ANDOPER`] regex
/// groups.
static COMMAND_PATTERN: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"((^|(?<{}>\|\|)\s+|(?<{}>&&)\s+)(?<{}>[a-zA-Z][\w-]+)($|\s))"#,
        groups::OROPER,
        groups::ANDOPER,
        groups::COMMAND
    )
});

/// Flag [`Regex`] pattern.
static FLAG_PATTERN: LazyLock<String> =
    LazyLock::new(|| format!(r#"(?<{}>-+[^\s]*)"#, groups::FLAG));

/// String [`Regex`] pattern.
static STRING_PATTERN: LazyLock<String> =
    LazyLock::new(|| format!(r#"(?<{}>'[^']*'|"[^"]*")"#, groups::STRING));

/// Subcommand [`Regex`] pattern.
static SUBCOMMAND_PATTERN: LazyLock<String> =
    LazyLock::new(|| format!(r#"(?<{}>[a-zA-Z][\w-]*)($|\s)"#, groups::SUBCOMMAND));

/// Number [`Regex`] pattern.
static NUMBER_PATTERN: LazyLock<String> =
    LazyLock::new(|| format!(r#"(?<{}>[\d]+)"#, groups::NUMBER));

/// Other [`Regex`] pattern for other kind of tokens (path, negation, etc).
static OTHER_REGEX: LazyLock<String> = LazyLock::new(|| format!(r#"(?<{}>[^\s]+)"#, groups::OTHER));

/// Apply color style like as command (requires item to implements [`AsRef<str>`] trait).
pub trait CommandStyle: AsRef<str> {
    /// Apply a color style to the designed item. The color being applied depends on how the impl
    /// was done.
    fn command_style(&self) -> String;
}

impl<T: AsRef<str>> CommandStyle for T {
    fn command_style(&self) -> String {
        let value = self.as_ref().trim();
        Token::get_tokens(value)
            .into_iter()
            .map(|tk| tk.color_it().to_string())
            .collect::<Vec<_>>()
            .join(" ")
            .dollar_it()
            .tick_it()
    }
}

static TOKENS_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let mut re_string = String::from(COMMAND_PATTERN.as_str());
    let mut push_regex = |re: &str| {
        re_string.push('|');
        re_string.push_str(re);
    };
    push_regex(STRING_PATTERN.as_str());
    push_regex(FLAG_PATTERN.as_str());
    push_regex(SUBCOMMAND_PATTERN.as_str());
    push_regex(NUMBER_PATTERN.as_str());
    push_regex(OTHER_REGEX.as_str());
    Regex::new(re_string.as_str()).surely_unwrap()
});

/// The possible variant for the matching token.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
enum Token<'a> {
    /// When the provided token is `command` kind.
    Command(&'a str),
    /// When the provided token is `subcommand` kind.
    Subcommand(&'a str),
    /// When the provided token is `flag` kind.
    Flag(&'a str),
    /// When the provided token is `string` kind.
    String(&'a str),
    /// When the provided token is `andoper` kind.
    AndOper,
    /// When the provided token is `oroper` kind.
    OrOper,
    /// When the provided token is `number` kind.
    Number(&'a str),
    /// Path and other stuff.
    Other(&'a str),
}

impl<'a> Into<&'a str> for Token<'a> {
    fn into(self) -> &'a str {
        match self {
            Self::Command(c) => c,
            Self::Subcommand(s) => s,
            Self::Flag(f) => f,
            Self::String(s) => s,
            Self::AndOper => "&&",
            Self::OrOper => "||",
            Self::Number(n) => n,
            Self::Other(o) => o,
        }
    }
}

impl<'h> Token<'h> {
    /// Converts a haystack into a [`Vec<Token>`].
    fn get_tokens(value: &'h str) -> Vec<Self> {
        let mut v = Vec::new();
        type MatchFromCapture<'h> = fn(&Captures<'h>) -> Option<Match<'h>>;
        type StrToToken<'h> = fn(&'h str) -> Token<'h>;
        let mappers: [(MatchFromCapture, StrToToken); 8] = [
            (
                |c: &Captures<'h>| c.name(groups::COMMAND),
                |cmd: &str| Token::Command(cmd),
            ),
            (
                |c: &Captures<'h>| c.name(groups::SUBCOMMAND),
                |subcmd: &str| Token::Subcommand(subcmd),
            ),
            (
                |c: &Captures<'h>| c.name(groups::FLAG),
                |flag: &str| Token::Flag(flag),
            ),
            (
                |c: &Captures<'h>| c.name(groups::STRING),
                |s: &str| Token::String(s),
            ),
            (
                |c: &Captures<'h>| c.name(groups::ANDOPER),
                |_: &str| Token::AndOper,
            ),
            (
                |c: &Captures<'h>| c.name(groups::OROPER),
                |_: &str| Token::OrOper,
            ),
            (
                |c: &Captures<'h>| c.name(groups::NUMBER),
                |n: &str| Token::Number(n),
            ),
            (
                |c: &Captures<'h>| c.name(groups::OTHER),
                |o: &str| Token::Other(o),
            ),
        ];
        for cap in TOKENS_REGEX.captures_iter(value) {
            let mut groups: Vec<(Match<'h>, StrToToken)> = mappers
                .into_iter()
                .filter_map(|(macther, conversor)| macther(&cap).map(|res| (res, conversor)))
                .collect();
            groups.sort_by_key(|(m, _)| m.start());
            v.extend(
                groups
                    .iter()
                    .map(|(matcher, conversor)| conversor(matcher.as_str())),
            );
        }
        v
    }

    /// Apply the designed color to the self [`Token`].
    fn color_it(&self) -> ColoredString {
        match self {
            Self::Command(c) => c.bright_yellow(),
            Self::Subcommand(s) => s.white(),
            Self::Flag(f) => f.bright_black(),
            Self::String(s) => s.cyan(),
            Self::Other(o) => o.bright_purple(),
            Self::Number(n) => n.yellow(),
            Self::AndOper | Self::OrOper => Token::into(self).bright_white(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ops::{RangeFrom, RangeInclusive};

    /// Common inputs.
    const INPUTS: [&str; 6] = [
        r#"abcde fghij --klmno -p --qrs=tu 'vwxyz' "01234""#,
        r#"ABCDE FGHIJ --KLMNO -P --QRS=TU 'VWXYZ' "01234""#,
        r#"AbCdE FgHiJ --KlMnO -p --qRs=tU 'vWxYz' "01234""#,
        r#"Ab_dE Fg_iJ --KlM_O -p --q_s=_U 'vW_Yz' "_12_4""#,
        r#"Ab-d- Fg-i- ---l-n- -p --q-s=t- 'v-x-z' "0-2-4""#,
        r#"Ab0dE Fg1iJ --KlM2O -p --q3s=4U 'vW5Yz' "61274""#,
    ];

    /// Index for `command` tokens (based on [`INPUTS`]).
    const CMD_IND: usize = 0;

    /// Index for `subcommand` tokens (based on [`INPUTS`]).
    const SUBCMD_IND: usize = 1;

    /// Indexes for `flag` tokens (based on [`INPUTS`]).
    const FLAG_INDS: RangeInclusive<usize> = 2..=4;

    /// Indexes for `string` tokens (based on [`INPUTS`]).
    const STRING_INDS: RangeFrom<usize> = 5..;

    /// Generates an enumerated iterator over [`Token`]s.
    fn enumerated_tokens<'a>(s: &'a str) -> impl IntoIterator<Item = (usize, Token<'a>)> {
        Token::get_tokens(s).into_iter().enumerate()
    }

    #[test]
    fn cmd() {
        for row in INPUTS {
            enumerated_tokens(row).into_iter().for_each(|(i, t)| {
                if i == CMD_IND {
                    assert!(
                        matches!(t, Token::Command(_)),
                        "`Command` token expected, but got `{:?}`",
                        t
                    );
                } else {
                    assert!(
                        !matches!(t, Token::Command(_)),
                        "`Command` token not expected: `{:?}`",
                        t
                    );
                }
            });
        }
    }

    #[test]
    fn subcmd() {
        for row in INPUTS {
            enumerated_tokens(row).into_iter().for_each(|(i, t)| {
                if i == SUBCMD_IND {
                    assert!(
                        matches!(t, Token::Subcommand(_)),
                        "`Subcommand` token expected, but got `{:?}`",
                        t
                    );
                } else {
                    assert!(
                        !matches!(t, Token::Subcommand(_)),
                        "`Subcommand` token not expected: `{:?}`",
                        t
                    );
                }
            });
        }
    }

    #[test]
    fn flag() {
        for row in INPUTS {
            enumerated_tokens(row).into_iter().for_each(|(i, t)| {
                if FLAG_INDS.contains(&i) {
                    assert!(
                        matches!(t, Token::Flag(_)),
                        "`Flag` token expected, but got `{:?}`",
                        t
                    );
                } else {
                    assert!(
                        !matches!(t, Token::Flag(_)),
                        "`Flag` token not expected: `{:?}`",
                        t
                    );
                }
            });
        }
    }

    #[test]
    fn string() {
        for row in INPUTS {
            enumerated_tokens(row).into_iter().for_each(|(i, t)| {
                if STRING_INDS.contains(&i) {
                    assert!(
                        matches!(t, Token::String(_)),
                        "`String` token expected, but got `{:?}`",
                        t
                    );
                } else {
                    assert!(
                        !matches!(t, Token::String(_)),
                        "`String` token not expected: `{:?}`",
                        t
                    );
                }
            });
        }
    }

    #[test]
    fn and_testing() {
        let inp = "touch file && open-it";
        let mut tks = Token::get_tokens(inp).into_iter();
        assert_eq!(tks.next(), Some(Token::Command("touch")));
        assert_eq!(tks.next(), Some(Token::Subcommand("file")));
        assert_eq!(tks.next(), Some(Token::AndOper));
        assert_eq!(tks.next(), Some(Token::Command("open-it")));
        assert_eq!(tks.next(), None);
    }

    #[test]
    fn or_testing() {
        let inp = "touch file || open-it";
        let mut tks = Token::get_tokens(inp).into_iter();
        assert_eq!(tks.next(), Some(Token::Command("touch")));
        assert_eq!(tks.next(), Some(Token::Subcommand("file")));
        assert_eq!(tks.next(), Some(Token::OrOper));
        assert_eq!(tks.next(), Some(Token::Command("open-it")));
        assert_eq!(tks.next(), None);
    }

    #[test]
    fn number_testing() {
        let inp = "run 23 --as 32";
        let mut tks = Token::get_tokens(inp).into_iter();
        assert_eq!(tks.next(), Some(Token::Command("run")));
        assert_eq!(tks.next(), Some(Token::Number("23")));
        assert_eq!(tks.next(), Some(Token::Flag("--as")));
        assert_eq!(tks.next(), Some(Token::Number("32")));
        assert_eq!(tks.next(), None);
    }

    #[test]
    fn extra_cases() {
        let inp = "mkdir -p 'parent' && cd 'child'";
        let mut tks = Token::get_tokens(inp).into_iter();
        assert_eq!(tks.next(), Some(Token::Command("mkdir")));
        assert_eq!(tks.next(), Some(Token::Flag("-p")));
        assert_eq!(tks.next(), Some(Token::String("'parent'")));
        assert_eq!(tks.next(), Some(Token::AndOper));
        assert_eq!(tks.next(), Some(Token::Command("cd")));
        assert_eq!(tks.next(), Some(Token::String("'child'")));
        assert_eq!(tks.next(), None);
    }
}
