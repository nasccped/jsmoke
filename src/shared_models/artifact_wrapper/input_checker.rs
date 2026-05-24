use super::{ARTIFACT_MAX_LENGTH, ARTIFACT_MIN_LENGTH, artifact_regex::ArtifactRegex};

/// Does the trivial input checking for [`super::ArtifactWrapper`].
pub struct InputChecker;

impl InputChecker {
    /// If the provided [`&str`] is empty.
    pub fn is_empty(input: &str) -> bool {
        input.is_empty()
    }

    /// If the provided [`&str`] contains an allowed length.
    pub fn is_allowed_length(input: &str) -> bool {
        (ARTIFACT_MIN_LENGTH..=ARTIFACT_MAX_LENGTH)
            .contains(&input.chars().filter(|&c| c != '-').count())
    }

    /// If the provided [`&str`] is an allowed [`super::ArtifactWrapper`] name pattern.
    pub fn is_allowed_pattern(input: &str) -> bool {
        ArtifactRegex::is_allowed(input)
    }

    /// If the provided [`&str`] is a fixable [`super::ArtifactWrapper`] name pattern.
    pub fn is_fixable_pattern(input: &str) -> bool {
        ArtifactRegex::is_fixable(input)
    }
}

#[cfg(test)]
mod tests {
    use super::InputChecker;

    #[test]
    fn is_empty() {
        assert!(InputChecker::is_empty(""))
    }

    #[test]
    fn allowed_artifacts() {
        ["mycoolapp", "with10number"].iter().for_each(|item| {
            assert!(
                InputChecker::is_allowed_pattern(item),
                "the provided artifact ({}) isn't allowed",
                item
            )
        });
    }

    #[test]
    fn fixable_artifacts() {
        ["my-cool-app", "with10-number"].iter().for_each(|item| {
            assert!(
                InputChecker::is_fixable_pattern(item),
                "the provided artifact ({}) isn't fixable",
                item
            )
        });
    }

    #[test]
    fn is_not_allowed_length() {
        ["a-very-long-artifact-name-that-is-obviously-invalid", "a"]
            .iter()
            .for_each(|item| {
                assert!(
                    !InputChecker::is_allowed_length(item),
                    "The provided artifact ({}) wasn't expected to constains an allowed lenght",
                    item
                );
            });
    }
}
