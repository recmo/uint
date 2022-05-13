#![doc = include_str!("../Readme.md")]
#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]
// Required
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_for)]
#![feature(const_mut_refs)]
#![feature(specialization)]

mod add;

#[cfg(feature = "proptest")]
pub mod proptest;

pub use self::add::OverflowingAdd;

/// Binary numbers modulo $2^n$.
#[derive(Clone, Copy, Debug)]
pub struct Uint<const BITS: usize>
where
    [(); num_limbs(BITS)]:,
{
    limbs: [u64; num_limbs(BITS)],
}

impl<const BITS: usize> Uint<BITS>
where
    [(); num_limbs(BITS)]:,
{
    pub const BITS: usize = BITS;
    pub const LIMBS: usize = num_limbs(BITS);

    #[must_use]
    pub const fn zero() -> Self {
        Self::from_limbs([0; num_limbs(BITS)])
    }

    #[must_use]
    pub const fn one() -> Self {
        let mut result = Self::zero();
        result.limbs[0] = 1;
        result
    }

    #[must_use]
    pub const fn from_limbs(limbs: [u64; num_limbs(BITS)]) -> Self {
        Self { limbs }
    }

    #[must_use]
    pub fn from_limbs_slice(slice: &[u64]) -> Self {
        let mut limbs = [0; num_limbs(BITS)];
        limbs.copy_from_slice(slice);
        Self { limbs }
    }
}

/// Number of `u64` limbs required to represent the given number of bits.
pub const fn num_limbs(bits: usize) -> usize {
    (bits + 63) / 64
}

#[cfg(feature = "bench")]
pub mod bench {
    use super::*;
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        add::bench::group(criterion);
    }
}
