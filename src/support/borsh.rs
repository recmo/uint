//! Support for the [`borsh`](https://crates.io/crates/borsh) crate.

#![cfg(feature = "borsh")]
#![cfg_attr(docsrs, doc(cfg(feature = "borsh")))]

use crate::{Bits, Uint};
use borsh::{io, BorshDeserialize, BorshSerialize};

impl<const BITS: usize, const LIMBS: usize> BorshDeserialize for Uint<BITS, LIMBS> {
    #[inline]
    fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        // This is a bit of an end-run around missing `generic_const_exprs`
        // We cannot declare a `[u8; Self::BYTES]` or `[u8; LIMBS * 8]`,
        // so we declare a `[u8; LIMBS]` and use unsafe to write to it.

        // TODO: Replace the unsafety with `generic_const_exprs` when
        // available
        let mut limbs = [0; LIMBS];

        // SAFETY: `limbs` is known to have identical memory layout and
        // alignment to `[u8; LIMBS * 8]`, which is guaranteed to safely
        // contain  [u8; Self::BYTES]`, as `LIMBS * 8 >= Self::BYTES`.
        // Reference:
        // https://doc.rust-lang.org/reference/type-layout.html#array-layout
        let this = unsafe {
            let ptr: *mut u8 = limbs.as_mut_ptr().cast();
            // target is only the first `SELF::BYTES` bytes
            let target = core::slice::from_raw_parts_mut(ptr, Self::BYTES);
            // this writes into the memory occupied by `limbs`
            reader.read_exact(target)?;

            // Using `Self::from_limbs(limbs)` would be incorrect here, as the
            // inner u64s are encoded in LE, and the platform may be BE.
            Self::try_from_le_slice(target)
        };

        this.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "value is too large for the type",
            )
        })
    }
}

impl<const BITS: usize, const LIMBS: usize> BorshSerialize for Uint<BITS, LIMBS> {
    #[inline]
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        // TODO: non-allocating and remove the `alloc` feature requirement
        let bytes = self.as_le_bytes_trimmed();
        writer.write_all(&bytes)
    }
}

impl<const BITS: usize, const LIMBS: usize> BorshDeserialize for Bits<BITS, LIMBS> {
    #[inline]
    fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        Uint::<BITS, LIMBS>::deserialize_reader(reader).map(Into::into)
    }
}

impl<const BITS: usize, const LIMBS: usize> BorshSerialize for Bits<BITS, LIMBS> {
    #[inline]
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        self.as_uint().serialize(writer)
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
    struct Something {
        is_it: bool,
        value: Uint<256, 4>,
    }

    #[test]
    fn test_uint() {
        let something = Something {
            is_it: true,
            value: Uint::<256, 4>::from_limbs([1, 2, 3, 4]),
        };
        let mut buf = [0; 33];

        something.serialize(&mut buf.as_mut_slice()).unwrap();
        assert_eq!(buf, [
            1, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0,
            0, 0, 0, 0
        ]);
        assert_eq!(&something.value.to_le_bytes::<32>(), &buf[1..]);
        assert_eq!(Something::try_from_slice(&mut &buf[..]).unwrap(), something);
    }

    #[derive(Debug, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
    struct AnotherThing {
        is_it: bool,
        value: Bits<256, 4>,
    }

    #[test]
    fn test_bits() {
        let another_thing = AnotherThing {
            is_it: true,
            value: Bits::<256, 4>::from_limbs([1, 2, 3, 4]),
        };
        let mut buf = [0; 33];

        another_thing.serialize(&mut buf.as_mut_slice()).unwrap();

        assert_eq!(buf, [
            1, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0,
            0, 0, 0, 0
        ]);
        assert_eq!(&another_thing.value.to_le_bytes::<32>(), &buf[1..]);
        assert_eq!(
            AnotherThing::try_from_slice(&mut &buf[..]).unwrap(),
            another_thing
        );
    }

    #[test]
    fn deser_invalid_value() {
        let buf = [0xff; 4];
        let mut reader = &mut &buf[..];

        let result = Uint::<31, 1>::deserialize_reader(&mut reader);
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert_eq!(err.to_string(), "value is too large for the type");
    }
}
