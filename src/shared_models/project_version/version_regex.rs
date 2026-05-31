use crate::utils::SurelyUnwrap;
use regex::{Captures, Regex};
use std::{ops::Deref, sync::LazyLock};

/// Different available constraint for versions.
enum ConstraintKind {
    /// Means `single version` kind constraints, like:
    /// - `=...`  => strictly equals to `...`
    /// - `>=...` => equals or greater than `...`
    Single,
    /// Means `range version` kind constraints, like:
    /// - `{left}..{right}`  => any version that's from `{left}` until `{N}` when `N` < `{right}`
    /// (exclusive)
    /// - `{left}..={right}` =>  any version that's from `{left}` until `{N}` when `N` <= `{right}`
    /// (inclusive)
    Range,
}

/// [`Regex`] specific for constraint kind matching.
struct ConstraintRegex(ConstraintKind);

impl ToString for ConstraintRegex {
    fn to_string(&self) -> String {
        r#"(?<GROUP_NAME>[^\d<DOT_EXCLUDE>\s]<LENGTH>)<OPTIONAL>"#
            .replace("<GROUP_NAME>", &format!("<{}>", Self::group_name()))
            .replace("<DOT_EXCLUDE>", if self.dot_exclude() { "\\." } else { "" })
            .replace("<LENGTH>", self.regex_length_str())
            .replace("<OPTIONAL>", if self.is_optional() { "?" } else { "" })
    }
}

impl ConstraintRegex {
    /// Builds a new [`ConstraintRegex`] based on a [`ConstraintKind`].
    fn new(kind: ConstraintKind) -> ConstraintRegex {
        Self(kind)
    }

    /// If dots should be excluded from matching.
    fn dot_exclude(&self) -> bool {
        matches!(self.0, ConstraintKind::Single)
    }

    /// Returns the regex group name for a [`ConstraintRegex`] item.
    fn group_name() -> &'static str {
        "constraint"
    }

    /// Returns the common regex string for length specification.
    fn regex_length_str(&self) -> &'static str {
        match self.0 {
            ConstraintKind::Single => "+",
            ConstraintKind::Range => "{2,}",
        }
    }

    /// If the constraint is optional (specific for [`ConstraintKind::Single`]).
    fn is_optional(&self) -> bool {
        matches!(self.0, ConstraintKind::Single)
    }
}

/// Different kinds of version in a [`VersionRegex`] haystack.
enum VersionKind {
    /// Single version input (`1`, `1.2`, `>=23.4.1`, `...`).
    Single,
    /// Left part from a range kind haystack (`<left>..=[...]`).
    RangeLeft,
    /// Right part from a range kind haystack (`[...]..=<right>`).
    RangeRight,
}

/// [`Regex`] specific for version matching.
///
/// This struct uses `Private` prefix to avoid duplicate conflict with [`VersionRegex`] public
/// item.
struct PrivateVersionRegex(VersionKind);

impl PrivateVersionRegex {
    /// Creates a new [`PrivateVersionRegex`] from a [`VersionKind`].
    fn new(kind: VersionKind) -> Self {
        Self(kind)
    }

    /// Returns the group name based on the `self` item.
    fn group_name(&self) -> &'static str {
        match self.0 {
            VersionKind::Single => "version",
            VersionKind::RangeLeft => "leftversion",
            VersionKind::RangeRight => "rightversion",
        }
    }

    /// Returns the major group for an entire version (`(?<group>\\d+)[...]`).
    fn major_group_name(&self) -> &'static str {
        "major"
    }

    /// Returns the minor group for an entire version (`[...](?:\\.)(?<group>\\d+)[...]`).
    fn minor_group_name(&self) -> &'static str {
        "minor"
    }

    /// Returns the patch group for an entire version (`[...](?:\\.)(?<group>\\d+)`).
    fn patch_group_name(&self) -> &'static str {
        "patch"
    }
}

impl ToString for PrivateVersionRegex {
    fn to_string(&self) -> String {
        match self.0 {
            VersionKind::Single => {
                r#"(?<GROUP_NAME>(?<MAJOR>\d+)(?:\.(?<MINOR>\d+))?(?:\.(?<PATCH>\d+))?)"#
                    .replace("<GROUP_NAME>", &format!("<{}>", self.group_name()))
                    .replace("<MAJOR>", &format!("<{}>", self.major_group_name()))
                    .replace("<MINOR>", &format!("<{}>", self.minor_group_name()))
                    .replace("<PATCH>", &format!("<{}>", self.patch_group_name()))
            }
            _ => r#"(?<GROUP_NAME>\d+(\.\d+){0,2})"#
                .replace("<GROUP_NAME>", &format!("<{}>", self.group_name())),
        }
    }
}

struct SingleKindRegex(Regex);

