use crate::utils::SurelyUnwrap;
use regex::{Captures, Regex};
use std::{
    fmt::{self, Display, Formatter},
    ops::Deref,
    sync::LazyLock,
};

/// Different available constraint for versions.
enum ConstraintKind {
    /// Means `single version` kind constraints, like:
    /// - `=...`  => strictly equals to `...`
    /// - `>=...` => equals or greater than `...`
    Single,
    /// Means `range version` kind constraints, like:
    /// - `{left}..{right}`  => any version that's from `{left}` until `{N}` when `N` < `{right}`
    ///   (exclusive)
    /// - `{left}..={right}` =>  any version that's from `{left}` until `{N}` when `N` <= `{right}`
    ///   (inclusive)
    Range,
}

/// [`Regex`] specific for constraint kind matching.
struct ConstraintRegex(ConstraintKind);

impl Display for ConstraintRegex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                ConstraintKind::Single => Self::SINGLE_KIND_REGEX_STR,
                ConstraintKind::Range => Self::RANGE_KIND_REGEX_STR,
            }
            .clone()
        )
    }
}

impl ConstraintRegex {
    const GROUP_NAME: &str = "constraint";

    /// [`Regex`] string for single kind constraints.
    const SINGLE_KIND_REGEX_STR: LazyLock<String> =
        LazyLock::new(|| format!(r#"(?<{}>[^\d\.\s]*)?"#, Self::GROUP_NAME));

    /// [`Regex`] string for range kind constraints.
    const RANGE_KIND_REGEX_STR: LazyLock<String> =
        LazyLock::new(|| format!(r#"(?<{}>[^\d\s]{{2,}})"#, Self::GROUP_NAME));

    /// Builds a new [`ConstraintRegex`] based on a [`ConstraintKind`].
    fn new(kind: ConstraintKind) -> ConstraintRegex {
        Self(kind)
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

impl Display for PrivateVersionRegex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                VersionKind::Single => Self::SINGLE_KIND_REGEX_STR,
                VersionKind::RangeLeft => Self::RANGE_LEFT_KIND_REGEX_STR,
                VersionKind::RangeRight => Self::RANGE_RIGHT_KIND_REGEX_STR,
            }
            .clone()
        )
    }
}

impl PrivateVersionRegex {
    const SINGLE_GROUP_NAME: &str = "version";
    const LEFT_GROUP_NAME: &str = "leftversion";
    const RIGHT_GROUP_NAME: &str = "rightversion";
    const MAJOR_GROUP_NAME: &str = "major";
    const MINOR_GROUP_NAME: &str = "minor";
    const PATCH_GROUP_NAME: &str = "patch";

    /// [`Regex`] string for single kind [`PrivateVersionRegex`]. It strictly match and group
    /// haystacks like: `1`, `1.2`, `=1.2`, `...`.
    const SINGLE_KIND_REGEX_STR: LazyLock<String> = LazyLock::new(|| {
        format!(
            r#"(?<{}>(?<{}>\d+)(?:\.(?<{}>\d+))?(?:\.(?<{}>\d+))?)"#,
            Self::SINGLE_GROUP_NAME,
            Self::MAJOR_GROUP_NAME,
            Self::MINOR_GROUP_NAME,
            Self::PATCH_GROUP_NAME
        )
    });

    /// [`Regex`] string for left side range matching. Works the same as
    /// [`PrivateVersionRegex::RANGE_RIGHT_KIND_REGEX_STR`] but with a different group name (avoid
    /// [`Regex::new`] runtime bugs).
    const RANGE_LEFT_KIND_REGEX_STR: LazyLock<String> =
        LazyLock::new(|| format!(r#"(?<{}>\d+(\.\d+){{0,2}})"#, Self::LEFT_GROUP_NAME));

    /// [`Regex`] string for left side range matching. Works the same as
    /// [`PrivateVersionRegex::RANGE_LEFT_KIND_REGEX_STR`] but with a different group name (avoid
    /// [`Regex::new`] runtime bugs).
    const RANGE_RIGHT_KIND_REGEX_STR: LazyLock<String> =
        LazyLock::new(|| format!(r#"(?<{}>\d+(\.\d+){{0,2}})"#, Self::RIGHT_GROUP_NAME));

    /// Creates a new [`PrivateVersionRegex`] from a [`VersionKind`].
    fn new(kind: VersionKind) -> Self {
        Self(kind)
    }
}

/// [`Regex`] for `single kind` haystack matching.
static SINGLE_KIND_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let constraint = ConstraintRegex::new(ConstraintKind::Single);
    let version = PrivateVersionRegex::new(VersionKind::Single);
    Regex::new(&format!("^{}\\s*{}$", constraint, version)).surely_unwrap()
});

/// [`Regex`] for `range kind` haystack matching.
static RANGE_KIND_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let constraint = ConstraintRegex::new(ConstraintKind::Range);
    let left = PrivateVersionRegex::new(VersionKind::RangeLeft);
    let right = PrivateVersionRegex::new(VersionKind::RangeRight);
    Regex::new(&format!("^{}\\s*{}\\s*{}$", left, constraint, right)).surely_unwrap()
});

/// Common trait for constraint unwrapping (polymorphism between [`SingleVersionCaptures`] and
/// [`RangeVersionCaptures`]).
trait ConstraintGetter {
    /// Get the constraint string.
    fn get_constraint(&self) -> Result<Option<&str>, ()>;
}

impl<'a> ConstraintGetter for SingleVersionCaptures<'a> {
    fn get_constraint(&self) -> Result<Option<&str>, ()> {
        Ok(self.name(ConstraintRegex::GROUP_NAME).map(|m| m.as_str()))
    }
}

impl<'a> ConstraintGetter for RangeVersionCaptures<'a> {
    fn get_constraint(&self) -> Result<Option<&str>, ()> {
        self.name(ConstraintRegex::GROUP_NAME)
            .ok_or(())
            .map(|m| Some(m.as_str()))
    }
}

#[derive(Debug)]
pub struct SingleVersionCaptures<'a>(Captures<'a>);

impl<'a> Deref for SingleVersionCaptures<'a> {
    type Target = Captures<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct RangeVersionCaptures<'a>(Captures<'a>);

#[derive(Debug)]
pub struct VersionCaptures<'a>(Captures<'a>);

