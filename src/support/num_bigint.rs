//! Support for the [`num-bigint`](https://crates.io/crates/num-bigint) crate.
#![cfg(feature = "num-bigint")]

use crate::{from::FromUintError, Uint};
use core::marker::PhantomData;
use num_bigint::{BigInt, BigUint};

impl<const BITS: usize, const LIMBS: usize> TryFrom<BigUint> for Uint<BITS, LIMBS> {
    type Error = FromUintError<BITS, Self>;

    fn try_from(value: BigUint) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl<const BITS: usize, const LIMBS: usize> TryFrom<&BigUint> for Uint<BITS, LIMBS> {
    type Error = FromUintError<BITS, Self>;

    fn try_from(value: &BigUint) -> Result<Self, Self::Error> {
        Self::checked_from_limbs_slice(value.to_u64_digits().as_slice())
            .ok_or(FromUintError::Overflow(PhantomData))
    }
}

impl<const BITS: usize, const LIMBS: usize> From<Uint<BITS, LIMBS>> for BigUint {
    fn from(value: Uint<BITS, LIMBS>) -> Self {
        Self::from(&value)
    }
}

impl<const BITS: usize, const LIMBS: usize> From<&Uint<BITS, LIMBS>> for BigUint {
    fn from(value: &Uint<BITS, LIMBS>) -> Self {
        Self::from_bytes_le(&value.as_le_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{const_for, nlimbs};
    use proptest::proptest;

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_roundtrip() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(value: U)| {
                let big: BigUint = value.into();
                let back: U = big.try_into().unwrap();
                assert_eq!(back, value);
            });
        });
    }
}
