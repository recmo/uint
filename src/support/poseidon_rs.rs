//! ⚠️ Support for the [`poseidon-rs`](https://crates.io/crates/poseidon-rs) crate.
//!
//! **Warning.** This is deprecated and will be removed soon.
#![cfg(feature = "poseidon-rs")]

use crate::aliases::U256;
use ff_ce::PrimeField;
use poseidon_rs::{Fr, FrRepr};

const MODULUS: U256 = U256::from_limbs([
    0x43e1_f593_f000_0001,
    0x2833_e848_79b9_7091,
    0xb850_45b6_8181_585d,
    0x3064_4e72_e131_a029,
]);

impl From<Fr> for U256 {
    fn from(n: Fr) -> Self {
        Self::from_limbs_slice(FrRepr::from(n).as_ref())
    }
}

impl From<&Fr> for U256 {
    fn from(n: &Fr) -> Self {
        (*n).into()
    }
}

#[allow(clippy::fallible_impl_from)] // This is deprecated anyway
impl From<U256> for Fr {
    fn from(mut n: U256) -> Self {
        n %= MODULUS;
        Self::from_repr(FrRepr(n.into_limbs())).unwrap()
    }
}

#[allow(clippy::fallible_impl_from)] // This is deprecated anyway
impl From<&U256> for Fr {
    fn from(n: &U256) -> Self {
        (*n).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_roundtrip() {
        proptest!(|(n: U256)| {
            let fr: Fr = n.into();
            let back: U256 = fr.into();
            assert_eq!(back, n % MODULUS);
        });
    }
}
