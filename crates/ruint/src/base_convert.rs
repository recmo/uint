use crate::Uint;
use thiserror::Error;

/// Error for [`from_base_le`][Uint::from_base_le] and
/// [`from_base_be`][Uint::from_base_be].
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum BaseConvertError {
    /// The value is too large to fit the target type.
    #[error("The value is too large to fit the target type")]
    Overflow,

    /// The requested number base `.0` is less than two.
    #[error("The requested number base {0} is less than two")]
    InvalidBase(u64),

    /// The provided digit `.0` is out of range for requested base `.1`.
    #[error("digit {0} is out of range for base {1}")]
    InvalidDigit(u64, u64),
}

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Returns an iterator over the base `base` digits of the number in
    /// little-endian order.
    ///
    /// Pro tip: instead of setting `base = 10`, set it to the highest
    /// power of `10` that still fits `u64`. This way much fewer iterations
    /// are required to extract all the digits.
    // OPT: Internalize this trick so the user won't have to worry about it.
    /// # Panics
    ///
    /// Panics if the base is less than 2.
    pub fn to_base_le(&self, base: u64) -> impl Iterator<Item = u64> {
        assert!(base > 1);
        SpigotLittle {
            base,
            limbs: self.limbs,
        }
    }

    /// Returns an iterator over the base `base` digits of the number in
    /// big-endian order.
    ///
    /// Pro tip: instead of setting `base = 10`, set it to the highest
    /// power of `10` that still fits `u64`. This way much fewer iterations
    /// are required to extract all the digits.
    ///
    /// # Panics
    ///
    /// Panics if the base is less than 2.
    pub fn to_base_be(&self, base: u64) -> impl Iterator<Item = u64> {
        assert!(base > 1);
        // OPT: Find an allocation free method. Maybe extract from the top?
        OwnedVecIterator {
            vec: self.to_base_le(base).collect(),
        }
    }

    /// Constructs the [`Uint`] from digits in the base `base` in little-endian.
    ///
    /// # Errors
    ///
    /// * [`BaseConvertError::InvalidBase`] if the base is less than 2.
    /// * [`BaseConvertError::InvalidDigit`] if a digit is out of range.
    /// * [`BaseConvertError::Overflow`] if the number is too large to
    /// fit.
    pub fn from_base_le<I: IntoIterator<Item = u64>>(
        base: u64,
        digits: I,
    ) -> Result<Self, BaseConvertError> {
        let mut digits: Vec<_> = digits.into_iter().collect();
        digits.reverse();
        Self::from_base_be(base, digits)
    }

    /// Constructs the [`Uint`] from digits in the base `base` in big-endian.
    ///
    /// # Errors
    ///
    /// * [`BaseConvertError::InvalidBase`] if the base is less than 2.
    /// * [`BaseConvertError::InvalidDigit`] if a digit is out of range.
    /// * [`BaseConvertError::Overflow`] if the number is too large to
    /// fit.
    pub fn from_base_be<I: IntoIterator<Item = u64>>(
        base: u64,
        digits: I,
    ) -> Result<Self, BaseConvertError> {
        // OPT: Special handling of bases that divide 2^64, and bases that are
        // powers of 2.
        // OPT: Same trick as with `to_base_le`, find the largest power of base
        // that fits `u64` and accumulate there first.
        if base < 2 {
            return Err(BaseConvertError::InvalidBase(base));
        }
        let mut result = Self::ZERO;
        for digit in digits {
            if digit >= base {
                return Err(BaseConvertError::InvalidDigit(digit, base));
            }
            // Multiply by base.
            // OPT: keep track of non-zero limbs and mul the minimum.
            let mut carry: u128 = u128::from(digit);
            #[allow(clippy::cast_possible_truncation)]
            for limb in &mut result.limbs {
                carry += u128::from(*limb) * u128::from(base);
                *limb = carry as u64;
                carry >>= 64;
            }
            if carry > 0 || (LIMBS != 0 && result.limbs[LIMBS - 1] > Self::MASK) {
                return Err(BaseConvertError::Overflow);
            }
        }

        Ok(result)
    }
}

