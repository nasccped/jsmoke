//! # `Chunk by Any` module
//!
//! This module provides the [`ChunkByAny`] trait and it's
//! implementations.

/// Chunk a &[`str`]/[`String`] by defined [`str`] sample. Works
/// similar to [`str::split`] but still remains with the split value.
///
/// This trait can't by applied to other types since
/// [`private_mod::Sealed`] trait implementing is required (which is
/// private in this mod).
///
/// The [`ChunkByAny::chunk_by_any`] function can panics according
/// to it's implementing. Read the docstring.
pub trait ChunkByAny<'a>: private_mod::Sealed {
    /// Designed type to chunk the str value.
    type ByType;

    /// Type to be returned `Vec<Output>` (since Rust doesn't deal
    /// with returning trait signatures).
    type Output;

    /// Chunk function. By taking a [`str`] like type, chunk it by
    /// the provided `by` param.
    fn chunk_by_any(&'a self, by: impl AsRef<[Self::ByType]>) -> Vec<Self::Output>;
}

impl<'a> ChunkByAny<'a> for String {
    type ByType = &'a str;
    type Output = &'a str;

    /// # Important
    ///
    /// This functions is just a wrapper for the
    /// [`str::chunk_by_any`] function. Since the return type is a
    /// `self` reference based, the `String` implement will behaves the
    /// same as the `str` impl.
    ///
    /// Consider reading the [`ChunkByAny`] implement's doc for
    /// [`str`] type.
    fn chunk_by_any(&'a self, by: impl AsRef<[&'a str]>) -> Vec<&'a str> {
        let s: &'a str = self.as_ref();
        s.chunk_by_any(by)
    }
}

impl<'a> ChunkByAny<'a> for str {
    type ByType = &'a str;
    type Output = &'a str;

    /// # Precedence order
    ///
    /// If a `by` element is a substring of another `by` element, the
    /// first element that appears in the `by` as array (since `by`
    /// must implement the [`AsRef<ArrayRef>`]) will be prefered when
    /// searching for sample within the `self` input.
    ///
    /// With this in mind, consider putting the substrings as latest
    /// itens in `by` param:
    ///
    /// ```compile_fail
    /// // will return ["Trying", "#", "@", "to", "#", "chunk", "@", "string"]
    /// // never chunks the "#@" item...
    /// let bad_chuncking = "Trying#@to#chunk@string".chunk_by_any(["#", "@", "#@"]);
    ///
    /// // will return ["Trying", "#@", "to", "#", "chunk", "@", "string"]
    /// // chunks the "#@" item...
    /// let good_chuncking = "Trying#@to#chunk@string".chunk_by_any(["#@", "#", "@"]);
    /// ```
    ///
    /// # Panics
    ///
    /// This program will panic if any of elements within `by` param
    /// is empty string "" (since there's not a way to chunk empty
    /// string from a string content).
    fn chunk_by_any(&'a self, by: impl AsRef<[&'a str]>) -> Vec<&'a str> {
        let mut v: Vec<&'a str> = Vec::new();
        let mut slice = self;
        let seps = by.as_ref();
        if seps.contains(&"") {
            panic!("the `ChunkByAny`'s (by) param can't hold empty str!");
        } else if seps.is_empty() && !slice.is_empty() {
            v.push(slice);
            return v;
        } else if slice.is_empty() {
            return v;
        }
        while let Some((pos, sample)) = seps
            .iter()
            .filter_map(|sample| slice.find(sample).map(|pos| (pos, sample)))
            .min_by_key(|(pos, _)| *pos)
        {
            let temp = &slice[..pos];
            if !temp.is_empty() {
                v.push(temp);
            }
            v.push(sample);
            slice = &slice[(pos + sample.len())..];
        }
        if !slice.is_empty() {
            v.push(slice);
        }
        v
    }
}

mod private_mod {
    //! Private module to enforce the [`super::ChunkByAny`]'s
    //! implementing rules. Types that implements
    //! [`super::ChunkByAny`] must also implements [`self::Sealed`]
    //! trait (which is private).
    pub trait Sealed {}
    impl Sealed for String {}
    impl Sealed for str {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_over_samples() {
        let samples = [
            ["!", "*", "@", "#", "&"],
            ["*", "#", "@", "&", "!"],
            ["@", "#", "*", "&", "!"],
            ["#", "@", "*", "&", "!"],
            ["&", "@", "*", "#", "!"],
            ["!", "@", "*", "#", "&"],
            ["*", "&", "@", "#", "!"],
            ["@", "&", "*", "#", "!"],
            ["#", "&", "*", "@", "!"],
            ["&", "#", "*", "@", "!"],
            ["!", "#", "*", "@", "&"],
            ["*", "!", "@", "#", "&"],
            ["@", "!", "*", "#", "&"],
            ["#", "!", "*", "@", "&"],
            ["&", "!", "*", "@", "#"],
            ["!", "&", "*", "@", "#"],
            ["*", "@", "&", "#", "!"],
            ["@", "*", "&", "#", "!"],
            ["#", "@", "&", "*", "!"],
            ["&", "@", "!", "*", "#"],
        ];
        let s = "Please!Sir*Separate@My#String&";
        let expected = [
            "Please", "!", "Sir", "*", "Separate", "@", "My", "#", "String", "&",
        ];
        samples.into_iter().enumerate().for_each(|(i, sample)| {
            assert_eq!(
                s.chunk_by_any(sample),
                expected,
                "Failed to assert string: #{i}"
            );
        });
    }

    #[test]
    fn empty_str() {
        assert_eq!(
            "".chunk_by_any(["a sample"]),
            [] as [&str; 0],
            "Failed to assert str reference"
        );
    }

    #[test]
    fn empty_str_no_sample() {
        assert_eq!("".chunk_by_any([]), [] as [&str; 0])
    }

    #[test]
    fn empty_samples() {
        assert_eq!(
            "Some string example".to_string().chunk_by_any([]),
            ["Some string example"],
            "Failed to assert str reference"
        );
    }

    #[test]
    fn sample_precedence() {
        let bad_chuncking = "Trying#@to#chunk@string".chunk_by_any(["#", "@", "#@"]);
        let expect = ["Trying", "#", "@", "to", "#", "chunk", "@", "string"];
        assert_eq!(bad_chuncking, expect);
        let good_chuncking = "Trying#@to#chunk@string".chunk_by_any(["#@", "#", "@"]);
        let expect = ["Trying", "#@", "to", "#", "chunk", "@", "string"];
        assert_eq!(good_chuncking, expect);
    }

    #[test]
    #[should_panic]
    fn empty_string_panics() {
        let s = "String example";
        let _ = s.chunk_by_any([""]);
    }
}
