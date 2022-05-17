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

use crate::Uint;
use core::{any::type_name, convert::TryFrom, fmt::Display, marker::PhantomData};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq, Hash)]
pub enum ToUintError {
    #[error("Value is too large for Uint<{0}>")]
    ValueTooLarge(usize),

    #[error("Negative values can not be represented as Uint<{0}>")]
    ValueNegative(usize),

    #[error("'Not a number' (NaN) not be represented as Uint<{0}>")]
    NotANumber(usize),
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq, Hash)]
pub enum FromUintError<const BITS: usize, T> {
    #[error("Uint<{}> value is too large for {}", BITS, type_name::<T>())]
    Overflow(PhantomData<T>),
}

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// # Panics
    /// Panics if the conversion fails, for example if the value is too large
    /// for the bit-size of the [`Uint`]. The panic will be attributed to the
    /// call site.
    #[must_use]
    #[track_caller]
    pub fn from<T>(value: T) -> Self
    where
        Self: TryFrom<T>,
        <Self as TryFrom<T>>::Error: Display,
    {
        match Self::try_from(value) {
            Ok(uint) => uint,
            Err(e) => panic!("Uint conversion error: {}", e),
        }
    }
}

// u64 is a single limb, so this is the base case
impl<const BITS: usize, const LIMBS: usize> TryFrom<u64> for Uint<BITS, LIMBS> {
    type Error = ToUintError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if Self::LIMBS <= 1 {
            if value > Self::MASK {
                return Err(ToUintError::ValueTooLarge(BITS));
            }
            if Self::LIMBS == 0 {
                return Ok(Self::MIN);
            }
        }
        let mut limbs = [0; LIMBS];
        limbs[0] = value;
        Ok(Self::from_limbs(limbs))
    }
}

// u128 version is handled specially in because it covers two limbs.
impl<const BITS: usize, const LIMBS: usize> TryFrom<u128> for Uint<BITS, LIMBS> {
    type Error = ToUintError;

    #[allow(clippy::cast_lossless)]
    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value <= u64::MAX as u128 {
            return Self::try_from(value as u64);
        }
        if Self::LIMBS < 2 {
            return Err(ToUintError::ValueTooLarge(BITS));
        }
        let lo = value as u64;
        let hi = (value >> 64) as u64;
        if Self::LIMBS == 2 && hi > Self::MASK {
            return Err(ToUintError::ValueTooLarge(BITS));
        }
        let mut limbs = [0; LIMBS];
        limbs[0] = lo;
        limbs[1] = hi;
        Ok(Self::from_limbs(limbs))
    }
}

