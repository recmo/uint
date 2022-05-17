use crate::Uint;

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    // Returns the number of leading zeros in the binary representation of `self`.
    #[must_use]
    pub fn leading_zeros(&self) -> usize {
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

    /// Length of the number in bits ignoring leading zeros.
    #[must_use]
    pub fn bit_len(&self) -> usize {
        BITS - self.leading_zeros()
    }

    /// Length of the number in bytes ignoring leading zeros.
    #[must_use]
    pub fn byte_len(&self) -> usize {
        (self.bit_len() + 7) / 8
    }

    /// Returns the base 2 logarithm of the number, rounded down.
    ///
    /// This is equivalent to the index of the highest set bit.
    ///
    /// Returns None if the number is zero.
    #[must_use]
    pub fn checked_log2(&self) -> Option<usize> {
        self.bit_len().checked_sub(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{aliases::U128, const_for, nlimbs};
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
        proptest!(|(value: u128)| {
            let uint = U128::from(value);
            assert_eq!(uint.leading_zeros(), value.leading_zeros() as usize);
        });
    }

    #[test]
    fn test_checked_log2() {
        assert_eq!(U128::from(0).checked_log2(), None);
        assert_eq!(U128::from(1).checked_log2(), Some(0));
        assert_eq!(U128::from(2).checked_log2(), Some(1));
        assert_eq!(U128::from(3).checked_log2(), Some(1));
        assert_eq!(U128::from(127).checked_log2(), Some(6));
        assert_eq!(U128::from(128).checked_log2(), Some(7));
    }
}
