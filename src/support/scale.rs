//! Support for the [`parity-scale-codec`](https://crates.io/crates/parity-scale-codec) crate.
#![cfg(feature = "parity-scale-codec")]
#![cfg_attr(has_doc_cfg, doc(cfg(feature = "parity-scale-codec")))]

use crate::Uint;
use parity_scale_codec::{Decode, Encode, Error, Input, MaxEncodedLen};

// FEATURE: Implement compact encoding

impl<const BITS: usize, const LIMBS: usize> Encode for Uint<BITS, LIMBS> {
    fn size_hint(&self) -> usize {
        Self::BYTES
    }

    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
        self.as_le_bytes().using_encoded(f)
    }
}

impl<const BITS: usize, const LIMBS: usize> MaxEncodedLen for Uint<BITS, LIMBS> {
    fn max_encoded_len() -> usize {
        core::mem::size_of::<Self>()
    }
}

impl<const BITS: usize, const LIMBS: usize> Decode for Uint<BITS, LIMBS> {
    fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
        Decode::decode(input).and_then(|b: Vec<_>| {
            Self::try_from_le_slice(&b).ok_or(Error::from("value is larger than fits the Uint"))
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{const_for, nlimbs, Uint};
    use parity_scale_codec::{Decode, Encode};
    use proptest::proptest;

    #[test]
    fn test_scale() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            proptest!(|(value: Uint<BITS, LIMBS>)| {
                let serialized = Encode::encode(&value);
                let deserialized = <Uint::<BITS, LIMBS> as Decode>::decode(&mut serialized.as_slice()).unwrap();
                assert_eq!(value, deserialized);
            });
        });
    }
}