impl<'a> Deref for RangeVersionCaptures<'a> {
    type Target = Captures<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// [`Regex`] utilities for [`super::ProjectVersion`].
pub struct VersionRegex;

impl VersionRegex {
    /// Returns a wrapper for [`PrivateVersionRegex`] ([`SingleVersionCaptures`]) captures (if it's
    /// [`VersionKind::Single`]), otherwise, returns [`None`].
    pub fn may_single<'a>(haystack: &'a str) -> Option<SingleVersionCaptures<'a>> {
        SINGLE_KIND_REGEX
            .captures(haystack)
            .map(SingleVersionCaptures)
    }

    /// Returns a wrapper for [`PrivateVersionRegex`] (both sides of [`RangeVersionCaptures`] -
    /// [`VersionKind::RangeLeft`] and [`VersionKind::RangeRight`]) captures, otherwise, returns
    /// [`None`].
    pub fn may_range<'a>(haystack: &'a str) -> Option<RangeVersionCaptures<'a>> {
        RANGE_KIND_REGEX
            .captures(haystack)
            .map(RangeVersionCaptures)
    }

    #[allow(private_bounds)]
    pub fn get_constraint<T: ConstraintGetter>(item: &T) -> Result<Option<&str>, ()> {
        item.get_constraint()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{
        any::type_name,
        fmt::{Debug, Display},
    };

    const SINGLE_INPUTS: [&str; 13] = [
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
    ];

    const RANGE_ALSO_INCLUSIVE_INPUTS: [(&str, bool); 13] = [
        ("1..2", false),
        ("1 ..2", false),
        ("1.. 2", false),
        ("1 .. 2", false),
        ("12 .. 34", false),
        ("12.34 .. 56.78", false),
        ("12.34 .. 5", false),
        ("1 .. 5.6.7", false),
        ("1..=2", true),
        ("1 ..=2", true),
        ("1..= 2", true),
        ("1 ..= 2", true),
        ("1.2 ..= 2.3", true),
    ];

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
        SINGLE_INPUTS.into_iter().for_each(|input| {
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
        RANGE_ALSO_INCLUSIVE_INPUTS
            .into_iter()
            .for_each(|(input, _)| {
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
