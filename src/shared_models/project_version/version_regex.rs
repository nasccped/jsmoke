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
    /// - `=<pattern>`  => strictly equals to `<pattern>`
    /// - `>=<pattern>` => equals or greater than `<pattern>`
    Single,
    /// Means `range version` kind constraints, like:
    /// - `{left}..{right}`  => any version that's from `{left}` until `{N}` when `N` < `{right}`
    ///   (exclusive)
    /// - `{left}..={right}` =>  any version that's from `{left}` until `{N}` when `N` <= `{right}`
    ///   (inclusive)
    Range,
}

/// [`Regex`] struct specific for constraint kind matching. It can be turned into a valid [`Regex`]
/// string since it implements [`Display`]. The [`String`] output is based on the inner
/// [`ConstraintKind`] variant.
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

// NOTE: allow `const` when `static` was expected since Rust doesn't allow `static` items at `impl`
// blocks.
#[allow(clippy::declare_interior_mutable_const)]
impl ConstraintRegex {
    /// The group name for [`ConstraintRegex`].
    const GROUP_NAME: &str = "constraint";

    /// [`Regex`] string for single kind constraints.
    const SINGLE_KIND_REGEX_STR: LazyLock<String> = LazyLock::new(|| {
        use super::version::SingleKind;
        let signs = Self::into_sign_string([
            SingleKind::STRICTLY_EQUALS_SIGN,
            SingleKind::EQUALS_OR_GREATER_SIGN,
        ]);
        format!(r#"(?<{}>{})?"#, Self::GROUP_NAME, signs)
    });

    /// [`Regex`] string for range kind constraints.
    const RANGE_KIND_REGEX_STR: LazyLock<String> = LazyLock::new(|| {
        use super::version::RangeKind;
        let signs = Self::into_sign_string([RangeKind::INCLUSIVE_SIGN, RangeKind::EXCLUSIVE_SIGN]);
        format!(r#"(?<{}>{})"#, Self::GROUP_NAME, signs)
    });

    /// Converts the provided items into a single regex string: `['a', 'b', 'c', ...]` =>
    /// `"a|b|c|..."`.
    fn into_sign_string<T: IntoIterator<Item = &'static str>>(items: T) -> String {
        items
            .into_iter()
            .map(|item| {
                item.chars()
                    .map(|c| {
                        if c == '.' {
                            format!("\\{}", c)
                        } else {
                            c.to_string()
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("|")
    }

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

/// [`Regex`] struct specific for version kind matching. It can be turned into a valid [`Regex`]
/// string since it implements [`Display`]. The [`String`] output is based on the inner
/// [`VersionKind`] variant.
///
/// Note: Using `Private` naming prefix to avoid conflict with [`VersionRegex`] which is public.
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

// NOTE: allow `const` when `static` was expected since Rust doesn't allow `static` items at `impl`
// blocks.
#[allow(clippy::declare_interior_mutable_const)]
impl PrivateVersionRegex {
    /// Group name when the used kind is [`VersionKind::Single`].
    const SINGLE_GROUP_NAME: &str = "version";

    /// Group name when the used kind is [`VersionKind::RangeLeft`].
    const LEFT_GROUP_NAME: &str = "leftversion";

    /// Group name when the used kind is [`VersionKind::RangeRight`].
    const RIGHT_GROUP_NAME: &str = "rightversion";

    /// Group name for [`VersionKind`] independent (targeting `major` group:
    /// `<pattern<major><rest...>`).
    const MAJOR_GROUP_NAME: &str = "major";

    /// Group name for [`VersionKind`] independent (targeting `minor` group:
    /// `<pattern<...><minor><rest...>`).
    const MINOR_GROUP_NAME: &str = "minor";

    /// Group name for [`VersionKind`] independent (targeting `patch` group:
    /// `<pattern<...><patch>`).
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

/// [`Regex`] for `kind` independent match/grouping (excludes constraint). Used to build
/// [`VersionCaptures`].
static ONLY_VERSION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let version = PrivateVersionRegex::new(VersionKind::Single);
    Regex::new(&format!("^{}$", version)).surely_unwrap()
});

/// Common trait for constraint unwrapping (polymorphism between [`SingleVersionCaptures`] and
/// [`RangeVersionCaptures`]).
pub trait ConstraintGetter<'a>: Deref<Target = Captures<'a>> {
    /// Get the constraint string from the [`Captures`] wrapper. It returns an [`Option`] since
    /// [`SingleVersionCaptures`] can hold (or not) a `constraint` specifier.
    fn get_constraint(&'a self) -> Option<&'a str> {
        self.name(ConstraintRegex::GROUP_NAME).map(|m| m.as_str())
    }
}

impl<'a> ConstraintGetter<'a> for SingleVersionCaptures<'a> {}
impl<'a> ConstraintGetter<'a> for RangeVersionCaptures<'a> {}

/// [`Captures`] wrapper for `single version` kind patterns.
#[derive(Debug)]
pub struct SingleVersionCaptures<'a>(Captures<'a>);

impl<'a> Deref for SingleVersionCaptures<'a> {
    type Target = Captures<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> SingleVersionCaptures<'a> {
    /// Returns the [`VersionCaptures`] based on the `self` inner captures.
    pub fn get_version(&self) -> VersionCaptures<'a> {
        // all the code bellow can be surely unwrapped since the `self` captures is built under a
        // surely matching regex.
        let s = self
            .name(PrivateVersionRegex::SINGLE_GROUP_NAME)
            .map(|m| m.as_str())
            .surely_unwrap();
        VersionCaptures(ONLY_VERSION_REGEX.captures(s).surely_unwrap())
    }
}

/// [`Captures`] wrapper for `range version` kind patterns.
#[derive(Debug)]
pub struct RangeVersionCaptures<'a>(Captures<'a>);

impl<'a> Deref for RangeVersionCaptures<'a> {
    type Target = Captures<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> RangeVersionCaptures<'a> {
    /// Returns the left side [`VersionCaptures`].
    fn get_left(&self) -> VersionCaptures<'a> {
        let s = self
            .name(PrivateVersionRegex::LEFT_GROUP_NAME)
            .map(|m| m.as_str())
            .surely_unwrap();
        VersionCaptures(ONLY_VERSION_REGEX.captures(s).surely_unwrap())
    }

    /// Returns the right side [`VersionCaptures`].
    fn get_right(&self) -> VersionCaptures<'a> {
        let s = self
            .name(PrivateVersionRegex::RIGHT_GROUP_NAME)
            .map(|m| m.as_str())
            .surely_unwrap();
        VersionCaptures(ONLY_VERSION_REGEX.captures(s).surely_unwrap())
    }
}

/// Generic captures for version matching. Usefull for kind independent matching and group (works
/// for [`VersionKind::Single`], [`VersionKind::RangeLeft`] and [`VersionKind::RangeRight`]) and
/// `constraint` excluding.
#[derive(Debug)]
pub struct VersionCaptures<'a>(Captures<'a>);

impl<'a> Deref for VersionCaptures<'a> {
    type Target = Captures<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> VersionCaptures<'a> {
    /// Returns the major field from a version pattern:
    /// - `1.2.3` => `1`
    /// - `12.43` => `12`
    /// - `...`
    ///
    /// This function will panic only if there's no `major` group matching (which isn't expected
    /// since it's build by version regex basic requirements).
    pub fn get_major(&self) -> &str {
        self.name(PrivateVersionRegex::MAJOR_GROUP_NAME)
            .map(|m| m.as_str())
            .surely_unwrap()
    }

