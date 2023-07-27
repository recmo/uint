/// Like `a % b` but returns `b` instead of `0`.
#[must_use]
pub(crate) const fn rem_up(a: usize, b: usize) -> usize {
    let rem = a % b;
    if rem > 0 {
        rem
    } else {
        b
    }
}

fn last_idx<T: PartialEq>(x: &[T], value: &T) -> Option<usize> {
    x.iter().rposition(|b| b != value)
}

#[must_use]
#[inline]
pub(crate) fn trim_end_slice<'a, T: PartialEq>(slice: &'a [T], value: &T) -> &'a [T] {
    &slice[..last_idx(slice, value).unwrap_or(0)]
}

#[inline]
pub(crate) fn trim_end_vec<T: PartialEq>(vec: &mut Vec<T>, value: &T) {
    vec.truncate(last_idx(vec, value).map_or(0, |idx| idx + 1));
}

// Branch prediction hints.
#[cfg(feature = "nightly")]
pub(crate) use core::intrinsics::{likely, unlikely};

// On stable we can use #[cold] to get a equivalent effect: this attribute
// suggests that the function is unlikely to be called
#[cfg(not(feature = "nightly"))]
#[inline(always)]
#[cold]
const fn cold() {}

#[cfg(not(feature = "nightly"))]
#[inline(always)]
pub(crate) const fn likely(b: bool) -> bool {
    if !b {
        cold();
    }
    b
}

#[cfg(not(feature = "nightly"))]
#[inline(always)]
pub(crate) const fn unlikely(b: bool) -> bool {
    if b {
        cold();
    }
    b
}
