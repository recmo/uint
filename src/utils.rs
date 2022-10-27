/// Like `a % b` but returns `b` instead of `0`.
#[must_use]
pub const fn rem_up(a: usize, b: usize) -> usize {
    let rem = a % b;
    if rem > 0 {
        rem
    } else {
        b
    }
}

#[must_use]
pub fn trim_end_slice<'a, T: PartialEq>(slice: &'a [T], value: &T) -> &'a [T] {
    slice
        .iter()
        .rposition(|b| b != value)
        .map_or_else(|| &slice[..0], |len| &slice[..=len])
}

pub fn trim_end_vec<T: PartialEq>(vec: &mut Vec<T>, value: &T) {
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

#[macro_export]
#[doc(hidden)]
// TODO: (BLOCKED) make this macro `pub(crate)` when supported.
// See <https://github.com/rust-lang/rust/issues/39412>
macro_rules! impl_bin_op {
    ($trait:ident, $fn:ident, $trait_assign:ident, $fn_assign:ident, $fdel:ident) => {
        impl<const BITS: usize, const LIMBS: usize> $trait_assign<Uint<BITS, LIMBS>>
            for Uint<BITS, LIMBS>
        {
            #[allow(clippy::inline_always)]
            #[inline(always)]
            #[track_caller]
            fn $fn_assign(&mut self, rhs: Uint<BITS, LIMBS>) {
                *self = self.$fdel(rhs);
            }
        }
        impl<const BITS: usize, const LIMBS: usize> $trait_assign<&Uint<BITS, LIMBS>>
            for Uint<BITS, LIMBS>
        {
            #[allow(clippy::inline_always)]
            #[inline(always)]
            #[track_caller]
            fn $fn_assign(&mut self, rhs: &Uint<BITS, LIMBS>) {
                *self = self.$fdel(*rhs);
            }
        }
        impl<const BITS: usize, const LIMBS: usize> $trait<Uint<BITS, LIMBS>>
            for Uint<BITS, LIMBS>
        {
            type Output = Uint<BITS, LIMBS>;

            #[allow(clippy::inline_always)]
            #[inline(always)]
            #[track_caller]
            fn $fn(self, rhs: Uint<BITS, LIMBS>) -> Self::Output {
                self.$fdel(rhs)
            }
        }
        impl<const BITS: usize, const LIMBS: usize> $trait<&Uint<BITS, LIMBS>>
            for Uint<BITS, LIMBS>
        {
            type Output = Uint<BITS, LIMBS>;

            #[allow(clippy::inline_always)]
            #[inline(always)]
            #[track_caller]
            fn $fn(self, rhs: &Uint<BITS, LIMBS>) -> Self::Output {
                self.$fdel(*rhs)
            }
        }
        impl<const BITS: usize, const LIMBS: usize> $trait<Uint<BITS, LIMBS>>
            for &Uint<BITS, LIMBS>
        {
            type Output = Uint<BITS, LIMBS>;

            #[allow(clippy::inline_always)]
            #[inline(always)]
            #[track_caller]
            fn $fn(self, rhs: Uint<BITS, LIMBS>) -> Self::Output {
                self.$fdel(rhs)
            }
        }
        impl<const BITS: usize, const LIMBS: usize> $trait<&Uint<BITS, LIMBS>>
            for &Uint<BITS, LIMBS>
        {
            type Output = Uint<BITS, LIMBS>;

            #[allow(clippy::inline_always)]
            #[inline(always)]
            #[track_caller]
            fn $fn(self, rhs: &Uint<BITS, LIMBS>) -> Self::Output {
                self.$fdel(*rhs)
            }
        }
    };
}
