use jsmoke_utils::{
    print_verbose::PrintVerbose,
    visuals::{
        color_highlights::ColorHighlights,
        tags::{NOTE_TAG, WARNING_TAG},
    },
};
use regex::Regex;
use thiserror::Error;

/// An accpetable regex pattern for the `--lock-version` rules.
const LOCK_VERSION_REGEX: &str =
    r"\s*(([0-9]+(\.[0-9]+){0,2}<=>[0-9]+(\.[0-9]+){0,2})|((\^|\^=|=)?[0-9]+(\.[0-9]+){0,2}))\s*";

/// Possible variants using `--lock-version` flag.
#[derive(Debug)]
pub enum LockVersionVariant {
    /// Version must be equals or greater than...
    GreaterOrEquals(LockVersion),
    /// Version must equals than...
    StrictlyEquals(LockVersion),
    /// Version must be within the range (note that's a range inclusive).
    InRange {
        /// Left lock version (minimum).
        left: LockVersion,
        /// Right lock version (maximum).
        right: LockVersion,
    },
}

impl TryFrom<&str> for LockVersionVariant {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(LOCK_VERSION_REGEX).expect("This is a valid regex, I swear!");
        let m = if re.find(value).map(|m| m.as_str() == value).unwrap_or(false) {
            Ok(value.trim())
        } else {
            Err(ParseError::UnparseableRegex(value.into()))
        }?;
        if m.starts_with("^=") || m.starts_with("^") {
            return Ok(LockVersionVariant::GreaterOrEquals(LockVersion::from(m)));
        } else if m.starts_with("=") {
            return Ok(LockVersionVariant::StrictlyEquals(LockVersion::from(m)));
        }
        let (left, right) = {
            let mut it = m.split("<=>").map(|s| LockVersion::from(s)).into_iter();
            let mut next = || {
                it.next()
                    .expect("this was already checked within regex (safe unwraping)")
            };
            (next(), next())
        };
        match left.partial_cmp(&right) {
            Some(std::cmp::Ordering::Greater) => {
                Err(ParseError::LeftValueGreaterThanRight { left, right })
            }
            _ => Ok(LockVersionVariant::InRange { left, right }),
        }
    }
}

/// Represents a locked version to compile and runs the project.
#[derive(Debug, PartialEq)]
pub struct LockVersion {
    /// The major part of a version pattern (`<MAJOR>._._`).
    major: u16,
    /// The minor part of a version pattern (`_.<MINOR>._`).
    minor: Option<u16>,
    /// The patch part of a version pattern (`_._.<PATCH>`).
    patch: Option<u16>,
}

impl PartialOrd for LockVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let pairs = [
            (Some(self.major), Some(other.major)),
            (self.minor, other.minor),
            (self.patch, other.patch),
        ];
        for i in 0..3 {
            match pairs[i] {
                (Some(l), None) if l > 0 => return Some(std::cmp::Ordering::Greater),
                (None, Some(r)) if r > 0 => return Some(std::cmp::Ordering::Less),
                (Some(l), Some(r)) if l != r => return l.partial_cmp(&r),
                _ => {}
            }
        }
        return Some(std::cmp::Ordering::Equal);
    }
}

impl From<&str> for LockVersion {
    fn from(value: &str) -> Self {
        let mut it = value.split(".").into_iter();
        let mut next = || {
            it.next().map(|s| {
                s.parse::<u16>()
                    .expect("this was already checked within regex (safe unwraping)")
            })
        };
        Self {
            major: next().expect("this was already checked within regex (safe unwraping)"),
            minor: next(),
            patch: next(),
        }
    }
}

impl ToString for LockVersion {
    fn to_string(&self) -> String {
        let mut parts = Vec::with_capacity(3);
        let mut fields = [Some(self.major), self.minor, self.patch].into_iter();
        while let Some(Some(p)) = fields.next() {
            parts.push(p.to_string())
        }
        parts
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(".")
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    /// When lock version parsing fails due to no match regex.
    #[error("couldn't parse the provided lock version (`{0}`)")]
    UnparseableRegex(String),

    /// When the regex is ok but the left match is greater than the
    /// right one (not allowed).
    #[error(
        "couldn't parse the lock version range (left val greather than right: {} | {})",
        .left.to_string(),
        .right.to_string()
    )]
    LeftValueGreaterThanRight {
        left: LockVersion,
        right: LockVersion,
    },
}

impl PrintVerbose for ParseError {
    fn print_verbose(&self) {
        let lock_flag = "`--lock-version`".flag_highlight();
        let caret = r#""^""#.string_highlight();
        let equals = r#""=""#.string_highlight();
        let caret_equals = r#""^=""#.string_highlight();
        let abc = "`A.B.C`".parameter_highlight();
        let left = "<LEFT>".parameter_highlight();
        let right = "<RIGHT>".parameter_highlight();
        let left_right_syntax: String = [
            "`".white_highlight(),
            left.clone(),
            "<=>".white_highlight(),
            right.clone(),
            "`".white_highlight(),
        ]
        .into_iter()
        .collect();
        let regex_rules = [
            format!(
                "\
Can starts with {}, {} or {}, where:
  - {} means greater than
  - {} means strictly equals than
  - {} means greater or equals than
{}no prefix does the same effect of {}.",
                caret, equals, caret_equals, caret, equals, caret_equals, NOTE_TAG, caret_equals
            ),
            format!(
                "\
Contains the {} pattern, where:
  - {} means the major version (required)
  - {} means the minor version (optional)
  - {} means the patch version (optional)
{}the versions are dot-separated.",
                abc,
                "A".parameter_highlight(),
                "B".parameter_highlight(),
                "C".parameter_highlight(),
                NOTE_TAG
            ),
            format!(
                "\
Range specify is allowed with the {}:
  - {} means the minimum version
  - {} means the maximum version
{}the {} must always greater or equals than {}.
",
                left_right_syntax, left, right, WARNING_TAG, right, left
            ),
        ];
        println!();
        match self {
            Self::UnparseableRegex(_) => {
                println!(
                    "The {} syntax is regex based and follows these rules:",
                    lock_flag
                );
                regex_rules.into_iter().enumerate().for_each(|(i, rule)| {
                    let item_index = format!("{}.", i + 1).bullet_point_highlight();
                    print!("{} ", item_index);
                    for (i, r) in rule.lines().enumerate() {
                        if i != 0 {
                            print!("   ");
                        }
                        println!("{}", r);
                    }
                    println!();
                })
            }
            Self::LeftValueGreaterThanRight { .. } => {
                println!("The {} lock version must be greater or equals than", right);
                println!("the {} one.", left);
            }
        }
    }
}