struct SpigotLittle<const LIMBS: usize> {
    base:  u64,
    limbs: [u64; LIMBS],
}

impl<const LIMBS: usize> Iterator for SpigotLittle<LIMBS> {
    type Item = u64;

    #[allow(clippy::cast_possible_truncation)] // Doesn't truncate
    fn next(&mut self) -> Option<Self::Item> {
        // Knuth Algorithm S.
        let mut zero: u64 = 0_u64;
        let mut remainder = 0_u128;
        // OPT: If we keep track of leading zero limbs we can half iterations.
        for limb in self.limbs.iter_mut().rev() {
            zero |= *limb;
            remainder <<= 64;
            remainder |= u128::from(*limb);
            *limb = (remainder / u128::from(self.base)) as u64;
            remainder %= u128::from(self.base);
        }
        if zero == 0 {
            None
        } else {
            Some(remainder as u64)
        }
    }
}

struct OwnedVecIterator {
    vec: Vec<u64>,
}

impl Iterator for OwnedVecIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.vec.pop()
    }
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
#[allow(clippy::zero_prefixed_literal)]
mod tests {
    use super::*;

    // 90630363884335538722706632492458228784305343302099024356772372330524102404852
    const N: Uint<256, 4> = Uint::from_limbs([
        0xa8ec92344438aaf4_u64,
        0x9819ebdbd1faaab1_u64,
        0x573b1a7064c19c1a_u64,
        0xc85ef7d79691fe79_u64,
    ]);

    #[test]
    fn test_to_base_le() {
        assert_eq!(
            Uint::<64, 1>::from(123456789)
                .to_base_le(10)
                .collect::<Vec<_>>(),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1]
        );
        assert_eq!(
            N.to_base_le(10000000000000000000_u64).collect::<Vec<_>>(),
            vec![
                2372330524102404852,
                0534330209902435677,
                7066324924582287843,
                0630363884335538722,
                9
            ]
        );
    }

    #[test]
    fn test_from_base_le() {
        assert_eq!(
            Uint::<64, 1>::from_base_le(10, [9, 8, 7, 6, 5, 4, 3, 2, 1]),
            Ok(Uint::<64, 1>::from(123456789))
        );
        assert_eq!(
            Uint::<256, 4>::from_base_le(10000000000000000000_u64, [
                2372330524102404852,
                0534330209902435677,
                7066324924582287843,
                0630363884335538722,
                9
            ])
            .unwrap(),
            N
        );
    }

    #[test]
    fn test_to_base_be() {
        assert_eq!(
            Uint::<64, 1>::from(123456789)
                .to_base_be(10)
                .collect::<Vec<_>>(),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
        assert_eq!(
            N.to_base_be(10000000000000000000_u64).collect::<Vec<_>>(),
            vec![
                9,
                0630363884335538722,
                7066324924582287843,
                0534330209902435677,
                2372330524102404852
            ]
        );
    }

    #[test]
    fn test_from_base_be() {
        assert_eq!(
            Uint::<64, 1>::from_base_be(10, [1, 2, 3, 4, 5, 6, 7, 8, 9]),
            Ok(Uint::<64, 1>::from(123456789))
        );
        assert_eq!(
            Uint::<256, 4>::from_base_be(10000000000000000000_u64, [
                9,
                0630363884335538722,
                7066324924582287843,
                0534330209902435677,
                2372330524102404852
            ])
            .unwrap(),
            N
        );
    }

    #[test]
    fn test_from_base_be_overflow() {
        assert_eq!(
            Uint::<0, 0>::from_base_be(10, [].into_iter()),
            Ok(Uint::<0, 0>::ZERO)
        );
        assert_eq!(
            Uint::<0, 0>::from_base_be(10, [0].into_iter()),
            Ok(Uint::<0, 0>::ZERO)
        );
        assert_eq!(
            Uint::<0, 0>::from_base_be(10, [1].into_iter()),
            Err(BaseConvertError::Overflow)
        );
        assert_eq!(
            Uint::<1, 1>::from_base_be(10, [1, 0, 0].into_iter()),
            Err(BaseConvertError::Overflow)
        )
    }
}
