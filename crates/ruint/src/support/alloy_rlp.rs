//! Support for the [`alloy-rlp`](https://crates.io/crates/alloy-rlp) crate.
#![cfg(feature = "alloy-rlp")]
#![cfg_attr(docsrs, doc(cfg(feature = "alloy-rlp")))]

use crate::Uint;
use alloy_rlp::{BufMut, Decodable, DecodeError, Encodable, Header};
use core::mem::size_of;

/// Allows a [`Uint`] to be serialized as RLP.
///
/// See <https://eth.wiki/en/fundamentals/rlp>
impl<const BITS: usize, const LIMBS: usize> Encodable for Uint<BITS, LIMBS> {
    fn length(&self) -> usize {
        let bits = self.bit_len();
        match bits {
            n if n <= 7 => 1,
            n if n <= 55 * 8 => 1 + (n + 7) / 8,
            n => {
                let bytes = (n + 7) / 8;
                let len_bytes = size_of::<usize>() - bytes.leading_zeros() as usize / 8;
                1 + len_bytes + bytes
            }
        }
    }

    fn encode(&self, out: &mut dyn BufMut) {
        match self.bit_len() {
            0 => out.put_u8(0x80),
            n if n <= 7 => {
                #[allow(clippy::cast_possible_truncation)] // self < 128
                out.put_u8(self.as_limbs()[0] as u8);
            }
            n if n <= 55 * 8 => {
                let bytes = self.to_be_bytes_vec();
                let bytes = trim_leading_zeros(&bytes);
                #[allow(clippy::cast_possible_truncation)] // bytes.len() < 56 < 256
                out.put_u8(0x80 + bytes.len() as u8);
                out.put_slice(bytes);
            }
            _ => {
                let bytes = self.to_be_bytes_vec();
                let bytes = trim_leading_zeros(&bytes);
                let length_bytes = bytes.len().to_be_bytes();
                let length_bytes = trim_leading_zeros(&length_bytes);
                #[allow(clippy::cast_possible_truncation)] // length_bytes.len() <= 8
                out.put_u8(0xb7 + length_bytes.len() as u8);
                out.put_slice(length_bytes);
                out.put_slice(bytes);
            }
        }
    }
}

/// Allows a [`Uint`] to be deserialized from RLP.
///
/// See <https://eth.wiki/en/fundamentals/rlp>
impl<const BITS: usize, const LIMBS: usize> Decodable for Uint<BITS, LIMBS> {
    fn decode(buf: &mut &[u8]) -> Result<Self, DecodeError> {
        let header = Header::decode(buf)?;
        if header.list {
            return Err(DecodeError::UnexpectedList);
        }
        let bytes = &buf[..header.payload_length];
        *buf = &buf[header.payload_length..];
        Self::try_from_be_slice(bytes).ok_or(DecodeError::Overflow)
    }
}

fn trim_leading_zeros(bytes: &[u8]) -> &[u8] {
    let zeros = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
    &bytes[zeros..]
}

alloy_rlp::impl_max_encoded_len!(crate::aliases::U256, alloy_rlp::length_of_length(32) + 32);

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        aliases::{U0, U256},
        const_for, nlimbs,
    };
    use hex_literal::hex;
    use proptest::proptest;

    fn encode<T: Encodable>(value: T) -> Vec<u8> {
        let mut buf = vec![];
        value.encode(&mut buf);
        buf
    }

    #[test]
    fn test_rlp() {
        // See <https://github.com/paritytech/parity-common/blob/436cb0827f0e3238ccb80d7d453f756d126c0615/rlp/tests/tests.rs#L214>
        assert_eq!(encode(U0::from(0))[..], hex!("80"));
        assert_eq!(encode(U256::from(0))[..], hex!("80"));
        assert_eq!(encode(U256::from(15))[..], hex!("0f"));
        assert_eq!(encode(U256::from(1024))[..], hex!("820400"));
        assert_eq!(encode(U256::from(0x1234_5678))[..], hex!("8412345678"));
    }

    #[test]
    fn test_roundtrip() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            proptest!(|(value: Uint<BITS, LIMBS>)| {
                let serialized = encode(value);
                assert_eq!(serialized.len(), value.length());
                let mut reader = &serialized[..];
                let deserialized = Uint::decode(&mut reader).unwrap();
                assert_eq!(reader.len(), 0);
                assert_eq!(value, deserialized);
            });
        });
    }

    #[test]
    #[cfg(feature = "rlp")]
    fn test_rlp_alloy_rlp_compat() {
        use rlp::Encodable;

        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            proptest!(|(value: Uint<BITS, LIMBS>)| {
                let serialized = encode(value);
                let serialized_rlp = value.rlp_bytes();
                assert_eq!(serialized, serialized_rlp);
                // We already test that they can deserialize from this.
            });
        });
    }
}
