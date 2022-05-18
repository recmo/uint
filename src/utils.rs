/// Like `a % b` but returns `b` instead of `0`.
pub(crate) fn rem_up(a: usize, b: usize) -> usize {
    let rem = a % b;
    if rem > 0 {
        rem
    } else {
        b
    }
}
