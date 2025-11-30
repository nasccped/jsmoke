use super::{
    how_much_of_pattern::{HowMuchOfPattern, Sealed},
    regex_wrapper::RegexWrapper,
};

/// A struct than can build regex constraint (repetition, leading
/// spaces...) into a [`RegexWrapper`].
#[derive(Clone, Debug, Default)]
pub struct RegexBuilder {
    /// The pattern to be applied.
    pattern: String,
    /// How many times the [`Self::pattern`] will be applied
    /// (according to regex pattern rules).
    times: Option<HowMuchOfPattern>,
    /// If this pattern allows leading whitespaces.
    leading_spaces: bool,
}

impl RegexBuilder {
    /// Creates a new instance of [`RegexBuilder`]. Note that this
    /// function requires a value to be converted into a string but
    /// the [`Self::times`] and [`Self::leading_spaces`] fields
    /// remains as default values ([`None`] and `bool`). Use other
    /// builder functions to change this instance hierarchically.
    pub fn new(pattern: impl ToString) -> Self {
        Self {
            pattern: pattern.to_string(),
            ..Default::default()
        }
    }

    /// Sets a new [`Self::times`] field using any type that
    /// implements [`RangeBounds`] to usize (range declarations are
    /// prefered (`x..y`, `..=z`, ...)).
    ///
    /// # Important
    ///
    /// This function calls other private objects and methods. Ensure
    /// passing valid ranges
    ///
    /// # Panic example
    ///
    /// The code bellow will fail 'cause it ends with **`0`
    /// excluded**. This means:
    ///
    /// 1. take the last value
    /// 2. subtract by 1
    /// 3. continue...
    ///
    /// This will fail because subtracting 0 with 1 results in _usize
    /// underflow_ (usize values can only be 0 or positive).
    ///
    /// ```compile_fail
    /// # let re_builder;
    /// re_builder.with_times(..0);
    /// ```
    pub fn with_times(&mut self, times: impl Sealed<usize>) -> &mut Self {
        let hmop = HowMuchOfPattern::from(times);
        self.times = Some(hmop);
        self
    }

    /// Clear the [`Self::times`] field (sets to [`None`]).
    pub fn clear_times(&mut self) -> &mut Self {
        self.times = None;
        self
    }

    /// If leading spaces should me applied.
    pub fn with_leading_spaces(&mut self, yes: bool) -> &mut Self {
        self.leading_spaces = yes;
        self
    }

    /// Merges another pattern in the self [`RegexBuilder`] object.
    /// The `oher` param must be an unwrapped [`RegexWrapper`], since
    /// parse error handling was already done.
    ///
    /// Note that the `other::leading_spaces` and times are still
    /// held by the pattern string.
    pub fn merge(&mut self, other: RegexWrapper) -> &mut Self {
        self.pattern
            .push_str(format!("({})", other.pattern).as_str());
        self
    }

    /// Tries to build the [`RegexWrapper`] from the self
    /// [`RegexBuilder`] object. If parsing fails, returns a
    /// [`regex::Error`] value.
    pub fn try_build(self) -> Result<RegexWrapper, regex::Error> {
        let pattern = format!(
            "{}{}{}",
            if self.leading_spaces { r"\s*" } else { "" },
            if let Some(t) = self.times {
                format!("({}){}", self.pattern, t.repr())
            } else {
                self.pattern
            },
            if self.leading_spaces { r"\s*" } else { "" },
        );
        RegexWrapper::try_from(pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_PATTERNS: [&str; 25] = [
        "abc",
        r#"hello_world"#,
        r#"foo123"#,
        r#"[a-z]"#,
        r#"[A-Z0-9_]+"#,
        r#"[^0-9]+"#,
        r#"^hello"#,
        r#"world$"#,
        r#"^start.*end$"#,
        r#"^hello"#,
        r#"world$"#,
        r#"^start.*end$"#,
        r#"a*"#,
        r#"b+"#,
        r#"c{2,5}"#,
        r#"ab{3}c+"#,
        r#"(abc)"#,
        r#"(foo|bar|baz)"#,
        r#"([a-z]+)([0-9]*)"#,
        r#"\d+"#,
        r#"\s*foo\s*"#,
        r#"\w{3}\."#,
        r#"^(https?)://[A-Za-z0-9.-]+(:[0-9]+)?(/.*)?$"#,
        r#"^[a-f0-9]{32}$"#,
        r#"(?i)hello"#,
    ];

    const INVALID_PATTERNS: [&str; 9] = [
        r#"[abc"#,
        r#"([a-z]+"#,
        r#"*abc"#,
        r#"+foo"#,
        r#"?bar"#,
        r#"\d("#,
        r#"foo\"#,
        r#"a{5,3}"#,
        r#"a{,3}"#,
    ];

    #[test]
    fn try_build() {
        VALID_PATTERNS.into_iter().for_each(|pat| {
            if let Err(e) = RegexBuilder::new(pat).try_build() {
                panic!("this was expected to be ok: {}", e);
            }
        });
        INVALID_PATTERNS.into_iter().for_each(|pat| {
            if let Ok(re) = RegexBuilder::new(pat).try_build() {
                panic!("this was expected to be error: {:?}", re);
            }
        });
    }

    #[test]
    fn merge() {
        let inner = RegexBuilder::new(r"\s*pattern\s*")
            .try_build()
            .expect("this was expected to be ok");
        let mut outer = RegexBuilder::new(r"outer");
        outer.merge(inner);
        let end = outer.try_build().expect("this was expected to be ok");
        assert_eq!(end.pattern, r"outer(\s*pattern\s*)");
    }
}
