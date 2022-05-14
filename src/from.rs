use crate::{nlimbs, Uint};
use core::convert::TryFrom;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq, Hash)]
pub enum UintConversionError {
    #[error("Value is too large for Uint<{0}>")]
    ValueTooLarge(usize),

    #[error("Negative values can not be represented as Uint<{0}>")]
    ValueNegative(usize),
}

impl<const BITS: usize> TryFrom<i64> for Uint<BITS>
where
    [(); nlimbs(BITS)]:,
{
    type Error = UintConversionError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(UintConversionError::ValueNegative(BITS))
        } else {
            Self::try_from(value as u64)
        }
    }
}

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::repeat;

    #[test]
    fn from_zeros() {
        repeat!({
            assert_eq!(Uint::<N>::try_from(0_u64), Ok(Uint::zero()));
        });
    }

    #[test]
    fn test_ones() {
        repeat!(non_zero, {
            assert_eq!(Uint::<N>::try_from(1_u64), Ok(Uint::one()));
        });
    }
}
