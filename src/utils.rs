/// Like `a % b` but returns `b` instead of `0`.
#[must_use]
pub(crate) fn rem_up(a: usize, b: usize) -> usize {
    let rem = a % b;
    if rem > 0 {
        rem
    } else {
        b
    }
}

#[must_use]
pub(crate) fn trim_end_slice<T: PartialEq>(slice: &[T], value: T) -> &[T] {
    slice
        .iter()
        .rposition(|b| b != &value)
        .map_or_else(|| &slice[..0], |len| &slice[..=len])
}

pub(crate) fn trim_end_vec<T: PartialEq>(vec: &mut Vec<T>, value: T) {
    if let Some(last) = vec.iter().rposition(|b| b != &value) {
        vec.truncate(last + 1);
    } else {
        vec.clear();
    }
}
