//! Support for the [`borsh`](https://crates.io/crates/borsh) crate.

#![cfg(feature = "borsh")]
#![cfg_attr(docsrs, doc(cfg(feature = "borsh")))]

use crate::{Bits, Uint};
use borsh::{io, BorshDeserialize, BorshSerialize};

impl<const BITS: usize, const LIMBS: usize> BorshDeserialize for Uint<BITS, LIMBS> {
    #[inline]
    fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let mut limbs = [0; LIMBS];

        limbs
            .iter_mut()
            .try_for_each(|limb| u64::deserialize_reader(reader).map(|value| *limb = value))?;

        Ok(Self::from_limbs(limbs))
    }
}

impl<const BITS: usize, const LIMBS: usize> BorshSerialize for Uint<BITS, LIMBS> {
    #[inline]
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        self.as_limbs()
            .iter()
            .try_for_each(|limb| limb.serialize(writer))
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
}
