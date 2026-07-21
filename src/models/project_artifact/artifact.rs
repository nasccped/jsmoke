use super::{
    ProjectArtifactParseError as PAPE, post_conversion_checker::PostConversionChecker as PostCC,
    pre_conversion_checker::PreConversionChecker as PreCC,
};

/// Project artifact wrapper.
#[derive(Debug)]
pub enum ProjectArtifact<'a> {
    /// When the provided string is an already valid artifact (for _'code version'_).
    Valid(&'a str),

    /// When the provided string must be fixed to a _'code version'_:
    /// - `my-art` to `myart`
    /// - `UpperCased` to `uppercased`
    /// - etc
    Fixed {
        /// Original string.
        original: &'a str,

        /// Fixed string.
        fixed: String,
    },
}

impl<'a> TryFrom<&'a str> for ProjectArtifact<'a> {
    type Error = PAPE<'a>;

    fn try_from(mut value: &'a str) -> Result<Self, Self::Error> {
        value = value.trim();

        if let Some(err) = PreCC::check(value) {
            return Err(PAPE::from(err));
        }

        let result = Self::new(value);

        match PostCC::check(&result) {
            None => Ok(result),
            Some(err) => Err(PAPE::from(err)),
        }
    }
}

impl<'a> ProjectArtifact<'a> {
    /// Takes a `str` slice and build a [`ProjectArtifact`] over it. This function already does the
    /// `valid` / `fixed` stuff, but it **doesn't** do any checking. It should be done at the
    /// **caller's function, actually** (I'm talking about [`ProjectArtifact::try_from`], [`PreCC`]
    /// and [`PostCC`]).
    fn new(value: &'a str) -> Self {
        // result initial state (zero length string)
        let mut result = Self::Valid(&value[0..0]);

        // for each position / char
        for (i, mut c) in value.char_indices() {
            // updates char / loop flow if necessary
            c = match c {
                '-' | '_' => {
                    result.to_fixed();
                    continue;
                }

                c if c.is_ascii_uppercase() => {
                    result.to_fixed();
                    c.to_ascii_lowercase()
                }

                c => c,
            };

            // add new data to result
            match &mut result {
                ProjectArtifact::Valid(s) => *s = &value[0..=i],
                ProjectArtifact::Fixed { original, fixed } => {
                    *original = &value[0..=i];
                    fixed.push(c);
                }
            }
        }

        result
    }

    /// Returns if the self item refers to [`ProjectArtifact::Fixed`] variant.
    #[inline]
    pub fn is_fixed(&self) -> bool {
        matches!(self, Self::Fixed { .. })
    }

    /// Converts a [`ProjectArtifact::Valid`] variant to [`ProjectArtifact::Fixed`], working
    /// similar to [`std::borrow::Cow::to_mut`] function when the data is expected to be owned.
    ///
    /// Note that this function copies the valid variant inner field to fixed fields (or does
    /// nothing if it's already fixed). Other checks / handling should be done manually at caller's
    /// function.
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    fn to_fixed(&mut self) -> &mut Self {
        match self {
            Self::Valid(s) => {
                *self = Self::Fixed {
                    original: s,
                    fixed: s.to_string(),
                }
            }
            Self::Fixed { .. } => {}
        }
        self
    }

    /// Returns the inner artifact name as fixed string (code valid).
    #[inline]
    pub fn fixed_str(&self) -> &str {
        match self {
            Self::Valid(s) => s,
            Self::Fixed { fixed, .. } => fixed,
        }
    }

    /// Returns the inner artifact name as it original string (as passed).
    #[inline]
    pub fn original_str(&self) -> &str {
        match self {
            Self::Valid(s) => s,
            Self::Fixed { original, .. } => original,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProjectArtifact as PA;

    /// Test dedicated type (for [`PA`]).
    struct PATest<'a> {
        input: &'a str,
        original: &'a str,
        fixed: &'a str,
    }

    impl<'a> PATest<'a> {
        /// Creates a new [`PATest`] item. It takes:
        /// - `input`, value used to build the [`PA`] object
        /// - `original`, value expected as [`PA::original_str`] output
        /// - `fixed`, value expected as [`PA::fixed_str`] output
        fn new(input: &'a str, original: &'a str, fixed: &'a str) -> Self {
            Self {
                input,
                original,
                fixed,
            }
        }

        /// Generates the [`PA`] item and test `str_func` outputs.
        ///
        /// # Panics
        ///
        /// This function expects `self.input` to be an valid [`PA`] input. The [`PA`] is
        /// built from [`TryFrom::try_from`] function and can panic if [`Err`] variant returned.
        fn assert_all(&self) {
            let art = PA::try_from(self.input)
                .unwrap_or_else(|err| panic!("`Ok` variant expected but got {:?}", err));

            assert_eq!(
                self.original,
                art.original_str(),
                "`original` assertion failed for '{}' input",
                self.input
            );

            assert_eq!(
                self.fixed,
                art.fixed_str(),
                "`fixed` assertion failed for '{}' input",
                self.input
            );
        }
    }

    #[test]
    fn artifact_names() {
        [
            ("normalname", "normalname", "normalname"),
            ("my-project", "my-project", "myproject"),
            ("MyProject", "MyProject", "myproject"),
            ("Super2-Dub", "Super2-Dub", "super2dub"),
        ]
        .into_iter()
        .for_each(|(input, original, fixed)| {
            PATest::new(input, original, fixed).assert_all();
        });
    }
}
