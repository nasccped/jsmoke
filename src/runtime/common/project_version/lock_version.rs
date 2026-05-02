use super::{
    constraint::{ConstraintError, RangeVersionConstraint, SingleVersionConstraint},
    version_literal::{VersionLiteral, VersionLiteralError},
};
use crate::{
    utils::{
        Inner, SurelyUnwrap, TrimAndBox,
        notifiers::NotifyFailure,
        verbose::{Verbose, VerboseWrapper},
        visuals::style::{ItemList, SuggestionStyle, TermStyle, WeakStyle},
    },
    verbose_wrapper,
};
use regex::Regex;
use std::{cmp::Ordering, sync::LazyLock};
use thiserror::Error;

/// [`Regex`] struct for [`LockVersion::Single`] variant.
static SINGLE_VERSION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\s*(?<constraint>[^\d]*)(?<version>\d+(?:\.\d+){0,2})\s*$"#).surely_unwrap()
});

/// [`Regex`] struct for [`LockVersion::Range`] variant.
static RANGE_VERSION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^\s*(?<left>\d+(?:\.\d+){0,2})(?<constraint>[^\d]+)(?<right>\d+(?:\.\d+){0,2})\s*$"#,
    )
    .surely_unwrap()
});

/// Express a `lock-version` value.
///
/// Can be [`LockVersion::Single`] (ensuring value according with [`SingleVersionConstraint`]), or
/// [`LockVersion::Range`] (ensuring value according with [`RangeVersionConstraint`]).
#[derive(Debug, PartialEq)]
pub enum LockVersion {
    /// When the passed version is a 'single-ver' value (`1.2`, `^23.1`, `...`).
    Single {
        /// Constraint for the provided lock-version.
        constraint: SingleVersionConstraint,
        /// [`VersionLiteral`] value.
        version: VersionLiteral,
    },
    /// When the passed version is a 'range-ver' value (`1..2`, `18..=24.2`, `...`).
    Range {
        /// Constraint for the provided lock-version.
        constraint: RangeVersionConstraint,
        /// left [`VersionLiteral`] value.
        left: VersionLiteral,
        /// right [`VersionLiteral`] value.
        right: VersionLiteral,
    },
}

/// Possible errors when trying to parse [`LockVersion`].
#[derive(Debug, Error)]
pub enum LockVersionError {
    /// Not recognized [`LockVersion`] pattern.
    #[error("not recognized as valid lock-version ({0})")]
    NotRecognized(Box<str>),
    /// When trying to parse `constraint`.
    #[error("symbol not recognized as lock-version constraint ({})", .0.inner())]
    Constraint(ConstraintError),
    /// When [`VersionLiteral`] parsing fails.
    #[error(transparent)]
    VersionLiteral(VersionLiteralError),
    /// When left [`VersionLiteral`] is equals/greater than right one (for range like patterns).
    #[error(
        "left version ({}) can't be equals/greater than right version ({})",
        .left.to_string(),
        .right.to_string()
    )]
    RangeLeftGreaterThanRight {
        /// Left field.
        left: VersionLiteral,
        /// Right field.
        right: VersionLiteral,
    },
}

impl From<ConstraintError> for LockVersionError {
    fn from(value: ConstraintError) -> Self {
        Self::Constraint(value)
    }
}

impl From<VersionLiteralError> for LockVersionError {
    fn from(value: VersionLiteralError) -> Self {
        match value {
            VersionLiteralError::NotRecognized(bs) => Self::NotRecognized(bs),
            other => Self::VersionLiteral(other),
        }
    }
}

impl NotifyFailure for LockVersionError {}

impl Verbose for LockVersionError {
    fn as_verbose(&self) -> VerboseWrapper {
        match self {
            Self::NotRecognized(_)
            | Self::VersionLiteral(VersionLiteralError::NotRecognized(_)) => {
                get_not_recognized_verbose()
            }
            Self::Constraint(c) => c.as_verbose(),
            Self::RangeLeftGreaterThanRight { .. } | Self::VersionLiteral(_) => verbose_wrapper!(),
        }
    }
}

