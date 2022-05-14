// TODO: It would be nice to impl From<_> as well, but then the generic
// implementation `impl<T: Into<U>, U> TryFrom<U> for T` conflicts with our
// own implementation. This means we can only implement one.
// In principle this can be worked around by `specialization`, but that
// triggers other compiler issues at the moment.

// impl<T, const BITS: usize> From<T> for Uint<BITS>
// where
//     [(); nlimbs(BITS)]:,
//     Uint<BITS>: TryFrom<T>,
// {
//     fn from(t: T) -> Self {
//         Self::try_from(t).unwrap()
//     }
// }

use crate::{nlimbs, Uint};
use core::convert::TryFrom;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq, Hash)]
pub enum UintConversionError {
    #[error("Value is too large for Uint<{0}>")]
    ValueTooLarge(usize),

    #[error("Negative values can not be represented as Uint<{0}>")]
    ValueNegative(usize),

    #[error("'Not a number' (NaN) not be represented as Uint<{0}>")]
    NotANumber(usize),
}

// u64 is a single limb, so this is the base case
impl<const BITS: usize> TryFrom<u64> for Uint<BITS>
where
    [(); nlimbs(BITS)]:,
{
    type Error = UintConversionError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if Self::LIMBS <= 1 {
            if value > Self::MASK {
                return Err(UintConversionError::ValueTooLarge(BITS));
            }
            if Self::LIMBS == 0 {
                return Ok(Self::zero());
            }
        }
        let mut limbs = [0; nlimbs(BITS)];
        limbs[0] = value;
        Ok(Self::from_limbs(limbs))
    }
}

// u128 version is handled specially in because it covers two limbs.
impl<const BITS: usize> TryFrom<u128> for Uint<BITS>
where
    [(); nlimbs(BITS)]:,
{
    type Error = UintConversionError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value <= u64::MAX as u128 {
            return Self::try_from(value as u64);
        }
        if Self::LIMBS < 2 {
            return Err(UintConversionError::ValueTooLarge(BITS));
        }
        let lo = value as u64;
        let hi = (value >> 64) as u64;
        if Self::LIMBS == 2 && hi > Self::MASK {
            return Err(UintConversionError::ValueTooLarge(BITS));
        }
        let mut limbs = [0; nlimbs(BITS)];
        limbs[0] = lo;
        limbs[1] = hi;
        Ok(Self::from_limbs(limbs))
    }
}

// Unsigned int version upcast to u64
macro_rules! impl_from_unsigned_int {
    ($uint:ty) => {
        impl<const BITS: usize> TryFrom<$uint> for Uint<BITS>
        where
            [(); nlimbs(BITS)]:,
        {
            type Error = UintConversionError;

            fn try_from(value: $uint) -> Result<Self, Self::Error> {
                Self::try_from(value as u64)
            }
        }
    };
}

impl_from_unsigned_int!(u8);
impl_from_unsigned_int!(u16);
impl_from_unsigned_int!(u32);
impl_from_unsigned_int!(usize);

// Signed int version check for positive and delegate to the corresponding
// `uint`.
macro_rules! impl_from_signed_int {
    ($int:ty, $uint:ty) => {
        impl<const BITS: usize> TryFrom<$int> for Uint<BITS>
        where
            [(); nlimbs(BITS)]:,
        {
            type Error = UintConversionError;

            fn try_from(value: $int) -> Result<Self, Self::Error> {
                if value < 0 {
                    Err(UintConversionError::ValueNegative(BITS))
                } else {
                    Self::try_from(value as $uint)
                }
            }
        }
    };
}

impl_from_signed_int!(i8, u8);
impl_from_signed_int!(i16, u16);
impl_from_signed_int!(i32, u32);
impl_from_signed_int!(i64, u64);
impl_from_signed_int!(i128, u128);
impl_from_signed_int!(isize, usize);

impl<const BITS: usize> TryFrom<f64> for Uint<BITS>
where
    [(); nlimbs(BITS)]:,
{
    type Error = UintConversionError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_nan() {
            return Err(UintConversionError::NotANumber(BITS));
        }
        if value < 0.0 {
            return Err(UintConversionError::ValueNegative(BITS));
        }
        if value >= (Self::BITS as f64).exp2() {
            return Err(UintConversionError::ValueTooLarge(BITS));
        }
        if value < 0.5 {
            return Ok(Self::zero());
        }
        // All non-normal cases should have been handled above
        assert!(value.is_normal());

        // Add offset to round to nearest integer.
        let value = value + 0.5;

        // Parse IEEE-754 double
        // Sign should be zero, exponent should be >= 0.
        let bits = value.to_bits();
        let sign = bits >> 63;
        assert!(sign == 0);
        let biased_exponent = (bits >> 52) & 0x7ff;
        assert!(biased_exponent >= 1023);
        let exponent = biased_exponent - 1023;
        let fraction = bits & 0xfffffffffffff;
        let mantissa = 0x10000000000000 | fraction;

        // Convert mantissa * 2^(exponent - 52) to Uint
        if exponent as usize > Self::BITS + 52 {
            return Err(UintConversionError::ValueTooLarge(BITS));
        }
        if exponent <= 52 {
            // Truncate mantissa
            Self::try_from(mantissa >> (52 - exponent))
        } else {
            let mantissa = Self::try_from(mantissa)?;
            todo!() // mantissa << (exponent - 52)
        }
    }
}

impl<const BITS: usize> TryFrom<f32> for Uint<BITS>
where
    [(); nlimbs(BITS)]:,
{
    type Error = UintConversionError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::try_from(value as f64)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::repeat;

    #[test]
    fn test_u64() {
        assert_eq!(Uint::<0>::try_from(0_u64), Ok(Uint::zero()));
        assert_eq!(
            Uint::<0>::try_from(1_u64),
            Err(UintConversionError::ValueTooLarge(0))
        );
        repeat!(non_zero, {
            assert_eq!(Uint::<N>::try_from(0_u64), Ok(Uint::zero()));
            assert_eq!(Uint::<N>::try_from(1_u64), Ok(Uint::one()));
        });
    }

    #[test]
    fn test_f64() {
        assert_eq!(Uint::<0>::try_from(0.0), Ok(Uint::zero()));
        repeat!(non_zero, {
            assert_eq!(Uint::<N>::try_from(0.0), Ok(Uint::zero()));
            assert_eq!(Uint::<N>::try_from(1.0), Ok(Uint::one()));
        });
        assert_eq!(Uint::<7>::try_from(123.499), Ok(Uint::from_limbs([123])));
        assert_eq!(Uint::<7>::try_from(123.500), Ok(Uint::from_limbs([124])));
    }
}
