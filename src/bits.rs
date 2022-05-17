use crate::Uint;

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    // Returns the number of leading zeros in the binary representation of `self`.
    #[must_use]
    pub fn leading_zeros(&self) -> usize {
        dbg!(BITS, &self);
        self.as_limbs()
            .iter()
            .rev()
            .position(|&limb| limb != 0)
            .map_or(BITS, |n| {
                let fixed = Self::MASK.leading_zeros() as usize;
                let skipped = n * 64;
                let top = self.as_limbs()[LIMBS - n - 1].leading_zeros() as usize;
                skipped + top - fixed
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{const_for, nlimbs};
    use proptest::proptest;

    #[test]
    fn test_leading_zeros() {
        assert_eq!(Uint::<0, 0>::ZERO.leading_zeros(), 0);
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            assert_eq!(Uint::<BITS, LIMBS>::ZERO.leading_zeros(), BITS);
            assert_eq!(Uint::<BITS, LIMBS>::MAX.leading_zeros(), 0);
            assert_eq!(Uint::<BITS, LIMBS>::from(1).leading_zeros(), BITS - 1);
            proptest!(|(value: Uint<BITS, LIMBS>)| {
                let zeros = value.leading_zeros();
                assert!(zeros <= BITS);
                // TODO: Check with bitshift operators.
                // assert!(value << zeros >= Uint::MAX >> 1);
                // assert_eq!(value >> (BITS - zeros), Uint::ZERO);
            });
        });
    }
}
