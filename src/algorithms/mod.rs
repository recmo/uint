//! ⚠️ Collection of bignum algorithms.
//!
//! **Warning.** Most functions in this module are currently not considered part
//! of the stable API and may be changed or removed in future minor releases.

#![allow(missing_docs)] // TODO: document algorithms

mod add;
mod cmp;
pub mod div;
mod gcd;
mod mul;
mod mul_redc;
mod ops;
mod shift;

pub use self::{
    add::{adc_n, sbb_n},
    cmp::{cmp, ge, gt, le, lt},
    div::div,
    gcd::{gcd, gcd_extended, inv_mod, LehmerMatrix},
    mul::{add_nx1, addmul, addmul_n, addmul_nx1, mul_nx1, submul_nx1},
    mul_redc::{mul_redc, square_redc},
    ops::{adc, sbb},
    shift::{shift_left_small, shift_right_small},
};

pub(crate) trait DoubleWord<T: Default>: Sized + Copy {
    /// `high << 64 | low`
    fn join(high: T, low: T) -> Self;
    /// `(low, high)`
    fn split(self) -> (T, T);

    /// `a * b + c + d`
    fn muladd2(a: T, b: T, c: T, d: T) -> Self;

    /// `a + b`
    #[inline(always)]
    fn add(a: T, b: T) -> Self {
        Self::muladd2(T::default(), T::default(), a, b)
    }
    /// `a * b`
    #[inline(always)]
    fn mul(a: T, b: T) -> Self {
        Self::muladd2(a, b, T::default(), T::default())
    }
    /// `a * b + c`
    #[inline(always)]
    fn muladd(a: T, b: T, c: T) -> Self {
        Self::muladd2(a, b, c, T::default())
    }

    #[inline(always)]
    fn high(self) -> T {
        self.split().1
    }
    #[inline(always)]
    fn low(self) -> T {
        self.split().0
    }
}

#[allow(clippy::cast_possible_truncation)]
impl DoubleWord<u64> for u128 {
    #[inline(always)]
    fn join(high: u64, low: u64) -> Self {
        (Self::from(high) << 64) | Self::from(low)
    }

    #[inline(always)]
    fn split(self) -> (u64, u64) {
        (self as u64, (self >> 64) as u64)
    }

    #[inline(always)]
    fn muladd2(a: u64, b: u64, c: u64, d: u64) -> Self {
        #[cfg(feature = "nightly")]
        {
            let (low, high) = u64::carrying_mul_add(a, b, c, d);
            Self::join(high, low)
        }
        #[cfg(not(feature = "nightly"))]
        {
            Self::from(a) * Self::from(b) + Self::from(c) + Self::from(d)
        }
    }
}

// Helper while [Rust#85532](https://github.com/rust-lang/rust/issues/85532) stabilizes.
#[inline]
#[must_use]
pub const fn carrying_add(lhs: u64, rhs: u64, carry: bool) -> (u64, bool) {
    #[cfg(feature = "nightly")]
    {
        lhs.carrying_add(rhs, carry)
    }
    #[cfg(not(feature = "nightly"))]
    {
        let (result, carry_1) = lhs.overflowing_add(rhs);
        let (result, carry_2) = result.overflowing_add(carry as u64);
        (result, carry_1 | carry_2)
    }
}

// Helper while [Rust#85532](https://github.com/rust-lang/rust/issues/85532) stabilizes.
#[inline]
#[must_use]
pub const fn borrowing_sub(lhs: u64, rhs: u64, borrow: bool) -> (u64, bool) {
    #[cfg(feature = "nightly")]
    {
        lhs.borrowing_sub(rhs, borrow)
    }
    #[cfg(not(feature = "nightly"))]
    {
        let (result, borrow_1) = lhs.overflowing_sub(rhs);
        let (result, borrow_2) = result.overflowing_sub(borrow as u64);
        (result, borrow_1 | borrow_2)
    }
}

#[inline]
pub(crate) const fn trim_end_zeros(mut x: &[u64]) -> &[u64] {
    while let [rest @ .., 0] = x {
        x = rest;
    }
    x
}

#[inline]
pub(crate) fn trim_end_zeros_mut(mut x: &mut [u64]) -> &mut [u64] {
    while let [rest @ .., 0] = x {
        x = rest;
    }
    x
}
