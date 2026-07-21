use crate::utils::{
    Verbose,
    styler::{ListStyle, Styler, StylerOutput},
};
use std::borrow::Cow;

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

impl<'a> Verbose for ProjectArtifactParseError<'a> {
    fn get_verbose_message(&self) -> Option<Cow<'_, str>> {
        let message = match self {
            Self::CompoundName | Self::NoArtifactProvided => format!(
                "Consider passing a simple name such as {}. \n\
                You can also use {} flag to skip artifact setting.",
                "my-app".suggestion_style(),
                "--no-artifact".command_style()
            ),
            Self::InvalidPattern(_) => {
                let mut conditions = ListStyle::new_unordered();
                conditions.set_left_padding(1);
                conditions.push_items([
                    "starts with a letter",
                    "contain dashes/capitalized",
                    "contain numbers",
                ]);
                let styled_out: StylerOutput<String> = conditions.into();
                format!(
                    "A valid artifact pattern can be expressed\n\
                    by the {} regex. In short:\n\
                    {}",
                    super::PROJECT_ARTIFACT_REGEX.as_str().term_style(),
                    styled_out
                )
            }
            Self::Reserved(res) => {
                let styled: StylerOutput<String> = {
                    let mut l = ListStyle::new_ordered();
                    l.set_left_padding(1);
                    l.push_items(["keywords", "common types", "well know names", "etc"]);
                    l.into()
                };
                let mut message = format!(
                    "To avoid {} or management undefined behavior, java\n\
                    environment reserved words aren't allowed. This refers to:\n\
                    {}",
                    "(run/comp)time".term_style(),
                    styled
                );
                if matches!(res, ReservedState::Fixed(_)) {
                    let note = format!(
                        "this check is done after artifact fixing, so\n\
                        the {} can be fixed to {} and turn into a reserved\n\
                        word.",
                        "ja-va".special_style(),
                        "java".special_style()
                    )
                    .note_style();
                    message.push_str(format!("\n\n{}", note).as_str())
                }
                message
            }
            Self::ShortName(_) => format!(
                "The project artifact must be at least {} char long!",
                super::PROJECT_ARTIFACT_MINIMUM_LENGTH.number_style(),
            ),
            Self::LongName(_) => format!(
                "The project artifact must contains less than {} chars!",
                (super::PROJECT_ARTIFACT_MAXIMUM_LENGTH + 1).number_style()
            ),
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
