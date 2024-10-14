//! Support for the [`der`](https://crates.io/crates/der) crate.
#![cfg(feature = "der")]
#![cfg_attr(docsrs, doc(cfg(feature = "der")))]

use crate::Uint;
use der::{
    asn1::{AnyRef, IntRef, UintRef},
    DecodeValue, EncodeValue, Error, FixedTag, Header, Length, Reader, Result, Tag, ValueOrd,
    Writer,
};
use std::cmp::Ordering;

impl<const BITS: usize, const LIMBS: usize> ValueOrd for Uint<BITS, LIMBS> {
    fn value_cmp(&self, other: &Self) -> Result<Ordering> {
        // DER encoding corresponds to integer comparison.
        Ok(self.cmp(other))
    }
}

impl<const BITS: usize, const LIMBS: usize> FixedTag for Uint<BITS, LIMBS> {
    const TAG: Tag = Tag::Integer;
}

impl<const BITS: usize, const LIMBS: usize> EncodeValue for Uint<BITS, LIMBS> {
    fn value_len(&self) -> Result<Length> {
        (1 + self.bit_len() / 8).try_into()
    }

    fn encode_value(&self, writer: &mut impl Writer) -> Result<()> {
        // Write bytes in big-endian order without leading zeros.
        let bytes = self.to_be_bytes_trimmed_vec();
        // Add leading `0x00` byte if the first byte has the highest bit set.
        // or if the sequence is empty.
        if bytes.first().copied().unwrap_or(0x80) >= 0x80 {
            writer.write_byte(0x00)?;
        }
        writer.write(&bytes)
    }
}

impl<'a, const BITS: usize, const LIMBS: usize> DecodeValue<'a> for Uint<BITS, LIMBS> {
    fn decode_value<R: Reader<'a>>(reader: &mut R, header: Header) -> Result<Self> {
        if header.length > Length::try_from(Self::BYTES + 1)? {
            return Err(Self::TAG.non_canonical_error());
        }
        let bytes = reader.read_vec(header.length)?;
        let bytes = match bytes.as_slice() {
            [] => Err(Tag::Integer.length_error()),
            [0, byte, ..] if *byte < 0x80 => Err(Tag::Integer.non_canonical_error()),
            [0, rest @ ..] => Ok(rest),
            [byte, ..] if *byte >= 0x80 => Err(Tag::Integer.value_error()),
            bytes => Ok(bytes),
        }?;
        Self::try_from_be_slice(bytes).ok_or_else(|| Tag::Integer.non_canonical_error())
    }
}

impl<const BITS: usize, const LIMBS: usize> TryFrom<AnyRef<'_>> for Uint<BITS, LIMBS> {
    type Error = Error;

    fn try_from(any: AnyRef<'_>) -> Result<Self> {
        any.decode_as()
    }
}

impl<const BITS: usize, const LIMBS: usize> TryFrom<IntRef<'_>> for Uint<BITS, LIMBS> {
    type Error = Error;

    fn try_from(any: IntRef<'_>) -> Result<Self> {
        any.decode_as()
    }
}

impl<const BITS: usize, const LIMBS: usize> TryFrom<UintRef<'_>> for Uint<BITS, LIMBS> {
    type Error = Error;

    fn try_from(any: UintRef<'_>) -> Result<Self> {
        any.decode_as()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{const_for, nlimbs};
    use der::{Decode, Encode};
    use proptest::proptest;

    #[test]
    fn test_der_roundtrip() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            proptest!(|(value: Uint<BITS, LIMBS>)| {
                let serialized = value.to_der().unwrap();
                let deserialized = Uint::from_der(&serialized).unwrap();
                assert_eq!(value, deserialized);
            });
        });
    }

    #[test]
    fn test_u128_equiv() {
        proptest!(|(value: u128)| {
            let uint = Uint::<128, 2>::from(value);
            let serialized1 = value.to_der().unwrap();
            let serialized2 = uint.to_der().unwrap();
            assert_eq!(serialized1, serialized2);
        });
    }
}
