use super::{
    PROJECT_ARTIFACT_MAXIMUM_LENGTH, PROJECT_ARTIFACT_MINIMUM_LENGTH, ProjectArtifact as PA,
    ProjectArtifactParseError as PAPE, error::ReservedState,
};
use crate::utils::Reserveds;

/// Does the [`PA`] testing after the string conversion. This struct helps to detect invalid state
/// **after** string fixing to a _'code version'_. A lot usefull to check reserved word using.
pub enum PostConversionChecker {
    /// When artifact name length is less than the minimum.
    MinLength(usize),

    /// When artifact name length is greater than the maximum.
    MaxLength(usize),

    /// When the artifact name is a reserved word.
    Reserved(ReservedState),
}

impl PostConversionChecker {
    /// Takes an [`PA`] reference and uses it to **may** build an error.
    ///
    /// This function requires a [`PA`] since some checks must be done **after** artifact name
    /// fixing.
    pub fn check(artifact: &PA<'_>) -> Option<Self> {
        let input = artifact.fixed_str();

        // optional state latter used on length matching fallback
        let state = Reserveds::is_reserved(input).then(|| {
            let input = input.to_string();

            if artifact.is_fixed() {
                ReservedState::Fixed(input)
            } else {
                ReservedState::NotFixed(input)
            }
        });

        match input.len() {
            len if len < PROJECT_ARTIFACT_MINIMUM_LENGTH => Some(Self::MinLength(len)),
            len if len > PROJECT_ARTIFACT_MAXIMUM_LENGTH => Some(Self::MaxLength(len)),
            _ => state.map(Self::Reserved),
        }
    }

    /// Converts the [`PAPE`] value into the designated [`PostConversionChecker`] item.
    ///
    /// # Panics
    ///
    /// Since [`PostConversionChecker`] contains a limited amount of variants, this function will
    /// panic if the [`PAPE`] variant refers to an unreachable one.
    #[allow(unused)]
    fn from_pape(pape: PAPE<'_>) -> Self {
        match pape {
            PAPE::ShortName(len) => Self::MinLength(len),
            PAPE::LongName(len) => Self::MaxLength(len),
            PAPE::Reserved(reserved) => Self::Reserved(reserved),
            other => unreachable!(
                "`from_pape` was called for an unreachable variant: {:?}",
                other
            ),
        }
    }
}

impl<'a> From<PostConversionChecker> for PAPE<'a> {
    fn from(value: PostConversionChecker) -> Self {
        use PostConversionChecker as PCC;
        match value {
            PCC::MinLength(len) => PAPE::ShortName(len),
            PCC::MaxLength(len) => PAPE::LongName(len),
            PCC::Reserved(reserved) => PAPE::Reserved(reserved),
        }
    }
}

#[cfg(test)]
mod tests {
    // NOTE: PostConversionChecker to ProjectArtifactParseError conversion testing isn't necessary
    //       at all.
    //
    //       The PostConversionChecker can only be get AFTER the ProjectArtifactParseError building
    //       (which is made using 'from' function). Basically, if the 'from_pape' function
    //       succeeds, the conversion was a success (independent of target direction).
    use super::{PA, PostConversionChecker as PCC};

    /// Test dedicated struct for [`PCC`] assertions.
    struct TestHelper;

    impl TestHelper {
        /// Create a new [`PCC`] item.
        ///
        /// # Panics
        ///
        /// This function can panics if provided input isn't a valid conversion to [`PCC`].
        #[allow(clippy::new_ret_no_self)]
        fn new(input: &str) -> PCC {
            let pape = match PA::try_from(input) {
                Ok(val) => panic!(
                    "'{}' input was expected to return error but it returned {:?}",
                    input, val
                ),
                Err(err) => err,
            };
            PCC::from_pape(pape)
        }
    }

    #[test]
    fn min_length() {
        ["a", "ab"].into_iter().for_each(|input| {
            assert!(matches!(TestHelper::new(input), PCC::MinLength(x) if x == input.len()))
        });
    }

    #[test]
    fn max_length() {
        ["abcdefghijklmnopqrstuvwxyz0123456789"]
            .into_iter()
            .for_each(|input| {
                assert!(matches!(TestHelper::new(input), PCC::MaxLength(x) if x == input.len()))
            });
    }

    #[test]
    fn reserved() {
        ["Integer", "java", "float", "interface"]
            .into_iter()
            .for_each(|input| assert!(matches!(TestHelper::new(input), PCC::Reserved(_))));
    }
}