// Unsigned int version upcast to u64
macro_rules! impl_from_unsigned_int {
    ($uint:ty) => {
        impl<const BITS: usize, const LIMBS: usize> TryFrom<$uint> for Uint<BITS, LIMBS> {
            type Error = ToUintError;

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
        impl<const BITS: usize, const LIMBS: usize> TryFrom<$int> for Uint<BITS, LIMBS> {
            type Error = ToUintError;

            fn try_from(value: $int) -> Result<Self, Self::Error> {
                if value < 0 {
                    Err(Self::Error::ValueNegative(BITS))
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

// TODO: Make this a const trait using
// #![feature(const_float_classify)]
// #![feature(const_fn_floating_point_arithmetic)]
// #![feature(const_float_bits_conv)]
// and more.
impl<const BITS: usize, const LIMBS: usize> TryFrom<f64> for Uint<BITS, LIMBS> {
    type Error = ToUintError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_nan() {
            return Err(ToUintError::NotANumber(BITS));
        }
        if value < 0.0 {
            return Err(ToUintError::ValueNegative(BITS));
        }
        #[allow(clippy::cast_precision_loss)] // BITS is small-ish
        if value >= (Self::BITS as f64).exp2() {
            return Err(ToUintError::ValueTooLarge(BITS));
        }
        if value < 0.5 {
            return Ok(Self::ZERO);
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
        let fraction = bits & 0x000f_ffff_ffff_ffff;
        let mantissa = 0x0010_0000_0000_0000 | fraction;

        // Convert mantissa * 2^(exponent - 52) to Uint
        #[allow(clippy::cast_possible_truncation)] // exponent is small-ish
        if exponent as usize > Self::BITS + 52 {
            return Err(ToUintError::ValueTooLarge(BITS));
        }
        if exponent <= 52 {
            // Truncate mantissa
            Self::try_from(mantissa >> (52 - exponent))
        } else {
            let _mantissa = Self::try_from(mantissa)?;
            todo!() // mantissa << (exponent - 52)
        }
    }
}

impl<const BITS: usize, const LIMBS: usize> TryFrom<f32> for Uint<BITS, LIMBS> {
    type Error = ToUintError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        #[allow(clippy::cast_lossless)]
        Self::try_from(value as f64)
    }
}

// Convert Uint to integer types
//

// Required because a generic rule violates the orphan rule
macro_rules! to_value_to_ref {
    ($t:ty) => {
        impl<const BITS: usize, const LIMBS: usize> TryFrom<Uint<BITS, LIMBS>> for $t {
            type Error = FromUintError<BITS, Self>;

            fn try_from(value: Uint<BITS, LIMBS>) -> Result<Self, Self::Error> {
                Self::try_from(&value)
            }
        }
    };
}

to_value_to_ref!(bool);

impl<const BITS: usize, const LIMBS: usize> TryFrom<&Uint<BITS, LIMBS>> for bool {
    type Error = FromUintError<BITS, Self>;

    fn try_from(value: &Uint<BITS, LIMBS>) -> Result<Self, Self::Error> {
        if BITS == 0 {
            return Ok(false);
        }
        if value.bit_len() > 1 {
            return Err(Self::Error::Overflow(PhantomData));
        }
        Ok(value.as_limbs()[0] != 0)
    }
}

macro_rules! to_int {
    ($int:ty, $bits:expr) => {
        to_value_to_ref!($int);

        impl<const BITS: usize, const LIMBS: usize> TryFrom<&Uint<BITS, LIMBS>> for $int {
            type Error = FromUintError<BITS, Self>;

            fn try_from(value: &Uint<BITS, LIMBS>) -> Result<Self, Self::Error> {
                if BITS == 0 {
                    return Ok(0);
                }
                if value.bit_len() > $bits {
                    return Err(Self::Error::Overflow(PhantomData));
                }
                Ok(value.as_limbs()[0] as $int)
            }
        }
    };
}

to_int!(i8, 7);
to_int!(u8, 8);
to_int!(i16, 15);
to_int!(u16, 16);
to_int!(i32, 31);
to_int!(u32, 32);
to_int!(i64, 63);
to_int!(u64, 64);

to_value_to_ref!(i128);

impl<const BITS: usize, const LIMBS: usize> TryFrom<&Uint<BITS, LIMBS>> for i128 {
    type Error = FromUintError<BITS, Self>;

    fn try_from(value: &Uint<BITS, LIMBS>) -> Result<Self, Self::Error> {
        if BITS == 0 {
            return Ok(0);
        }
        if value.bit_len() > 127 {
            return Err(Self::Error::Overflow(PhantomData));
        }
        let mut result: i128 = value.as_limbs()[0] as i128;
        result |= (value.as_limbs()[1] as i128) << 64;
        Ok(result)
    }
}

to_value_to_ref!(u128);

impl<const BITS: usize, const LIMBS: usize> TryFrom<&Uint<BITS, LIMBS>> for u128 {
    type Error = FromUintError<BITS, Self>;

    fn try_from(value: &Uint<BITS, LIMBS>) -> Result<Self, Self::Error> {
        if BITS == 0 {
            return Ok(0);
        }
        if value.bit_len() > 128 {
            return Err(Self::Error::Overflow(PhantomData));
        }
        let mut result: u128 = value.as_limbs()[0] as u128;
        result |= (value.as_limbs()[1] as u128) << 64;
        Ok(result)
    }
}

// Convert Uint to floating point
//

impl<const BITS: usize, const LIMBS: usize> From<Uint<BITS, LIMBS>> for f32 {
    fn from(value: Uint<BITS, LIMBS>) -> Self {
        Self::from(&value)
    }
}

impl<const BITS: usize, const LIMBS: usize> From<&Uint<BITS, LIMBS>> for f32 {
    /// Convert to IEEE-754 single-precision floating point number.
    ///
    /// Returns `f32::INFINITY` if the value is too large to represent.
    fn from(value: &Uint<BITS, LIMBS>) -> Self {
        let (bits, exponent) = value.most_significant_bits();
        (bits as Self) * (exponent as Self).exp2()
    }
}

impl<const BITS: usize, const LIMBS: usize> From<Uint<BITS, LIMBS>> for f64 {
    fn from(value: Uint<BITS, LIMBS>) -> Self {
        Self::from(&value)
    }
}

impl<const BITS: usize, const LIMBS: usize> From<&Uint<BITS, LIMBS>> for f64 {
    /// Convert to IEEE-754 double-precision floating point number.
    ///
    /// Returns `f64::INFINITY` if the value is too large to represent.
    fn from(value: &Uint<BITS, LIMBS>) -> Self {
        let (bits, exponent) = value.most_significant_bits();
        (bits as Self) * (exponent as Self).exp2()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{const_for, nlimbs};

    #[test]
    fn test_u64() {
        assert_eq!(Uint::<0, 0>::try_from(0_u64), Ok(Uint::ZERO));
        assert_eq!(
            Uint::<0, 0>::try_from(1_u64),
            Err(ToUintError::ValueTooLarge(0))
        );
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            assert_eq!(Uint::<BITS, LIMBS>::try_from(0_u64), Ok(Uint::ZERO));
            assert_eq!(Uint::<BITS, LIMBS>::try_from(1_u64).unwrap().as_limbs()[0], 1);
        });
    }

    #[test]
    fn test_f64() {
        assert_eq!(Uint::<0, 0>::try_from(0.0_f64), Ok(Uint::ZERO));
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            assert_eq!(Uint::<BITS, LIMBS>::try_from(0.0_f64), Ok(Uint::ZERO));
            assert_eq!(Uint::<BITS, LIMBS>::try_from(1.0_f64).unwrap().as_limbs()[0], 1);
        });
        assert_eq!(
            Uint::<7, 1>::try_from(123.499_f64),
            Ok(Uint::from_limbs([123]))
        );
        assert_eq!(
            Uint::<7, 1>::try_from(123.500_f64),
            Ok(Uint::from_limbs([124]))
        );
    }
}
