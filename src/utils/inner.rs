/// Trait for extracting inner data from complex types. A lot useful for tuple structs to avoid
/// the `Struct(pub item)` thing.
pub trait Inner<T> {
    /// Extracts the item inner data.
    fn inner(&self) -> T;
}
