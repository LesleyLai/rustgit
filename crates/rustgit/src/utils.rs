/// Removes the last element from a slice
pub fn remove_last<T>(slice: &[T]) -> &[T] {
    &slice[..slice.len() - 1]
}

/// trim characters such as space, tab, or new lines
pub fn trim_whitespace(x: &[u8]) -> &[u8] {
    match x.last() {
        Some(c) if c.is_ascii_whitespace() => &x[..x.len() - 1],
        _ => x,
    }
}
