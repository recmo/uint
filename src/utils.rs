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

#[cfg(has_core_intrinsics)]
pub use core::intrinsics::{likely, unlikely};

#[cfg(not(has_core_intrinsics))]
#[inline(always)]
pub const fn unlikely(b: bool) -> bool {
    b
}

#[cfg(not(has_core_intrinsics))]
#[inline(always)]
pub const fn likely(b: bool) -> bool {
    b
}
