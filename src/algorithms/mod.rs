//! ⚠️ Collection of bignum algorithms.
//!
//! **Warning.** Most functions in this module are currently not considered part
//! of the stable API and may be changed or removed in future minor releases.

mod add;
pub mod div;
mod gcd;
mod mul;
#[cfg(feature = "alloc")] // TODO: Make mul_redc alloc-free
mod mul_redc;
mod ops;
mod shift;

pub use self::{
    add::{adc_n, cmp, sbb_n},
    div::div,
    gcd::{gcd, gcd_extended, inv_mod, LehmerMatrix},
    mul::{add_nx1, addmul, addmul_n, addmul_nx1, addmul_ref, submul_nx1},
    ops::{adc, sbb},
    shift::{shift_left_small, shift_right_small},
};
#[cfg(feature = "alloc")]
pub use mul_redc::mul_redc;

trait DoubleWord<T>: Sized + Copy {
    fn join(high: T, low: T) -> Self;
    fn add(a: T, b: T) -> Self;
    fn mul(a: T, b: T) -> Self;
    fn muladd(a: T, b: T, c: T) -> Self;
    fn muladd2(a: T, b: T, c: T, d: T) -> Self;

    fn high(self) -> T;
    fn low(self) -> T;
    fn split(self) -> (T, T);
}

impl DoubleWord<u64> for u128 {
    #[inline(always)]
    fn join(high: u64, low: u64) -> Self {
        (Self::from(high) << 64) | Self::from(low)
    }

    /// Computes `a + b` as a 128-bit value.
    #[inline(always)]
    fn add(a: u64, b: u64) -> Self {
        Self::from(a) + Self::from(b)
    }

    /// Computes `a * b` as a 128-bit value.
    #[inline(always)]
    fn mul(a: u64, b: u64) -> Self {
        Self::from(a) * Self::from(b)
    }

    /// Computes `a * b + c` as a 128-bit value. Note that this can not
    /// overflow.
    #[inline(always)]
    fn muladd(a: u64, b: u64, c: u64) -> Self {
        Self::from(a) * Self::from(b) + Self::from(c)
    }

    /// Computes `a * b + c + d` as a 128-bit value. Note that this can not
    /// overflow.
    #[inline(always)]
    fn muladd2(a: u64, b: u64, c: u64, d: u64) -> Self {
        Self::from(a) * Self::from(b) + Self::from(c) + Self::from(d)
    }

    #[inline(always)]
    fn high(self) -> u64 {
        (self >> 64) as u64
    }

    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    fn low(self) -> u64 {
        self as u64
    }

    #[inline(always)]
    fn split(self) -> (u64, u64) {
        (self.low(), self.high())
    }
}
