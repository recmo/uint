//! Support for the [`rlp`](https://crates.io/crates/rlp) crate.
#![cfg(feature = "rlp")]

use crate::Uint;
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};

/// Allows a [`Uint`] to be serialized as RLP.
///
/// See <https://eth.wiki/en/fundamentals/rlp>
impl<const BITS: usize, const LIMBS: usize> Encodable for Uint<BITS, LIMBS> {
    fn rlp_append(&self, s: &mut RlpStream) {
        let bytes = self.to_be_bytes_vec();
        // Strip most-significant zeros.
        let bytes = trim_leading_zeros(&bytes);
        bytes.rlp_append(s);
    }
}

/// Allows a [`Uint`] to be deserialized from RLP.
///
/// See <https://eth.wiki/en/fundamentals/rlp>
impl<const BITS: usize, const LIMBS: usize> Decodable for Uint<BITS, LIMBS> {
    fn decode(s: &Rlp) -> Result<Self, DecoderError> {
        Self::try_from_be_slice(s.data()?).ok_or(DecoderError::Custom(
            "RLP integer value too large for Uint.",
        ))
    }
}

fn trim_leading_zeros(bytes: &[u8]) -> &[u8] {
    let zeros = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
    &bytes[zeros..]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        aliases::{U0, U256},
        const_for, nlimbs,
    };
    use hex_literal::hex;
    use proptest::proptest;

    #[test]
    fn test_rlp() {
        // See <https://github.com/paritytech/parity-common/blob/436cb0827f0e3238ccb80d7d453f756d126c0615/rlp/tests/tests.rs#L214>
        assert_eq!(U0::from(0).rlp_bytes()[..], hex!("80"));
        assert_eq!(U256::from(0).rlp_bytes()[..], hex!("80"));
        assert_eq!(U256::from(15).rlp_bytes()[..], hex!("0f"));
        assert_eq!(U256::from(1024).rlp_bytes()[..], hex!("820400"));
        assert_eq!(U256::from(0x1234_5678).rlp_bytes()[..], hex!("8412345678"));
    }

    #[test]
    fn test_roundtrip() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            proptest!(|(value: Uint<BITS, LIMBS>)| {
                let serialized = value.rlp_bytes();
                let deserialized = Uint::decode(&Rlp::new(&serialized)).unwrap();
                assert_eq!(value, deserialized);
            });
        });
    }
}
