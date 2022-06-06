//! ⚠️ Collection of bignum algorithms.
//!
//! **Warning.** Most functions in this module are currently not considered part
//! of the stable API and may be changed or removed in future minor releases.

mod div;
mod gcd;
mod mul;

pub use self::{
    div::div_rem,
    gcd::{gcd, gcd_extended, inv_mod, LehmerMatrix},
    mul::{mul, mul_inline},
};

#[cfg(feature = "bench")]
pub mod bench {
    use super::*;
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        gcd::bench::group(criterion);
    }
}
