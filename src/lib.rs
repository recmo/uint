#![doc = include_str!("../Readme.md")]
#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]
#![cfg_attr(
    any(test, feature = "bench"),
    allow(clippy::wildcard_imports, clippy::cognitive_complexity)
)]
#![allow(incomplete_features)]
// We need these features unfortunately.
// This allows us to compute the number of limbs required from the bits.
#![feature(generic_const_exprs)]

mod add;
mod bytes;
mod const_for;
mod from;
mod support;

pub use self::{add::OverflowingAdd, bytes::nbytes};
pub use uint_macro::uint;

/// The ring of numbers modulo $2^{\mathtt{BITS}}$.
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
    /// Synonym for [`Self::ZERO`].
    pub const MIN: Self = Self::ZERO;

    /// The value zero. This is the only value that exists in all [`Uint`]
    /// types.
    pub const ZERO: Self = Self {
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

    /// # Panics
    /// Panics if the value is to large for the bit-size of the Uint.
    #[must_use]
    #[track_caller]
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

    #[must_use]
    #[track_caller]
    pub fn from_limbs_slice(slice: &[u64]) -> Self {
        let mut limbs = [0; nlimbs(BITS)];
        limbs.copy_from_slice(slice);
        Self::from_limbs(limbs)
    }
}

impl<const BITS: usize> Default for Uint<BITS>
where
    [(); nlimbs(BITS)]:,
{
    fn default() -> Self {
        Self::ZERO
    }
}

/// Number of `u64` limbs required to represent the given number of bits.
/// This needs to be public because it is used in the `Uint` type.
#[must_use]
pub const fn nlimbs(bits: usize) -> usize {
    (bits + 63) / 64
}

/// Mask to apply to the highest limb to get the correct number of bits.
#[must_use]
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
        assert_eq!(Uint::<0>::MAX, Uint::ZERO);
        assert_eq!(Uint::<1>::MAX, Uint::from_limbs([1]));
        assert_eq!(Uint::<7>::MAX, Uint::from_limbs([127]));
        assert_eq!(Uint::<64>::MAX, Uint::from_limbs([u64::MAX]));
        assert_eq!(
            Uint::<100>::MAX,
            Uint::from_limbs([u64::MAX, u64::MAX >> 28])
        );
    }

    #[test]
    fn test_constants() {
        const_for!(BITS in SIZES {
            assert_eq!(Uint::<BITS>::MIN, Uint::<BITS>::ZERO);
            let _ = Uint::<BITS>::MAX;
        });
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
