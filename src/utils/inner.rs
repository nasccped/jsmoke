/// Trait to borrow the inner data (as reference) from complex types. A lot useful for tuple
/// structs to avoid the `Struct(pub item)` thing.
pub trait Inner<T: ?Sized> {
    /// Extracts the item inner data as reference.
    fn inner(&self) -> &T;
}
