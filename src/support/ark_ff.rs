//! Support for the [`ark-ff`](https://crates.io/crates/ark-ff) crate.
#![cfg(feature = "ark-ff")]

use crate::{from::ToUintError, Uint};
use ark_ff::biginteger::*;

macro_rules! impl_from_ark {
    ($ark:ty, $bits:expr, $limbs:expr) => {
        impl From<$ark> for Uint<$bits, $limbs> {
            fn from(value: $ark) -> Self {
                Self::from_limbs(value.0)
            }
        }

        impl From<&$ark> for Uint<$bits, $limbs> {
            fn from(value: &$ark) -> Self {
                Self::from_limbs(value.0)
            }
        }

        impl From<Uint<$bits, $limbs>> for $ark {
            fn from(value: Uint<$bits, $limbs>) -> Self {
                Self(value.into_limbs())
            }
        }

        impl From<&Uint<$bits, $limbs>> for $ark {
            fn from(value: &Uint<$bits, $limbs>) -> Self {
                Self(value.into_limbs())
            }
        }
    };
}

impl_from_ark!(BigInteger64, 64, 1);
impl_from_ark!(BigInteger128, 128, 2);
impl_from_ark!(BigInteger256, 256, 4);
impl_from_ark!(BigInteger320, 320, 5);
impl_from_ark!(BigInteger384, 384, 6);
impl_from_ark!(BigInteger448, 448, 7);
impl_from_ark!(BigInteger768, 768, 12);
impl_from_ark!(BigInteger832, 832, 13);

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

    macro_rules! test_roundtrip {
        ($ark:ty, $bits:expr, $limbs:expr) => {
            proptest!(|(value: Uint<$bits, $limbs>)| {
                let ark: $ark = value.into();
                let back: Uint<$bits, $limbs> = ark.into();
                assert_eq!(back, value);
            });
        }
    }

    #[test]
    fn test_roundtrip() {
        test_roundtrip!(BigInteger64, 64, 1);
        test_roundtrip!(BigInteger128, 128, 2);
        test_roundtrip!(BigInteger256, 256, 4);
        test_roundtrip!(BigInteger320, 320, 5);
        test_roundtrip!(BigInteger384, 384, 6);
        test_roundtrip!(BigInteger448, 448, 7);
        test_roundtrip!(BigInteger768, 768, 12);
        test_roundtrip!(BigInteger832, 832, 13);
    }
}
