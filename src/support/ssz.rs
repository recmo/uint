#![cfg(feature = "ssz")]
#![cfg_attr(docsrs, doc(cfg(feature = "ssz")))]
use ssz::{Decode, DecodeError, Encode};

use crate::{nbytes, Uint};

impl<const BITS: usize, const LIMBS: usize> Encode for Uint<BITS, LIMBS> {
    fn is_ssz_fixed_len() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        nbytes(BITS)
    }

    fn ssz_bytes_len(&self) -> usize {
        nbytes(BITS)
    }

    fn ssz_append(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.as_le_bytes());
    }
}

impl<const BITS: usize, const LIMBS: usize> Decode for Uint<BITS, LIMBS> {
    fn is_ssz_fixed_len() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        nbytes(BITS)
    }

    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, DecodeError> {
        Ok(Self::from_le_slice(bytes))
    }
}

#[cfg(test)]
mod tests {
    use proptest::proptest;
    use ruint::{const_for, nlimbs, Uint};

    #[test]
    fn test_ssz_human_readable() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            proptest!(|(value: Uint<BITS, LIMBS>)| {

                let expected = value;
                let encoded = ssz::Encode::as_ssz_bytes(&expected);
                let actual = ssz::Decode::from_ssz_bytes(&encoded).unwrap();
                assert_eq!(expected, actual, "Failed for value: {:?}", value);
            });

        });
    }
}