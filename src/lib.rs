#![doc = include_str!("../Readme.md")]
#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]
// This allows us to compute the number of limbs required from the bits.
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]

mod add;
mod constructors;
mod from;
mod utils;

#[cfg(feature = "proptest")]
pub mod proptest;

pub use self::add::OverflowingAdd;

/// Binary numbers modulo $2^n$.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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
    /// The size of this integer type in 64-bit limbs.
    pub const LIMBS: usize = nlimbs(BITS);

    /// Bit mask for the last limb.
    const MASK: u64 = mask(BITS);

    /// The size of this integer type in bits.
    pub const BITS: usize = BITS;

    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = Self {
        limbs: [0; nlimbs(BITS)],
    };

    /// The largest value that can be represented by this integer type,
    /// $2^{\mathtt{BITS}} − 1$.
    pub const MAX: Self = {
        let mut limbs = [u64::MAX; nlimbs(BITS)];
        if BITS > 0 {
            limbs[Self::LIMBS - 1] &= Self::MASK;
        }
        Self { limbs }
    };

    #[must_use]
    pub const fn as_limbs(&self) -> &[u64; nlimbs(BITS)] {
        &self.limbs
    }

    // TODO: Can be made `const` with `#![feature(const_mut_refs)]`.
    #[must_use]
    pub fn as_limbs_mut(&mut self) -> &mut [u64; nlimbs(BITS)] {
        &mut self.limbs
    }

    #[must_use]
    pub const fn from_limbs(limbs: [u64; nlimbs(BITS)]) -> Self {
        if BITS > 0 {
            // TODO: Add `<{BITS}>` to the type when Display works in const fn.
            assert!(
                limbs[Self::LIMBS - 1] <= Self::MASK,
                "Value too large for this Uint"
            );
        }
        Self { limbs }
    }
}

/// Number of `u64` limbs required to represent the given number of bits.
/// This needs to be public because it is used in the `Uint` type.
pub const fn nlimbs(bits: usize) -> usize {
    (bits + 63) / 64
}

/// Mask to apply to the highest limb to get the correct number of bits.
const fn mask(bits: usize) -> u64 {
    if bits == 0 {
        return 0;
    }
    let bits = bits % 64;
    if bits == 0 {
        u64::MAX
    } else {
        (1 << bits) - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mask() {
        assert_eq!(mask(0), 0);
        assert_eq!(mask(1), 1);
        assert_eq!(mask(5), 0x1f);
        assert_eq!(mask(63), u64::max_value() >> 1);
        assert_eq!(mask(64), u64::max_value());
    }

    #[test]
    fn test_max() {
        assert_eq!(Uint::<0>::MAX, Uint::zero());
        assert_eq!(Uint::<1>::MAX, Uint::from_limbs([1]));
        assert_eq!(Uint::<7>::MAX, Uint::from_limbs([127]));
        assert_eq!(Uint::<64>::MAX, Uint::from_limbs([u64::MAX]));
        assert_eq!(
            Uint::<100>::MAX,
            Uint::from_limbs([u64::MAX, u64::MAX >> 28])
        );
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
