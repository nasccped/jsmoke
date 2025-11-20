//! # AutoTrim module
//!
//! Provides auto trim feature for &[`str`] and [`String`] types.
//!
//! I could just use `some_string.trim()` from [`std`] lib, but this
//! approach enforces modularization and standardization over the
//! entire workspace.

/// Automatically trims the given value (works like [`str::trim`]'s
/// function).
pub trait AutoTrim<'a> {
    /// Output type to be returned.
    type Output: ToOwned + ?Sized;

    /// Function that actually trims the `&self` content.
    ///
    /// This function takes a reference to the `self` object (in
    /// other words, don't consume a [`String`] value). If you want
    /// to replace the self obj with the new trimmed, use the Rust's
    /// `shadowing`:
    /// ```compile_fail
    /// fn some_func<T: AutoTrim>(input: T) {
    ///     let input = input.auto_trim().to_string();
    ///     // do stuff...
    /// }
    /// some_func(format!("  Passing a {}  ", "string"));
    /// ```
    fn auto_trim(&'a self) -> Self::Output;
}

impl<'a> AutoTrim<'a> for &'a str {
    type Output = Self;
    fn auto_trim(&'a self) -> Self::Output {
        self.trim()
    }
}

impl<'a> AutoTrim<'a> for String {
    type Output = &'a str;
    fn auto_trim(&'a self) -> Self::Output {
        self.trim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLES: [(&str, &str); 5] = [
        (" simple trim ", "simple trim"),
        ("    left trim", "left trim"),
        ("right trim    ", "right trim"),
        ("  strange     tri m   ", "strange     tri m"),
        ("              ", ""),
    ];

    #[test]
    fn trim_test() {
        SAMPLES
            .iter()
            .for_each(|(inp, out)| assert_eq!(&inp.auto_trim(), out))
    }
}