    /// Returns the minor field from a version pattern:
    /// - `1.2.3` => `Some(2)`
    /// - `12.34.56` => `Some(34)`
    /// - `1` => `None`
    pub fn get_minor(&self) -> Option<&str> {
        self.name(PrivateVersionRegex::MINOR_GROUP_NAME)
            .map(|m| m.as_str())
    }

    /// Returns the patch field from a version pattern:
    /// - `1.2.3` => `Some(3)`
    /// - `12.34.56` => `Some(56)`
    /// - `1` => `None`
    /// - `1.2` => `None`
    pub fn get_patch(&self) -> Option<&str> {
        self.name(PrivateVersionRegex::PATCH_GROUP_NAME)
            .map(|m| m.as_str())
    }
}

/// [`Regex`] utilities for [`super::ProjectVersion`].
pub struct VersionRegex;

impl VersionRegex {
    /// Returns a [`SingleVersionCaptures`] if it matches with [`ConstraintKind::Single`] +
    /// [`VersionKind::Single`] pattern rules, otherwise returns [`None`].
    pub fn may_single<'a>(haystack: &'a str) -> Option<SingleVersionCaptures<'a>> {
        SINGLE_KIND_REGEX
            .captures(haystack)
            .map(SingleVersionCaptures)
    }

    /// Returns a [`RangeVersionCaptures`] if it matches with [`VersionKind::RangeLeft`] +
    /// ([`ConstraintKind::Range`]) + [`VersionKind::RangeRight`] pattern rules, otherwise returns
    /// [`None`].
    pub fn may_range<'a>(haystack: &'a str) -> Option<RangeVersionCaptures<'a>> {
        RANGE_KIND_REGEX
            .captures(haystack)
            .map(RangeVersionCaptures)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::any::{Any, type_name};

    /// Panics if provided value isn't [`Some`] variant.
    fn sure_some<T: Any>(input: impl Display, value: Option<T>) -> T {
        value.unwrap_or_else(|| {
            panic!(
                "'{}' was expected to return `Some({})`",
                input,
                type_name::<T>()
            )
        })
    }

    #[test]
    fn constraint_assertion() {
        let none_assertion = |input: &'static str| {
            let captures = sure_some(input, VersionRegex::may_single(input));
            assert_eq!(captures.get_constraint(), None);
        };
        ["1", "1.2", "1.2.3", " 1", " 1.2", " 1.2.3"]
            .into_iter()
            .for_each(none_assertion);
        let strictly_equals_assertion = |input: &'static str| {
            let captures = sure_some(input, VersionRegex::may_single(input));
            assert_eq!(captures.get_constraint(), Some("="));
        };
        ["=1", "=1.2", "=1.2.3", "= 1", "= 2.3", "= 2.3.4"]
            .into_iter()
            .for_each(strictly_equals_assertion);
        let equals_or_greater_assertion = |input: &'static str| {
            let captures = sure_some(input, VersionRegex::may_single(input));
            assert_eq!(captures.get_constraint(), Some(">="));
        };
        [">=1", ">=1.2", ">=1.2.3", ">= 1", ">= 2.3", ">= 2.3.4"]
            .into_iter()
            .for_each(equals_or_greater_assertion);
        let exclusive_assertion = |input: &'static str| {
            let captures = sure_some(input, VersionRegex::may_range(input));
            assert_eq!(captures.get_constraint(), Some(".."));
        };
        [
            "1..2",
            "1 ..2",
            "1.. 2",
            "1 .. 2",
            "1.2..3.4",
            "1.2 ..3.4",
            "1.2.. 3.4",
            "1.2 .. 3.4",
        ]
        .into_iter()
        .for_each(exclusive_assertion);
        let inclusive_assertion = |input: &'static str| {
            let captures = sure_some(input, VersionRegex::may_range(input));
            assert_eq!(captures.get_constraint(), Some("..="));
        };
        [
            "1..=2",
            "1 ..=2",
            "1..= 2",
            "1 ..= 2",
            "1.2..=3.4",
            "1.2 ..=3.4",
            "1.2..= 3.4",
            "1.2 ..= 3.4",
        ]
        .into_iter()
        .for_each(inclusive_assertion);
    }

    #[test]
    fn single_assertion() {
        [
            ("1", ("1", None, None)),
            ("1.2", ("1", Some("2"), None)),
            ("1.2.3", ("1", Some("2"), Some("3"))),
            ("12.34.56", ("12", Some("34"), Some("56"))),
            ("1.23.456", ("1", Some("23"), Some("456"))),
            ("=1", ("1", None, None)),
            ("=1.2", ("1", Some("2"), None)),
            ("=1.2.3", ("1", Some("2"), Some("3"))),
            ("=12.34.56", ("12", Some("34"), Some("56"))),
            ("=1.23.456", ("1", Some("23"), Some("456"))),
            ("= 1", ("1", None, None)),
            (">=1.2.3", ("1", Some("2"), Some("3"))),
            (">= 1.2.3", ("1", Some("2"), Some("3"))),
        ]
        .into_iter()
        .for_each(|(input, (exp_major, exp_minor, exp_patch))| {
            let single = VersionRegex::may_single(input).surely_unwrap();
            if let Some(range) = VersionRegex::may_range(input) {
                panic!(
                    "`None` was expected for '{}' input but got `{:?}`",
                    input, range
                );
            }
            let assert_major = |version: &VersionCaptures<'_>| {
                assert_eq!(
                    version.get_major(),
                    exp_major,
                    "failed assertion for '{}'",
                    input
                )
            };
            let assert_minor = |version: &VersionCaptures<'_>| {
                assert_eq!(
                    version.get_minor(),
                    exp_minor,
                    "failed assertion for '{}'",
                    input
                )
            };
            let assert_patch = |version: &VersionCaptures<'_>| {
                assert_eq!(
                    version.get_patch(),
                    exp_patch,
                    "failed assertion for '{}'",
                    input
                )
            };
            let version = single.get_version();
            assert_major(&version);
            assert_minor(&version);
            assert_patch(&version);
        });
    }

    #[test]
    fn range_assertion() {
        [
            ("1", "2"),
            ("1 ", "2"),
            ("1", " 2"),
            ("1 ", " 2"),
            ("12 ", " 34"),
            ("12.34 ", " 56.78"),
            ("12.34 ", " 5"),
            ("1 ", " 5.6.7"),
        ]
        .into_iter()
        .for_each(|(left, right)| {
            let do_assertions = |captures: &RangeVersionCaptures<'_>| {
                let (left_cap, right_cap) = (captures.get_left(), captures.get_right());
                assert_eq!(left_cap.get_match().as_str(), left.trim());
                assert_eq!(right_cap.get_match().as_str(), right.trim());
            };
            let exclusive_input = format!("{}..{}", left, right);
            let captures = VersionRegex::may_range(&exclusive_input).surely_unwrap();
            do_assertions(&captures);
            let inclusive_input = format!("{}..={}", left, right);
            let captures = VersionRegex::may_range(&inclusive_input).surely_unwrap();
            do_assertions(&captures);
        });
    }
}