impl Deref for SingleKindRegex {
    type Target = Regex;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct RangeKindRegex(Regex);

impl Deref for RangeKindRegex {
    type Target = Regex;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SingleKindRegex {
    fn new() -> Self {
        let constraint = ConstraintRegex::new(ConstraintKind::Single);
        let version = PrivateVersionRegex::new(VersionKind::Single);
        Self(
            Regex::new(&format!(
                "^{}\\s*{}$",
                constraint.to_string(),
                version.to_string()
            ))
            .surely_unwrap(),
        )
    }
}

/// [`Regex`] for `single kind` haystack matching.
static SINGLE_KIND_REGEX: LazyLock<SingleKindRegex> = LazyLock::new(SingleKindRegex::new);

/// [`Regex`] for `range kind` haystack matching.
static RANGE_KIND_REGEX: LazyLock<RangeKindRegex> = LazyLock::new(|| {
    let constraint = ConstraintRegex::new(ConstraintKind::Range);
    let left = PrivateVersionRegex::new(VersionKind::RangeLeft);
    let right = PrivateVersionRegex::new(VersionKind::RangeRight);
    RangeKindRegex(
        Regex::new(&format!(
            "^{}\\s*{}\\s*{}$",
            left.to_string(),
            constraint.to_string(),
            right.to_string()
        ))
        .surely_unwrap(),
    )
});

#[derive(Debug)]
pub struct SingleVersionCaptures<'a>(Captures<'a>);

#[derive(Debug)]
pub struct RangeVersionCaptures<'a>(Captures<'a>);

/// [`Regex`] utilities for [`super::ProjectVersion`].
pub struct VersionRegex;

impl VersionRegex {
    /// If the provided haystack refers to a [`super::ProjectVersion::Single`] variant.
    pub fn is_single_kind(haystack: &str) -> bool {
        SINGLE_KIND_REGEX.is_match(haystack)
    }

    /// If the provided haystack refers to the [`super::ProjectVersion::Range`] variant.
    pub fn is_range_kind(haystack: &str) -> bool {
        RANGE_KIND_REGEX.is_match(haystack)
    }

    pub fn may_single<'a>(haystack: &'a str) -> Option<SingleVersionCaptures<'a>> {
        SINGLE_KIND_REGEX
            .captures(haystack)
            .map(SingleVersionCaptures)
    }

    pub fn may_range<'a>(haystack: &'a str) -> Option<RangeVersionCaptures<'a>> {
        RANGE_KIND_REGEX
            .captures(haystack)
            .map(RangeVersionCaptures)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{
        any::type_name,
        fmt::{Debug, Display},
    };

    /// Assert if a value is [`Some`], otherwise panics with a template + an extra message.
    fn assert_is_some<T>(
        init_input: impl Display,
        value: Option<T>,
        message: Option<impl Display>,
    ) {
        assert!(
            value.is_some(),
            "`{}` was expected to result in `Some({})` but `None` was returned.{}",
            init_input,
            type_name::<T>(),
            message
                .map(|mes| format!("\n\nextra message:\n{}", mes))
                .unwrap_or_default()
        )
    }

    /// Assert if a value is [`None`], otherwise panics with a template + an extra message.
    fn assert_is_none<T: Debug>(
        init_input: impl Display,
        value: Option<T>,
        message: Option<impl Display>,
    ) {
        assert!(
            value.is_none(),
            "`{}` was expected to result in `None` but `Some({:?})` was returned.{}",
            init_input,
            value,
            message
                .map(|mes| format!("\n\nextra message:\n{}", mes))
                .unwrap_or_default()
        )
    }

    #[test]
    fn is_single_kind() {
        [
            "1",
            "1.2",
            "1.2.3",
            "12.34.56",
            "1.23.456",
            "=1",
            "=1.2",
            "=1.2.3",
            "=12.34.56",
            "=1.23.456",
            "= 1",
            ">=1.2.3",
            ">= 1.2.3",
        ]
        .into_iter()
        .for_each(|input| {
            let (single, range) = (
                VersionRegex::may_single(input),
                VersionRegex::may_range(input),
            );
            assert_is_some(
                input,
                single,
                Some(format!(
                    "failed for regex: `{}`",
                    SINGLE_KIND_REGEX.as_str()
                )),
            );
            assert_is_none(
                input,
                range,
                Some(format!(
                    "failed for regex: `{}`",
                    SINGLE_KIND_REGEX.as_str()
                )),
            );
        });
    }

    #[test]
    fn is_range_kind() {
        [
            "1..2",
            "1 ..2",
            "1.. 2",
            "1 .. 2",
            "12 .. 34",
            "12.34 .. 56.78",
            "12.34 .. 5",
            "1 .. 5.6.7",
            "1..=2",
            "1 ..=2",
            "1..= 2",
            "1 ..= 2",
            "1.2 ..= 2.3",
        ]
        .into_iter()
        .for_each(|input| {
            let (single, range) = (
                VersionRegex::may_single(input),
                VersionRegex::may_range(input),
            );
            assert_is_none(
                input,
                single,
                Some(format!(
                    "failed for regex: `{}`",
                    SINGLE_KIND_REGEX.as_str()
                )),
            );
            assert_is_some(
                input,
                range,
                Some(format!(
                    "failed for regex: `{}`",
                    SINGLE_KIND_REGEX.as_str()
                )),
            );
        });
    }
}
