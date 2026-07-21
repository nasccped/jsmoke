use super::PROJECT_ARTIFACT_REGEX;
use super::ProjectArtifactParseError as PAPE;

/// Does the `str` check **before** building the actual [`super::ProjectArtifact`] item. This allow
/// the string to be checked before eventual fixes for _'code target'_.
#[derive(Debug)]
pub enum PreConversionChecker<'a> {
    /// When input refers to an empty string.
    Empty,

    /// When input refers to a compound name.
    Compound,

    /// When input refers to a non pattern matching on `PROJECT_ARTIFACT_REGEX`.
    NoMatch(&'a str),
}

impl<'a> PreConversionChecker<'a> {
    pub fn check(value: &'a str) -> Option<Self> {
        match value {
            "" => Some(Self::Empty),
            // whitespace char at middle means two (or more) words
            s if s.contains(|x: char| x.is_whitespace()) => Some(Self::Compound),
            s if !PROJECT_ARTIFACT_REGEX.is_match(s) => Some(Self::NoMatch(s)),
            _ => None,
        }
    }

    /// Does the same as [`PreConversionChecker::check`] but it forces the `Self` item returning.
    /// If result is [`None`], a panic message is displayed and the program crash.
    #[allow(unused)]
    fn force_build(value: &'a str) -> Self {
        Self::check(value).unwrap_or_else(|| {
            panic!(
                "'{}' returns `None` for `PreConversionChecker::check` function!",
                value
            )
        })
    }
}

impl<'a> From<PreConversionChecker<'a>> for PAPE<'a> {
    fn from(value: PreConversionChecker<'a>) -> Self {
        use PreConversionChecker as PCC;
        match value {
            PCC::Empty => PAPE::NoArtifactProvided,
            PCC::Compound => PAPE::CompoundName,
            PCC::NoMatch(s) => PAPE::InvalidPattern(s),
        }
    }
}

#[cfg(test)]
mod tests {
    // HACK: all functions should contains 'input.trim' since the caller function
    //       (ProjectArtifact::try_from) does this...
    use super::PAPE;
    use super::PreConversionChecker as PCC;

    #[test]
    fn empty() {
        ["", "   ", " \t ", "\n"].into_iter().for_each(|input| {
            let pcc = PCC::force_build(input.trim());

            if !matches!(pcc, PCC::Empty) {
                panic!(
                    "`{:?}` expected for '{}' input. Got {:?}",
                    PCC::Empty,
                    input,
                    pcc
                );
            }
        });

        assert!(matches!(PAPE::from(PCC::Empty), PAPE::NoArtifactProvided));
    }

    #[test]
    fn compound() {
        ["some name", "some \t name"].into_iter().for_each(|input| {
            let pcc = PCC::force_build(input.trim());

            if !matches!(pcc, PCC::Compound) {
                panic!(
                    "`{:?}` expected for '{}' input. Got {:?}",
                    PCC::Compound,
                    input,
                    pcc
                );
            }
        });

        assert!(matches!(PAPE::from(PCC::Compound), PAPE::CompoundName));
    }

    #[test]
    fn no_match() {
        [
            "-dash-prefix",
            "dash-sufix-",
            "0starts-with-number",
            "starts-1with-number",
            "non-valid?-char",
        ]
        .into_iter()
        .for_each(|input| {
            let pcc = PCC::force_build(input.trim());

            if !matches!(pcc, PCC::NoMatch(_)) {
                panic!(
                    "`PCC::NoMatch(_)` expected for '{}' input. Got {:?}",
                    input, pcc
                );
            }

            // NOTE: the conversion should be done at this scope to allow PCC's string owning.
            assert!(matches!(PAPE::from(pcc), PAPE::InvalidPattern(_)));
        });
    }
}
