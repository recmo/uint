use crate::Uint;

use core::ops::ShrAssign;

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    pub fn reverse_bits(&mut self) {
        self.limbs.reverse();
        for limb in &mut self.limbs {
            *limb = limb.reverse_bits();
        }
        if BITS % 64 != 0 {
            *self >>= 64 - BITS % 64;
        }
    }

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

    /// Returns the most significant 64 bits of the number and the exponent.
    ///
    /// Given return value $(\mathtt{bits}, \mathtt{exponent})$, the `self` can
    /// be approximated as
    ///
    /// $$
    /// \mathtt{self} ≈ \mathtt{bits} ⋅ 2^\mathtt{exponent}
    /// $$
    ///
    /// If `self` is $<≥> 2^{63}$, then `exponent` will be zero and `bits` will
    /// have leading zeros.
    #[must_use]
    pub fn most_significant_bits(&self) -> (u64, usize) {
        let first_set_limb = self
            .as_limbs()
            .iter()
            .rposition(|&limb| limb != 0)
            .unwrap_or(0);
        if first_set_limb == 0 {
            (self.as_limbs().first().copied().unwrap_or(0), 0)
        } else {
            let hi = self.as_limbs()[first_set_limb];
            let lo = self.as_limbs()[first_set_limb - 1];
            let leading_zeros = hi.leading_zeros();
            let bits = if leading_zeros > 0 {
                (hi << leading_zeros) | (lo >> (64 - leading_zeros))
            } else {
                hi
            };
            let exponent = first_set_limb * 64 - leading_zeros as usize;
            (bits, exponent)
        }
    }
}

impl<const BITS: usize, const LIMBS: usize> ShrAssign<usize> for Uint<BITS, LIMBS> {
    fn shr_assign(&mut self, rhs: usize) {
        let (limbs, bits) = (rhs / 64, rhs % 64);
        if bits == 0 {
            for i in 0..LIMBS - limbs {
                self.limbs[i] = self.limbs[i + limbs];
            }
        } else {
            for i in 0..LIMBS - limbs {
                self.limbs[i] =
                    self.limbs[i + limbs] >> bits | self.limbs[i + limbs + 1] << (64 - bits);
            }
        }
        for i in LIMBS - limbs..LIMBS {
            self.limbs[i] = 0;
        }
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
    fn test_most_significant_bits() {
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint::<BITS, LIMBS>;
            proptest!(|(value: u64)| {
                let value = if U::LIMBS <= 1 { value & U::MASK } else { value };
                assert_eq!(U::from(value).most_significant_bits(), (value, 0));
            });
        });
        proptest!(|(mut limbs: [u64; 2])| {
            if limbs[1] == 0 {
                limbs[1] = 1;
            }
            let (bits, exponent) = U128::from_limbs(limbs).most_significant_bits();
            assert!(bits >= 1_u64 << 63);
            assert_eq!(exponent, 64 - limbs[1].leading_zeros() as usize);
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
