//! # AutoTrim module
//!
//! Provides auto trim feature for &[`str`] and [`String`] types.
//!
//! I could just use `some_string.trim()` from [`std`] lib, but this
//! approach enforces modularization and standardization over the
//! entire workspace.

/// Automatically trims the given value (works like [`Into`] standard
/// trait but converting default strings into trimmed).
pub trait AutoTrim<'a>: private_mod::Sealed {
    /// Function that actually trims the `&self` content.
    ///
    /// This function takes a reference to the `self` object (in
    /// other words, don't consume a [`String`] value). If you want
    /// to replace the self obj with the new trimmed, use the Rust's
    /// `shadowing`:
    /// ```compile_fail
    /// fn some_func<T: AutoTrim>(input: T) {
    ///     input = input.auto_trim().to_string();
    ///     // do stuff...
    /// }
    /// some_func(format!("  Passing a {}  ", "string"));
    /// ```
    fn auto_trim(&'a self) -> &'a str;
}

impl<'a> AutoTrim<'a> for &'a str {
    fn auto_trim(&'a self) -> &'a str {
        self.trim()
    }
}

impl<'a> AutoTrim<'a> for String {
    fn auto_trim(&'a self) -> &'a str {
        self.trim()
    }
}

mod private_mod {
    //! # [`crate::strings::auto_trim`]'s private module
    //!
    //! This module is intended to be private and declare basic rules
    //! to [`super::AutoTrim`] trait implementing.
    //!
    //! The [`super::AutoTrim`] trait can be implemented only if the
    //! given type also implements [`Sealed`] trait (which is private).
    //!
    //! By default, only &[`str`] and [`String`] types implements
    //! [`Sealed`]. So, just these types can also implement
    //! [`super::AutoTrim`].

    /// Private trait...
    pub trait Sealed {}
    impl Sealed for &str {}
    impl Sealed for String {}
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
