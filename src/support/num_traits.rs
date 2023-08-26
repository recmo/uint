//! Support for the [`num-traits`](https://crates.io/crates/num-traits) crate.
#![cfg(feature = "num-traits")]
#![cfg_attr(docsrs, doc(cfg(feature = "num-traits")))]

use crate::Uint;
use num_traits::{
    bounds::Bounded,
    ops::bytes::{FromBytes, ToBytes},
    One, Zero,
};

impl<const BITS: usize, const LIMBS: usize> Zero for Uint<BITS, LIMBS> {
    fn zero() -> Self {
        Self::ZERO
    }

    fn is_zero(&self) -> bool {
        self.is_zero()
    }
}

impl<const BITS: usize, const LIMBS: usize> One for Uint<BITS, LIMBS> {
    fn one() -> Self {
        Self::one()
    }
}

impl<const BITS: usize, const LIMBS: usize> Bounded for Uint<BITS, LIMBS> {
    fn min_value() -> Self {
        Self::ZERO
    }

    fn max_value() -> Self {
        Self::MAX
    }
}

impl<const BITS: usize, const LIMBS: usize> FromBytes for Uint<BITS, LIMBS> {
    type Bytes = [u8];

    fn from_le_bytes(bytes: &[u8]) -> Self {
        Self::try_from_le_slice(bytes).unwrap()
    }

    fn from_be_bytes(bytes: &[u8]) -> Self {
        Self::try_from_be_slice(bytes).unwrap()
    }
}

impl<const BITS: usize, const LIMBS: usize> ToBytes for Uint<BITS, LIMBS> {
    type Bytes = Vec<u8>;

    fn to_le_bytes(&self) -> Self::Bytes {
        self.to_le_bytes_vec()
    }

    fn to_be_bytes(&self) -> Self::Bytes {
        self.to_be_bytes_vec()
    }
}

// TODO: PrimInt
