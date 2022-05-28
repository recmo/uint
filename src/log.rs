use crate::Uint;

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    #[must_use]
    pub fn checked_log(self, base: u64) -> Option<u64> {
        if base < 2 || self == Self::ZERO {
            return None;
        }
        Some(self.log(base))
    }

    #[must_use]
    pub fn checked_log10(self) -> Option<u64> {
        self.checked_log(10)
    }

    /// Returns the base 2 logarithm of the number, rounded down.
    ///
    /// This is equivalent to the index of the highest set bit.
    ///
    /// Returns None if the number is zero.
    #[must_use]
    pub fn checked_log2(self) -> Option<u64> {
        self.checked_log(2)
    }

    pub fn log(self, base: u64) -> u64 {
        assert!(base >= 2);
        assert!(self != Self::ZERO);
        if base == 2 {
            return self.bit_len() as u64 - 1;
        }
        if self < Self::from(base) {
            return 0;
        }

        // Find approximate result
        let approx_self = f64::from(self);
        let approx_base = base as f64;
        let approx_log = approx_self.log(approx_base);
        debug_assert!(approx_log > 0.0);

        let mut result = approx_log as u64;

        // Adjust result to get the exact value
        loop {
            if let Some(value) = Self::from(base).checked_pow(Self::from(result)) {
                if value > self {
                    result -= 1;
                    continue;
                }
            }
            break;
        }
        loop {
            if let Some(value) = Self::from(base).checked_pow(Self::from(result + 1)) {
                if value <= self {
                    result += 1;
                    continue;
                }
            }
            break;
        }

        assert!(Self::from(base).pow(Self::from(result)) <= self);
        if let Some(value) = Self::from(base).checked_pow(Self::from(result + 1)) {
            assert!(value > self);
        }

        result
    }

    pub fn log10(self) -> u64 {
        self.log(10)
    }

    pub fn log2(self) -> u64 {
        self.log(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{aliases::U128, const_for, nlimbs};
    use proptest::proptest;

    #[test]
    fn test_checked_log2() {
        assert_eq!(U128::from(0).checked_log2(), None);
        assert_eq!(U128::from(1).checked_log2(), Some(0));
        assert_eq!(U128::from(2).checked_log2(), Some(1));
        assert_eq!(U128::from(3).checked_log2(), Some(1));
        assert_eq!(U128::from(127).checked_log2(), Some(6));
        assert_eq!(U128::from(128).checked_log2(), Some(7));
    }

    #[test]
    fn test_pow_log() {
        const_for!(BITS in NON_ZERO if (BITS >= 64) {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(b in 2_u64..100, e in 0..BITS)| {
                if let Some(value) = U::from(b).checked_pow(U::from(e)) {
                    assert!(value > U::ZERO);
                    assert_eq!(value.log(b), e as u64);
                    // assert_eq!(value.log(b + U::from(1)), e as u64);
                }
            });
        });
    }
}
