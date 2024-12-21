//! Support for the [`borsh`](https://crates.io/crates/borsh) crate.

#![cfg(feature = "borsh")]
#![cfg_attr(docsrs, doc(cfg(feature = "borsh")))]

use crate::{Bits, Uint};
use borsh::{io, BorshDeserialize, BorshSerialize};

macro_rules! impl_borsh_for_uint_and_bits {
    ($(($bits:expr, $bytes:expr)),*) => {
        $(
            impl<const LIMBS: usize> BorshDeserialize for Uint<$bits, LIMBS> {
                #[inline]
                fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
                    <[u8; $bytes]>::deserialize_reader(reader).map(Self::from_le_bytes)
                }
            }

            impl<const LIMBS: usize> BorshSerialize for Uint<$bits, LIMBS> {
                #[inline]
                fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
                    self.to_le_bytes::<$bytes>().serialize(writer)
                }
            }

            impl<const LIMBS: usize> BorshDeserialize for Bits<$bits, LIMBS> {
                #[inline]
                fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
                    Uint::<$bits, LIMBS>::deserialize_reader(reader).map(Into::into)
                }
            }

            impl<const LIMBS: usize> BorshSerialize for Bits<$bits, LIMBS> {
                #[inline]
                fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
                    self.as_uint().serialize(writer)
                }
            }
        )*
    };
}

impl_borsh_for_uint_and_bits!(
    // impl every 8 bits between 8 and 256 inclusive
    (8, 1),
    (16, 2),
    (24, 3),
    (32, 4),
    (40, 5),
    (48, 6),
    (56, 7),
    (64, 8),
    (72, 9),
    (80, 10),
    (88, 11),
    (96, 12),
    (104, 13),
    (112, 14),
    (120, 15),
    (128, 16),
    (136, 17),
    (144, 18),
    (152, 19),
    (160, 20),
    (168, 21),
    (176, 22),
    (184, 23),
    (192, 24),
    (200, 25),
    (208, 26),
    (216, 27),
    (224, 28),
    (232, 29),
    (240, 30),
    (248, 31),
    (256, 32),
    // arbitrarily impl for multiples of 256 bits
    (512, 64),
    (768, 96),
    (1024, 128),
    (1280, 160),
    (1536, 192),
    (1792, 224),
    (2048, 256),
    (2304, 288),
    (2560, 320),
    (2816, 352),
    (3072, 384),
    (3328, 416),
    (3584, 448),
    (3840, 480),
    (4096, 512)
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_uint() {
        #[derive(Debug, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
        struct Something {
            is_it: bool,
            value: Uint<240, 4>,
        }

        let something = Something {
            is_it: true,
            value: Uint::from_limbs([1, 2, 3, 4]),
        };
        let mut buf = [0; 31];

        something.serialize(&mut buf.as_mut_slice()).unwrap();
        assert_eq!(buf, [
            1, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0,
            0, 0
        ]);
        assert_eq!(&something.value.to_le_bytes::<30>(), &buf[1..]);
        assert_eq!(Something::try_from_slice(&mut &buf[..]).unwrap(), something);
    }

    #[test]
    fn test_bits() {
        #[derive(Debug, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
        struct AnotherThing {
            is_it: bool,
            value: Bits<224, 4>,
        }

        let another_thing = AnotherThing {
            is_it: true,
            value: Bits::from_limbs([1, 2, 3, 4]),
        };
        let mut buf = [0; 29];

        another_thing.serialize(&mut buf.as_mut_slice()).unwrap();
        assert_eq!(buf, [
            1, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0
        ]);
        assert_eq!(&another_thing.value.to_le_bytes::<28>(), &buf[1..]);
        assert_eq!(
            AnotherThing::try_from_slice(&mut &buf[..]).unwrap(),
            another_thing
        );
    }
}
