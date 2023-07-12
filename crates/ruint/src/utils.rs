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

#[must_use]
pub(crate) fn trim_end_slice<'a, T: PartialEq>(slice: &'a [T], value: &T) -> &'a [T] {
    slice
        .iter()
        .rposition(|b| b != value)
        .map_or_else(|| &slice[..0], |len| &slice[..=len])
}

pub(crate) fn trim_end_vec<T: PartialEq>(vec: &mut Vec<T>, value: &T) {
    if let Some(last) = vec.iter().rposition(|b| b != value) {
        vec.truncate(last + 1);
    } else {
        vec.clear();
    }
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
