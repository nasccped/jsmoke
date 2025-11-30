use regex::Regex;

/// A wrapper for the [`Regex`] object.
#[derive(Debug)]
pub struct RegexWrapper {
    /// The regex itself.
    pub re: Regex,
    /// The pattern that this regex holds. This data must be exposed
    /// since we can't get pattern from an already formed [`Regex`].
    pub pattern: String,
}

impl TryFrom<String> for RegexWrapper {
    type Error = regex::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for RegexWrapper {
    type Error = regex::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Regex::new(value).map(|re| Self {
            re,
            pattern: value.to_string(),
        })
    }
}
