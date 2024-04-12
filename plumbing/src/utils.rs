/// Removes the last element from a slice
pub fn remove_last<T>(slice: &[T]) -> &[T] {
    &slice[..slice.len() - 1]
}

pub fn remove_last_if_match<'a, 'b, T: Eq>(slice: &'a [T], val: &'b T) -> &'a [T] {
    match slice.last() {
        Some(e) if *e == *val => &slice[..slice.len() - 1],
        _ => slice,
    }
}

pub fn remove_last_if_endline(slice: &[u8]) -> &[u8] {
    remove_last_if_match(slice, &b'\n')
}
