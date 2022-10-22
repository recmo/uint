//! ⚠️ Collection of bignum algorithms.
//!
//! **Warning.** Most functions in this module are currently not considered part
//! of the stable API and may be changed or removed in future minor releases.

mod add;
mod div;
mod gcd;
mod mul;
mod mul_redc;
mod ops;
mod shift;

pub use self::{
    div::div_rem,
    gcd::{gcd, gcd_extended, inv_mod, LehmerMatrix},
    mul::{mul, mul_inline},
    mul_redc::mul_redc,
    shift::{shift_left_small, shift_right_small},
};

trait DoubleWord<T>: Sized + Copy {
    fn join(high: T, low: T) -> Self;
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

    #[inline(always)]
    fn mul(a: u64, b: u64) -> Self {
        Self::from(a) * Self::from(b)
    }

    #[inline(always)]
    fn muladd(a: u64, b: u64, c: u64) -> Self {
        Self::from(a) * Self::from(b) + Self::from(c)
    }

    #[inline(always)]
    fn muladd2(a: u64, b: u64, c: u64, d: u64) -> Self {
        Self::from(a) * Self::from(b) + Self::from(c) + Self::from(d)
    }

    #[inline(always)]
    fn high(self) -> u64 {
        (self >> 64) as u64
    }

    #[inline(always)]
    fn low(self) -> u64 {
        self as u64
    }

    #[inline(always)]
    fn split(self) -> (u64, u64) {
        (self.low(), self.high())
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        gcd::bench::group(criterion);
        div::bench::group(criterion);
    }
}
