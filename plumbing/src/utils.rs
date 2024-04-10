/// Removes the last element from a slice
pub fn remove_last<T>(slice: &[T]) -> &[T] {
    &slice[..slice.len() - 1]
}
