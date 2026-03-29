/// A trait that implements the [`OptionallyFrom::optionally_from`] function.
///
/// This function is specific for warnings and things that **can** return error or other kind of
/// stuff. I think that's better than using [`Result<(), Error>`] or [`Some<Error>`].
pub trait OptionallyFrom<T> {
    /// A function that **can** return [`Some`] from a given value.
    fn optionally_from(value: T) -> Option<Self>
    where
        Self: Sized;
}
