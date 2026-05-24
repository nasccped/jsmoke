/// Works similar to [`TryFrom`] trait, but returns [`Option`] over `T`, instead.
pub trait MayFrom<T>: Sized {
    /// Returns an optional value of `Self` based on `T` value.
    fn may_from(value: T) -> Option<Self>;
}
