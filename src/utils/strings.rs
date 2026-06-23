//! Provides [`StringUtils`] trait.
use std::ops::Deref;

/// Utility functions for any type that implements [`Deref`] trait for [`str`] type.
pub trait StringUtils {
    /// If the self string is quoted:
    /// ```
    /// let s: &str;
    /// s = "non quoted";
    /// assert!(!s.is_quoted());
    /// s = "'quoted'";
    /// assert!(s.is_quoted());
    /// ```
    ///
    /// Note that this function works for simple and double quotes, but only when the string starts
    /// **and** ends with the **same quote** kind.
    fn is_quoted(&self) -> bool;

    /// If the self string is a repetition of the pattern string:
    /// ```
    /// assert!("aaa".is_repetition_of("a"));
    /// assert!("zipozipo".is_repetition_of("zipo"));
    /// ```
    ///
    /// Note that this function matches the entire pattern (no overlap + entirely contained within
    /// the self string).
    ///
    /// In short, the code above fails:
    /// ```
    /// assert!("aaa".is_repetition_of("aa"));
    /// ```
    fn is_repetition_of(&self, pattern: &str) -> bool;
}

impl<T: Deref<Target = str>> StringUtils for T {
    fn is_quoted(&self) -> bool {
        (self.starts_with("'") && self.ends_with("'"))
            || (self.starts_with("\"") && self.ends_with("\""))
    }

    fn is_repetition_of(&self, pattern: &str) -> bool {
        if !self.len().is_multiple_of(pattern.len()) {
            return false;
        }
        let begins = (0..self.len()).step_by(pattern.len());
        let ends = (pattern.len()..self.len()).step_by(pattern.len());
        for (b, e) in begins.zip(ends) {
            if &self[b..e] != pattern {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::StringUtils;

    #[test]
    fn is_quoted() {
        [
            "'single quotes'",
            "\"double quotes\"",
            "''",
            "\"\"",
            "'   '",
            "\"   \"",
        ]
        .into_iter()
        .for_each(|input| assert!(input.is_quoted()));
        ["no quotes", "quotes at 'midle'"]
            .into_iter()
            .for_each(|input| assert!(!input.is_quoted()));
    }

    #[test]
    fn is_repetition_of() {
        assert!("aaa".is_repetition_of("a"));
        assert!("zipozipo".is_repetition_of("zipo"));
        assert!(!"aaa".is_repetition_of("aa"));
    }
}