/// Returns the long verbose message for [`LockVersionError::NotRecognized`].
fn get_not_recognized_verbose() -> VerboseWrapper {
    let mut version_examples = VersionLiteral::get_examples();
    version_examples.push("...");
    let version_examples = version_examples
        .into_iter()
        .map(|ex| ex.suggestion_style())
        .collect::<Vec<_>>()
        .join(", ");
    let single_suggestion = "<constraint><version>".suggestion_style();
    let range_suggestion = "<version><constraint><version>".suggestion_style();
    let version_term = "<version>".term_style();
    let constraint_term = "<constraint>".term_style();
    let detailed_single_constraint = format!(
        "{} {} / ({}) {}",
        SingleVersionConstraint::get_strictly_equals_symbol().suggestion_style(),
        "(strictly equals)".weak_style(),
        SingleVersionConstraint::get_equals_or_greater_symbols()
            .into_iter()
            .map(|ex| ex.suggestion_style())
            .collect::<Vec<_>>()
            .join("|"),
        "(equals or greater)".weak_style(),
    )
    .item_list_style()
        + "\n  for single version kind";
    let detailed_range_constraint = format!(
        "{} {} / {} {} for range version",
        RangeVersionConstraint::get_inclusive_symbol().suggestion_style(),
        "(inclusive)".weak_style(),
        RangeVersionConstraint::get_exclusive_symbol().suggestion_style(),
        "(exclusive)".weak_style(),
    )
    .item_list_style()
        + "\n  kind";
    verbose_wrapper!(
        "Lock version can be single like ({})" => single_suggestion;
        "or range like ({})." => range_suggestion;
        "";
        "The {} field must be a 'version' like pattern, such" => version_term;
        "as {}" => version_examples;
        "";
        "The {} field must be a 'constraint' like pattern, such" => constraint_term;
        "as:";
        "";
        "{} or" => detailed_single_constraint;
        "{}." => detailed_range_constraint;
    )
}

impl TryFrom<&str> for LockVersion {
    type Error = LockVersionError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if SINGLE_VERSION_REGEX.is_match(value) {
            try_single(value)
        } else if RANGE_VERSION_REGEX.is_match(value) {
            try_range(value)
        } else {
            Err(LockVersionError::NotRecognized(value.trim_and_box()))
        }
    }
}

/// Tries to build a [`LockVersion::Single`] over a string slice.
fn try_single(value: &str) -> Result<LockVersion, LockVersionError> {
    let captures = SINGLE_VERSION_REGEX
        .captures(value)
        .ok_or(LockVersionError::NotRecognized(value.trim_and_box()))?;
    let constraint = SingleVersionConstraint::try_from(
        captures
            .name("constraint")
            .map(|m| m.as_str())
            .unwrap_or_default(),
    )?;
    let version = VersionLiteral::try_from(
        captures
            .name("version")
            .map(|m| m.as_str())
            .unwrap_or_default(),
    )?;
    Ok(LockVersion::Single {
        constraint,
        version,
    })
}

/// Tries to build a [`LockVersion::Range`] over a string slice.
fn try_range(value: &str) -> Result<LockVersion, LockVersionError> {
    let captures = RANGE_VERSION_REGEX
        .captures(value)
        .ok_or(LockVersionError::NotRecognized(value.trim_and_box()))?;
    let left = VersionLiteral::try_from(
        captures
            .name("left")
            .map(|m| m.as_str())
            .unwrap_or_default(),
    )?;
    let constraint = RangeVersionConstraint::try_from(
        captures
            .name("constraint")
            .map(|m| m.as_str())
            .unwrap_or_default(),
    )?;
    let right = VersionLiteral::try_from(
        captures
            .name("right")
            .map(|m| m.as_str())
            .unwrap_or_default(),
    )?;
    (left.cmp(&right) == Ordering::Less)
        .then(|| LockVersion::Range {
            left: left.clone(),
            right: right.clone(),
            constraint: constraint.clone(),
        })
        .ok_or(LockVersionError::RangeLeftGreaterThanRight { left, right })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single() {
        [
            "1", "1.2", "1.2.3", " 1", "1 ", " 1 ", "=1", "= 1", " =1", " =1 ", "^1", "^=1", "^ 1",
            " ^1", " ^ 1 ", ">=1", ">1", " >1", " > 1 ",
        ]
        .into_iter()
        .for_each(|inp| {
            assert!(
                LockVersion::try_from(inp).is_ok_and(|lv| matches!(lv, LockVersion::Single { .. }))
            )
        });
    }

    #[test]
    fn range() {
        [
            "1..2",
            "1.. 2",
            "1 ..3",
            "1 .. 2",
            "1 . . 2",
            "1..=2",
            "1..= 2",
            "1 ..=2",
            "1 . .= 2",
            "1.   . =2",
        ]
        .into_iter()
        .for_each(|inp| {
            assert!(
                LockVersion::try_from(inp).is_ok_and(|lv| matches!(lv, LockVersion::Range { .. }))
            )
        });
    }
}
