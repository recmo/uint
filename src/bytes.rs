// TODO: Use u64::from_{be/le}_bytes().
// TODO: Make `const fn`s when `const_for` is stable.

use crate::{nlimbs, Uint};

impl<const BITS: usize> Uint<BITS>
where
    [(); nlimbs(BITS)]:,
{
    /// The size of this integer type in bytes. Note that some bits may be
    /// forced zero if BITS is not cleanly divisible by eight.
    pub const BYTES: usize = (BITS + 7) / 8;

    #[must_use]
    pub fn try_from_be_bytes(bytes: &[u8]) -> Option<Self> {
        Self::try_from_le_bytes_impl(bytes.iter().copied().rev())
    }

    #[must_use]
    pub fn try_from_le_bytes(bytes: &[u8]) -> Option<Self> {
        Self::try_from_le_bytes_impl(bytes.iter().copied())
    }

    /// # Panics
    /// Panics if the value is too large for the bit-size of the Uint.
    #[must_use]
    #[track_caller]
    pub fn from_be_bytes(bytes: [u8; nbytes(BITS)]) -> Self {
        match Self::try_from_be_bytes(&bytes) {
            Some(uint) => uint,
            None => panic!("Value too large for Uint<{}>", BITS),
        }
    }

    /// # Panics
    /// Panics if the value is too large for the bit-size of the Uint.
    #[must_use]
    #[track_caller]
    pub fn from_le_bytes(bytes: [u8; nbytes(BITS)]) -> Self {
        match Self::try_from_le_bytes(&bytes) {
            Some(uint) => uint,
            None => panic!("Value too large for Uint<{}>", BITS),
        }
    }

    #[must_use]
    pub fn to_be_bytes(&self) -> [u8; nbytes(BITS)] {
        let mut bytes = [0; nbytes(BITS)];
        for (chunk, limb) in bytes
            .chunks_mut(8)
            .zip(self.as_limbs().iter().copied().rev())
        {
            chunk.copy_from_slice(&limb.to_be_bytes()[(8 - chunk.len())..]);
        }
        bytes
    }

    #[must_use]
    pub fn to_le_bytes(&self) -> [u8; nbytes(BITS)] {
        let mut bytes = [0; nbytes(BITS)];
        for (chunk, limb) in bytes.chunks_mut(8).zip(self.as_limbs().iter().copied()) {
            chunk.copy_from_slice(&limb.to_le_bytes()[..chunk.len()]);
        }
        bytes
    }

    #[must_use]
    fn try_from_le_bytes_impl<I>(iter: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let mut limbs = [0; nlimbs(BITS)];
        for (i, byte) in iter.enumerate() {
            if byte == 0 {
                continue;
            }
            let limb_index = i / 8;
            if limb_index >= Self::LIMBS {
                return None;
            }
            let byte_index = i % 8;
            limbs[limb_index] += (byte as u64) << (byte_index * 8);
        }
        if Self::LIMBS > 0 && limbs[Self::LIMBS - 1] > Self::MASK {
            return None;
        }
        Some(Self::from_limbs(limbs))
    }
}

/// Number of `u64` limbs required to represent the given number of bits.
/// This needs to be public because it is used in the `Uint` type.
#[must_use]
pub const fn nbytes(bits: usize) -> usize {
    (bits + 7) / 8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{uint, Uint};

    const N: Uint<128> = Uint::<128>::from_limbs([0x7890123456789012_u64, 0x1234567890123456_u64]);
    const BE: [u8; 16] = [
        0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90,
        0x12,
    ];
    const LE: [u8; 16] = [
        0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34,
        0x12,
    ];

    #[test]
    fn test_from_bytes() {
        assert_eq!(Uint::<16>::from_be_bytes([0x12, 0x34]), Uint::from(0x1234));
        assert_eq!(Uint::<16>::from_le_bytes([0x34, 0x12]), Uint::from(0x1234));
        assert_eq!(Uint::from_be_bytes(BE), N);
        assert_eq!(Uint::from_le_bytes(LE), N);
    }

    #[test]
    fn test_to_bytes() {
        assert_eq!(Uint::<16>::from(0x1234_u64).to_le_bytes(), [0x34, 0x12]);
        assert_eq!(Uint::<16>::from(0x1234_u64).to_be_bytes(), [0x12, 0x34]);
        assert_eq!(N.to_be_bytes(), BE);
        assert_eq!(N.to_le_bytes(), LE);
    }
}
