use crate::{
    utils::{
        Inner,
        verbose::{Verbose, VerboseWrapper},
        visuals::style::{ItemList, SuggestionStyle, TermStyle, WeakStyle},
    },
    verbose_wrapper,
};

/// Symbol for [`SingleVersionConstraint::StrictlyEquals`].
const STRICTLY_EQUALS_SYMBOL: &str = "=";

/// Symbols for [`SingleVersionConstraint::EqualsOrGreater`].
const EQUALS_OR_GREATER_SYMBOLS: [&str; 5] = ["^", "^=", ">", ">=", ""];

/// Symbol for [`RangeVersionConstraint::Inclusive`].
const INCLUSIVE_SYMBOL: &str = "..=";

/// Symbol for [`RangeVersionConstraint::Exclusive`].
const EXCLUSIVE_SYMBOL: &str = "..";

/// Constraint mode used at beiginning of single lock-version value. Example:
/// - `=` for [`SingleVersionConstraint::StrictlyEquals`]
/// - `^|^=|>|>=` for [`SingleVersionConstraint::EqualsOrGreater`]
#[derive(Default, Debug, PartialEq, Clone)]
pub enum SingleVersionConstraint {
    /// When the constraint means strictly equals.
    StrictlyEquals,
    /// When the constraint means equals or greater.
    #[default]
    EqualsOrGreater,
}

impl SingleVersionConstraint {
    /// Returns [`SingleVersionConstraint::StrictlyEquals`] valid input.
    pub fn get_strictly_equals_symbol() -> &'static str {
        STRICTLY_EQUALS_SYMBOL
    }

    /// Returns [`SingleVersionConstraint::EqualsOrGreater`] valid inputs (except the empty string
    /// input).
    pub fn get_equals_or_greater_symbols() -> Vec<&'static str> {
        EQUALS_OR_GREATER_SYMBOLS
            .into_iter()
            .take(EQUALS_OR_GREATER_SYMBOLS.len() - 1)
            .collect()
    }
}

/// Constraint mode used between a lock-version pair (range) value. Example:
/// - `..=` for [`RangeVersionConstraint::Inclusive`]
/// - `..` for [`RangeVersionConstraint::Exclusive`]
#[derive(Debug, PartialEq, Clone)]
pub enum RangeVersionConstraint {
    /// When the constraint means from `X` to `Z` with `Z` included.
    Inclusive,
    /// When the constraint means from `X` to `Z` with `Z` excluded.
    Exclusive,
}

impl RangeVersionConstraint {
    /// Returns the symbol for [`RangeVersionConstraint::Inclusive`].
    pub fn get_inclusive_symbol() -> &'static str {
        INCLUSIVE_SYMBOL
    }

    /// Returns the symbol for [`RangeVersionConstraint::Exclusive`].
    pub fn get_exclusive_symbol() -> &'static str {
        EXCLUSIVE_SYMBOL
    }
}

/// Error for [`SingleVersionConstraint`] and [`RangeVersionConstraint`] parsing fails.
#[derive(Debug)]
pub enum ConstraintError {
    /// When [`SingleVersionConstraint`] is being used.
    SingleVariant(Box<str>),
    /// When [`RangeVersionConstraint`] is being used.
    RangeVariant(Box<str>),
}

impl Inner<str> for ConstraintError {
    fn inner(&self) -> &str {
        match self {
            Self::SingleVariant(s) => s,
            Self::RangeVariant(s) => s,
        }
    }
}

impl TryFrom<&str> for SingleVersionConstraint {
    type Error = ConstraintError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.split_whitespace().collect::<String>().as_str() {
            x if x == STRICTLY_EQUALS_SYMBOL => Ok(Self::StrictlyEquals),
            x if EQUALS_OR_GREATER_SYMBOLS.contains(&x) => Ok(Self::EqualsOrGreater),
            other => Err(ConstraintError::SingleVariant(other.into())),
        }
    }
}

impl TryFrom<&str> for RangeVersionConstraint {
    type Error = ConstraintError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.split_whitespace().collect::<String>().as_str() {
            x if x == INCLUSIVE_SYMBOL => Ok(Self::Inclusive),
            x if x == EXCLUSIVE_SYMBOL => Ok(Self::Exclusive),
            other => Err(ConstraintError::RangeVariant(other.into())),
        }
    }
}

impl Verbose for ConstraintError {
    fn as_verbose(&self) -> VerboseWrapper {
        let mut vw = get_default_verbose_wrapper();
        vw.pushln("");
        match self {
            Self::SingleVariant(_) => {
                vw_push_single_suggestions(&mut vw);
            }
            Self::RangeVariant(_) => vw_push_range_suggestions(&mut vw),
        }
        vw
    }
}

/// Returns the main [`VerboseWrapper`] message for [`ConstraintError`].
fn get_default_verbose_wrapper() -> VerboseWrapper {
    verbose_wrapper!(
        "The {} is expected to be:" => "lock-version".term_style();
    )
}

/// Pushes suggestions for [`SingleVersionConstraint`] into [`VerboseWrapper`].
fn vw_push_single_suggestions(vw: &mut VerboseWrapper) {
    let strictly_equals = format!(
        "{} for strictly equals constraint",
        STRICTLY_EQUALS_SYMBOL.suggestion_style()
    );
    let equals_or_greater = format!(
        "{} {} for equals or greater constraint",
        EQUALS_OR_GREATER_SYMBOLS.join("|").suggestion_style(),
        "(or empty)".weak_style()
    );
    vw.pushln(strictly_equals.item_list_style());
    vw.pushln(equals_or_greater.item_list_style());
}

/// Pushes suggestions for [`RangeVersionConstraint`] into [`VerboseWrapper`].
fn vw_push_range_suggestions(vw: &mut VerboseWrapper) {
    let inclusive = format!(
        "{} (includes right-hand version as valid)",
        INCLUSIVE_SYMBOL.suggestion_style()
    );
    let not_inclusive = format!(
        "{} (excludes right-hand version as valid)",
        INCLUSIVE_SYMBOL.suggestion_style()
    );
    vw.pushln(inclusive.item_list_style());
    vw.pushln(not_inclusive.item_list_style());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single() {
        assert!(
            SingleVersionConstraint::try_from("=")
                .is_ok_and(|c| c == SingleVersionConstraint::StrictlyEquals)
        );
        ["^", "^=", ">", ">=", ""].into_iter().for_each(|inp| {
            let svc = SingleVersionConstraint::try_from(inp);
            assert!(svc.is_ok_and(|c| c == SingleVersionConstraint::EqualsOrGreater))
        });
    }

    #[test]
    fn range() {
        let inclusive = ["..=", ".. =", ". .=", " ..=", "..= ", ". . ="];
        inclusive.into_iter().for_each(|inp| {
            let rvc = RangeVersionConstraint::try_from(inp);
            assert!(rvc.is_ok_and(|c| c == RangeVersionConstraint::Inclusive));
        });
        let exclusive = ["..", ". .", ".. ", " .."];
        exclusive.into_iter().for_each(|inp| {
            let rvc = RangeVersionConstraint::try_from(inp);
            assert!(rvc.is_ok_and(|c| c == RangeVersionConstraint::Exclusive));
        });
    }
}
