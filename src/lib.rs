#![doc = include_str!("../Readme.md")]
#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]
// This allows us to compute the number of limbs required from the bits.
#![feature(generic_const_exprs)]
// This allows architecture specific overrides.
// TODO: Might use conditional code instead.
#![feature(min_specialization)]

mod add;

#[cfg(feature = "proptest")]
pub mod proptest;

pub use self::add::OverflowingAdd;

/// Binary numbers modulo $2^n$.
#[derive(Clone, Copy, Debug)]
pub struct Uint<const BITS: usize>
where
    [(); nlimbs(BITS)]:,
{
    limbs: [u64; nlimbs(BITS)],
}

impl<const BITS: usize> Uint<BITS>
where
    [(); nlimbs(BITS)]:,
{
    pub const BITS: usize = BITS;
    pub const LIMBS: usize = nlimbs(BITS);
    const MASK: u64 = mask(BITS);

    #[must_use]
    pub const fn zero() -> Self {
        Self::from_limbs([0; nlimbs(BITS)])
    }

    #[must_use]
    pub const fn one() -> Self {
        let mut result = Self::zero();
        result.limbs[0] = 1;
        result
    }

    #[must_use]
    pub const fn from_limbs(limbs: [u64; nlimbs(BITS)]) -> Self {
        Self { limbs }
    }

    #[must_use]
    pub fn from_limbs_slice(slice: &[u64]) -> Self {
        let mut limbs = [0; nlimbs(BITS)];
        limbs.copy_from_slice(slice);
        Self { limbs }
    }
}

/// Number of `u64` limbs required to represent the given number of bits.
const fn nlimbs(bits: usize) -> usize {
    (bits + 63) / 64
}

/// Mask to apply to the highest limb to get the correct number of bits.
const fn mask(bits: usize) -> u64 {
    let bits = bits % 64;
    if bits == 0 {
        0
    } else {
        (1 << bits) - 1
    }
}

#[cfg(feature = "bench")]
pub mod bench {
    use super::*;
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        add::bench::group(criterion);
    }
}
