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
