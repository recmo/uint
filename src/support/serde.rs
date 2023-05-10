//! Support for the [`serde`](https://crates.io/crates/serde) crate.
#![cfg(feature = "serde")]
#![cfg_attr(has_doc_cfg, doc(cfg(feature = "serde")))]

use crate::{nbytes, Bits, Uint};
use core::fmt::{Formatter, Result as FmtResult};
use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{fmt::Write, str};

/// Serialize a [`Uint`] value.
///
/// For human readable formats a `0x` prefixed lower case hex string is used.
/// For binary formats a byte array is used. Leading zeros are included.
impl<const BITS: usize, const LIMBS: usize> Serialize for Uint<BITS, LIMBS> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let bytes = self.to_be_bytes_vec();
        if serializer.is_human_readable() {
            // Special case for zero, which encodes as `0x0`.
            if BITS == 0 {
                return serializer.serialize_str("0x0");
            }
            // OPT: Allocation free method.
            let mut result = String::with_capacity(2 * Self::BYTES + 2);
            result.push_str("0x");
            for byte in bytes {
                write!(result, "{byte:02x}").unwrap();
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
impl<'de, const BITS: usize, const LIMBS: usize> Deserialize<'de> for Uint<BITS, LIMBS> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(StrVisitor)
        } else {
            deserializer.deserialize_bytes(ByteVisitor)
        }
    }
}

impl<const BITS: usize, const LIMBS: usize> Serialize for Bits<BITS, LIMBS> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_uint().serialize(serializer)
    }
}

impl<'de, const BITS: usize, const LIMBS: usize> Deserialize<'de> for Bits<BITS, LIMBS> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Uint::deserialize(deserializer).map(Self::from)
    }
}

/// Serde Visitor for human readable formats
struct StrVisitor<const BITS: usize, const LIMBS: usize>;

impl<'de, const BITS: usize, const LIMBS: usize> Visitor<'de> for StrVisitor<BITS, LIMBS> {
    type Value = Uint<BITS, LIMBS>;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "a {} byte hex string", nbytes(BITS))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let value = trim_hex_prefix(value);

        // ensure the string is the correct length (two characters in the string per
        // byte) exception: zero, for a Uint where BITS == 0
        if BITS == 0 {
            // special case, this class of ints has one member, zero.
            // zero is represented as "0x0" only
            if value != "0" {
                return Err(Error::invalid_value(Unexpected::Str(value), &self));
            }
            return Ok(Uint::<BITS, LIMBS>::ZERO);
        }
        if nbytes(BITS) * 2 < value.len() {
            return Err(Error::invalid_length(value.len(), &self));
        }

        let mut limbs = [0; LIMBS];
        for (i, chunk) in value.as_bytes().rchunks(16).enumerate() {
            let chunk = str::from_utf8(chunk)
                .map_err(|_| Error::invalid_value(Unexpected::Str(value), &self))?;
            let limb = u64::from_str_radix(chunk, 16)
                .map_err(|_| Error::invalid_value(Unexpected::Str(value), &self))?;
            if limb == 0 {
                continue;
            }
            if i >= LIMBS {
                return Err(Error::invalid_value(Unexpected::Str(value), &self));
            }
            limbs[i] = limb;
        }
        if BITS > 0 && limbs[LIMBS - 1] > Self::Value::MASK {
            return Err(Error::invalid_value(Unexpected::Str(value), &self));
        }
        Ok(Uint::from_limbs(limbs))
    }
}

/// Serde Visitor for non-human readable formats
struct ByteVisitor<const BITS: usize, const LIMBS: usize>;

impl<'de, const BITS: usize, const LIMBS: usize> Visitor<'de> for ByteVisitor<BITS, LIMBS> {
    type Value = Uint<BITS, LIMBS>;

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
        Uint::try_from_be_slice(value).ok_or_else(|| {
            E::invalid_value(
                Unexpected::Other(&format!("Value to large for Uint<{BITS}>")),
                &self,
            )
        })
    }
}

/// Helper function to remove optionally `0x` prefix from hex strings.
#[allow(clippy::missing_const_for_fn)]
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
    use crate::{const_for, nlimbs};
    use proptest::proptest;

    #[test]
    fn test_serde_human_readable() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            proptest!(|(value: Uint<BITS, LIMBS>)| {
                let serialized = serde_json::to_string(&value).unwrap();
                let deserialized = serde_json::from_str(&serialized).unwrap();
                assert_eq!(value, deserialized);
            });
        });
    }

    #[test]
    fn test_serde_machine_readable() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            proptest!(|(value: Uint<BITS, LIMBS>)| {
                let serialized = bincode::serialize(&value).unwrap();
                let deserialized = bincode::deserialize(&serialized[..]).unwrap();
                assert_eq!(value, deserialized);
            });
            proptest!(|(value: Bits<BITS, LIMBS>)| {
                let serialized = bincode::serialize(&value).unwrap();
                let deserialized = bincode::deserialize(&serialized[..]).unwrap();
                assert_eq!(value, deserialized);
            });
        });
    }

    #[test]
    fn test_serde_invalid_size_error() {
        // Test that if we add a character to a value that is already the max length for
        // the given number of bits, we get an error.
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            let value = Uint::<BITS, LIMBS>::MAX;
            let mut serialized = serde_json::to_string(&value).unwrap();

            // ensure format of serialized value is correct ("0x...")
            assert_eq!(&serialized[..3], "\"0x");
            // last character should be a quote
            assert_eq!(&serialized[serialized.len() - 1..], "\"");

            // strip the last character, add a zero, and finish with a quote
            serialized.pop();
            serialized.push('0');
            serialized.push('"');
            let deserialized = serde_json::from_str::<Uint<BITS, LIMBS>>(&serialized);
            assert!(deserialized.is_err());
        });
    }

    #[test]
    fn test_serde_zero_invalid_size_error() {
        // Test that if we add a zero to a large zero string, we get an error.
        // This is done by replacing a max string `0xffff...` with zeros: `0x0000...`
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            let value = Uint::<BITS, LIMBS>::MAX;
            let mut serialized = serde_json::to_string(&value).unwrap();

            // replace `f`s with `0`s
            serialized = serialized.replace('f', "0");
            // ensure format of serialized value is correct ("0x...")
            assert_eq!(&serialized[..3], "\"0x");
            // last character should be a quote
            assert_eq!(&serialized[serialized.len() - 1..], "\"");

            // strip the last character, add a zero, and finish with a quote
            serialized.pop();
            serialized.push('0');
            serialized.push('"');
            let deserialized = serde_json::from_str::<Uint<BITS, LIMBS>>(&serialized);
            assert!(deserialized.is_err());
        });
    }
}
