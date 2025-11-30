use std::ops::{
    Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

/// How many times can the given pattern repeats. Works like regex
/// pattern sufix (`?`, `+`, `*`, ...).
#[derive(Clone, Debug, PartialEq)]
pub enum HowMuchOfPattern {
    /// No more than one occurrence of the pattern (`a?`).
    ZeroOrOne,
    /// Any number of occurrences (`a*`).
    ZeroOrMore,
    /// At least one occurrence (`a+`).
    OneOrMore,
    /// Exactly this amount of occurrences (`a{N}`).
    Exactly(usize),
    /// At least this amount (`a{N,}`).
    AtLeast(usize),
    /// Until this amount (`a{0,N}`).
    Until(usize),
    /// Between this amount (`a{MIN,MAX}`). Note that `<MIN>` are
    /// range inclusive `MAX` + `MAX` must always be greater than
    /// `MIN`.
    Between(usize, usize),
}

impl HowMuchOfPattern {
    /// returns the string representation (regex) of the
    /// [`HowMuchOfPattern`] object.
    pub fn repr(&self) -> String {
        match self {
            Self::ZeroOrOne => "?".into(),
            Self::ZeroOrMore => "*".into(),
            Self::OneOrMore => "+".into(),
            Self::Exactly(x) => format!("{{{}}}", x),
            Self::AtLeast(x) => format!("{{{},}}", x),
            Self::Until(x) => format!("{{0,{}}}", x),
            Self::Between(a, b) => format!("{{{},{}}}", a, b),
        }
    }
}

/// Private trait to prevent recursive implementing with
/// `impl<T: Sealed> From<T> for HowMuchOfPattern`.
pub trait Sealed<T>: RangeBounds<T> {}

impl<T> Sealed<T> for Range<T> {}
impl<T> Sealed<T> for RangeFrom<T> {}
impl<T> Sealed<T> for RangeFull {}
impl<T> Sealed<T> for RangeInclusive<T> {}
impl<T> Sealed<T> for RangeTo<T> {}
impl<T> Sealed<T> for RangeToInclusive<T> {}

impl<T: Sealed<usize>> From<T> for HowMuchOfPattern {
    fn from(value: T) -> Self {
        let simple_unwrapper = |b: Bound<&usize>| match b {
            Bound::Included(&x) => Some(x),
            Bound::Excluded(&x) => Some(x - 1),
            Bound::Unbounded => None,
        };
        let s = simple_unwrapper(value.start_bound()).unwrap_or_default();
        let e = simple_unwrapper(value.end_bound());
        match (s, e) {
            (0, Some(1)) => Self::ZeroOrOne,
            (0, None) => Self::ZeroOrMore,
            (1, None) => Self::OneOrMore,
            (a, Some(b)) if a == b => Self::Exactly(a),
            (x, None) => Self::AtLeast(x),
            (0, Some(x)) => Self::Until(x),
            (a, Some(b)) => Self::Between(a, b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /* I know. These tests s****, but I can't create a
     * `Box<dyn RangeBounds<usize>>` collection :^( */

    #[test]
    fn test_from() {
        assert_eq!(HowMuchOfPattern::ZeroOrOne, HowMuchOfPattern::from(..=1));
        assert_eq!(HowMuchOfPattern::ZeroOrOne, HowMuchOfPattern::from(0..=1));
        assert_eq!(HowMuchOfPattern::ZeroOrOne, HowMuchOfPattern::from(0..2));
        assert_eq!(HowMuchOfPattern::ZeroOrMore, HowMuchOfPattern::from(..));
        assert_eq!(HowMuchOfPattern::ZeroOrMore, HowMuchOfPattern::from(0..));
        assert_eq!(HowMuchOfPattern::OneOrMore, HowMuchOfPattern::from(1..));
        assert_eq!(HowMuchOfPattern::Exactly(3), HowMuchOfPattern::from(3..4));
        assert_eq!(HowMuchOfPattern::Exactly(7), HowMuchOfPattern::from(7..=7));
        assert_eq!(HowMuchOfPattern::Exactly(0), HowMuchOfPattern::from(..1));
        assert_eq!(HowMuchOfPattern::AtLeast(3), HowMuchOfPattern::from(3..));
        assert_eq!(HowMuchOfPattern::Until(3), HowMuchOfPattern::from(..=3));
        assert_eq!(HowMuchOfPattern::Until(40), HowMuchOfPattern::from(0..41));
        assert_eq!(
            HowMuchOfPattern::Between(10, 20),
            HowMuchOfPattern::from(10..21)
        );
        assert_eq!(
            HowMuchOfPattern::Between(10, 20),
            HowMuchOfPattern::from(10..=20)
        );
    }

    #[test]
    #[should_panic]
    fn bad_range_panics() {
        let _ = HowMuchOfPattern::from(0..0);
    }

    #[test]
    fn test_repr() {
        assert_eq!("?", HowMuchOfPattern::ZeroOrOne.repr());
        assert_eq!("*", HowMuchOfPattern::ZeroOrMore.repr());
        assert_eq!("+", HowMuchOfPattern::OneOrMore.repr());
        assert_eq!("{3}", HowMuchOfPattern::Exactly(3).repr());
        assert_eq!("{2,}", HowMuchOfPattern::AtLeast(2).repr());
        assert_eq!("{0,43}", HowMuchOfPattern::Until(43).repr());
        assert_eq!("{1,2}", HowMuchOfPattern::Between(1, 2).repr());
    }
}
