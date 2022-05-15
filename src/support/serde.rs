#![cfg(feature = "serde")]
use crate::{nbytes, nlimbs, Uint};
use core::fmt::{Formatter, Result as FmtResult};
use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::str;

/// Serialize a [`Uint`] value.
///
/// For human readable formats a `0x` prefixed lower case hex string is used.
/// For binary formats a byte array is used. Leading zeros are included.
impl<const BITS: usize> Serialize for Uint<BITS>
where
    [(); nlimbs(BITS)]:,
    [(); nbytes(BITS)]:,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let bytes = self.to_be_bytes();
        if serializer.is_human_readable() {
            // TODO: Without allocations
            let mut result = String::with_capacity(2 * Self::BYTES + 2);
            result.push_str("0x");
            for byte in bytes.iter() {
                result.push_str(&format!("{:02x}", byte));
            }
            serializer.serialize_str(&result)
        } else {
            // Write as bytes directly
            serializer.serialize_bytes(&bytes[..])
        }
    }
}

/// Deserialize human readable hex strings or byte arrays into hashes.
/// Hex strings can be upper/lower/mixed case, have an optional `0x` prefix, and
/// can be any length. They are interpreted big-endian.
// TODO: Document and test the range of valid inputs.
impl<'de, const BITS: usize> Deserialize<'de> for Uint<BITS>
where
    [(); nlimbs(BITS)]:,
    [(); nbytes(BITS)]:,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(StrVisitor)
        } else {
            deserializer.deserialize_bytes(ByteVisitor)
        }
    }
}

/// Serde Visitor for human readable formats
struct StrVisitor<const BITS: usize>
where
    [(); nlimbs(BITS)]:,
    [(); nbytes(BITS)]:;

impl<'de, const BITS: usize> Visitor<'de> for StrVisitor<BITS>
where
    [(); nlimbs(BITS)]:,
    [(); nbytes(BITS)]:,
{
    type Value = Uint<BITS>;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "a {} byte hex string", nbytes(BITS))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let value = trim_hex_prefix(value);
        let mut limbs = [0; nlimbs(BITS)];
        for (i, chunk) in value.as_bytes().rchunks(16).enumerate() {
            let chunk = str::from_utf8(chunk)
                .map_err(|_| Error::invalid_value(Unexpected::Str(value), &self))?;
            let limb = u64::from_str_radix(chunk, 16)
                .map_err(|_| Error::invalid_value(Unexpected::Str(value), &self))?;
            if limb == 0 {
                continue;
            }
            if i >= nlimbs(BITS) {
                return Err(Error::invalid_value(Unexpected::Str(value), &self));
            }
            limbs[i] = limb;
        }
        if BITS > 0 && limbs[nlimbs(BITS) - 1] > Self::Value::MASK {
            return Err(Error::invalid_value(Unexpected::Str(value), &self));
        }
        Ok(Uint::from_limbs(limbs))
    }
}

/// Serde Visitor for non-human readable formats
struct ByteVisitor<const BITS: usize>
where
    [(); nlimbs(BITS)]:,
    [(); nbytes(BITS)]:;

impl<'de, const BITS: usize> Visitor<'de> for ByteVisitor<BITS>
where
    [(); nlimbs(BITS)]:,
    [(); nbytes(BITS)]:,
{
    type Value = Uint<BITS>;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "{BITS} bits of binary data in big endian order")
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if value.len() != nbytes(BITS) {
            return Err(E::invalid_length(value.len(), &self));
        }
        Uint::try_from_be_bytes(value).ok_or_else(|| {
            E::invalid_value(
                Unexpected::Other(&format!("Value to large for Uint<{}", BITS)),
                &self,
            )
        })
    }
}

/// Helper function to remove  optionally `0x` prefix from hex strings.
fn trim_hex_prefix(str: &str) -> &str {
    if str.len() >= 2 && (&str[..2] == "0x" || &str[..2] == "0X") {
        &str[2..]
    } else {
        str
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::const_for;
    use proptest::proptest;

    #[test]
    fn test_serde_human_readable() {
        const_for!(BITS in SIZES {
            proptest!(|(value: Uint<BITS>)| {
                let serialized = serde_json::to_string(&value).unwrap();
                let deserialized = serde_json::from_str(&serialized).unwrap();
                assert_eq!(value, deserialized);
            });
        });
    }

    #[test]
    fn test_serde_machine_readable() {
        const_for!(BITS in SIZES {
            proptest!(|(value: Uint<BITS>)| {
                let serialized = bincode::serialize(&value).unwrap();
                let deserialized = bincode::deserialize(&serialized[..]).unwrap();
                assert_eq!(value, deserialized);
            });
        });
    }
}