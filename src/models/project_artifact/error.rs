use super::{
    PROJECT_ARTIFACT_MAXIMUM_LENGTH as MAXIMUM_LENGTH,
    PROJECT_ARTIFACT_MINIMUM_LENGTH as MINIMUM_LENGTH, PROJECT_ARTIFACT_REGEX as REGEX,
};
use crate::utils::{
    Verbose,
    styler::{ListStyle, Styler, StylerOutput},
};
use std::{borrow::Cow, sync::LazyLock};

/// Verbose message when refering to [`ProjectArtifactParseError::CompoundName`] or
/// [`ProjectArtifactParseError::NoArtifactProvided`] variants.
static COMPOUND_OR_EMPTY_VERBOSE: LazyLock<String> = LazyLock::new(|| {
    format!(
        "\
        Consider passing a simple name such as {}. \n\
        You can also use {} flag to skip artifact setting.",
        "my-app".suggestion_style(),
        "--no-artifact".command_style()
    )
});

/// Verbose message when refering to [`ProjectArtifactParseError::InvalidPattern`] variant.
static INVALID_PATTERN_VERBOSE: LazyLock<String> = LazyLock::new(|| {
    let mut conditions = ListStyle::new_unordered();

    conditions.set_left_padding(1);
    conditions.push_items([
        "should start and end with a letter",
        "(can) contain dashes / capitalized",
        "(can) contain numbers",
    ]);

    let styled_out: StylerOutput<String> = conditions.into();

    format!(
        "\
        A valid artifact pattern can be expressed\n\
        by the {} regex. In short:\n\
        {}",
        REGEX.as_str().term_style(),
        styled_out
    )
});

/// Function that returns the verbose message when refering to
/// [`ProjectArtifactParseError::InvalidPattern`] variant.
///
/// A function should be used (instead of [`LazyLock`]) since this verbose message expects a
/// conditional param which changes the message output.
///
/// This function is capitalized to follow the look and fell of
/// [`ProjectArtifactParseError::get_verbose_message`] function body.
#[allow(non_snake_case)]
fn RESERVED_VERBOSE(is_fixed: bool) -> String {
    let styled: StylerOutput<String> = {
        let mut l = ListStyle::new_ordered();
        l.set_left_padding(1);
        l.push_items(["keywords", "common types", "well know names", "etc"]);
        l.into()
    };
    let mut message = format!(
        "\
        To avoid {} undefined behavior, java environment\n\
        reserved words aren't allowed. This refers to:\n\
        {}",
        "(run/comp)time".term_style(),
        styled
    );
    if is_fixed {
        let note = format!(
            "\
            this check is done {} artifact fixing, so\n\
            the {} can be fixed to {} and turn into a reserved\n\
            word.",
            "after".strong_style(),
            "ja-va".special_style(),
            "java".special_style()
        )
        .note_style();
        message.push_str(format!("\n\n{}", note).as_str())
    }
    message
}

/// Verbose message when refering to [`ProjectArtifactParseError::ShortName`] variant.
static SHORT_NAME_VERBOSE: LazyLock<String> = LazyLock::new(|| {
    format!(
        "The project artifact must be at least {} chars long!",
        MINIMUM_LENGTH.number_style(),
    )
});

/// Verbose message when refering to [`ProjectArtifactParseError::LongName`] variant.
static LONG_NAME_VERBOSE: LazyLock<String> = LazyLock::new(|| {
    format!(
        "The project artifact must contains less than {} chars!",
        (MAXIMUM_LENGTH + 1).number_style()
    )
});

/// Possible fails when trying to parse a [`super::ProjectArtifact`] from a string slice.
#[derive(thiserror::Error, Debug)]
pub enum ProjectArtifactParseError<'a> {
    /// When a single name is expected but a compound one was passed.
    #[error("the project artifact carries a compound name")]
    CompoundName,

    /// When an empty/whitespace string is passed as artifact name.
    #[error("the project artifact was expected but not provided")]
    NoArtifactProvided,

    /// When the given artifact name doesn't follows the [`super::PROJECT_ARTIFACT_REGEX`] rules.
    #[error("given artifact refers to an invalid pattern ({})", .0)]
    InvalidPattern(&'a str),

    /// When the provided artifact name refers to a reserved word.
    #[error(transparent)]
    Reserved(ReservedState),

    /// When the name is too short (less than [`super::PROJECT_ARTIFACT_MINIMUM_LENGTH`] chars
    /// length).
    #[error("the project artifact is too short ({} chars long)", .0)]
    ShortName(usize),

    /// When the name is too long (greater than [`super::PROJECT_ARTIFACT_MINIMUM_LENGTH`] chars
    /// length).
    #[error("the project artifact is too long ({} chars long)", .0)]
    LongName(usize),
}

/// Refers to the [`ProjectArtifactParseError::Reserved`] variant inner state.
#[derive(thiserror::Error, Debug)]
pub enum ReservedState {
    /// Means that the artifact name was valid but it refers to a reserved word, like when passing
    /// `integer` or `class` as artifact name.
    #[error("the project artifact is a reserved name ({})", .0)]
    NotFixed(String),

    /// Means that the artifact name was fixed and now, it refers to a reserved word, like as:
    /// - `inter-face` turns into `interface`
    /// - `pri-va-te` turns into `private`
    /// - _and so on..._
    #[error("the fixed project artifact results in a reserved name ({})", .0)]
    Fixed(String),
}

impl ReservedState {
    /// Returns if the `self` item refers to the [`ReservedState::Fixed`] variant.
    #[inline]
    fn is_fixed(&self) -> bool {
        matches!(self, Self::Fixed(_))
    }
}

impl<'a> Verbose for ProjectArtifactParseError<'a> {
    fn get_verbose_message(&self) -> Option<Cow<'_, str>> {
        let message = match self {
            Self::CompoundName | Self::NoArtifactProvided => COMPOUND_OR_EMPTY_VERBOSE.clone(),
            Self::InvalidPattern(_) => INVALID_PATTERN_VERBOSE.clone(),
            Self::Reserved(res) => RESERVED_VERBOSE(res.is_fixed()),
            Self::ShortName(_) => SHORT_NAME_VERBOSE.clone(),
            Self::LongName(_) => LONG_NAME_VERBOSE.clone(),
        };
        Some(Cow::Owned(message))
    }
}

#[cfg(test)]
mod tests {
    // NOTE: There's no ProjectArtifactParseError testing anymore since all test stuff is done at
    //       auxiliar modules (artifact.rs and conversions.rs).
    //
    //       This commend purpose is to remind future refactors!
}
