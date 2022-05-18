use crate::Uint;

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Returns an iterator over the base `base` digits of the number in
    /// little-endian order.
    pub fn to_base_le(&self, base: u64) -> impl Iterator<Item = u64> {
        SpigotLittle {
            base,
            limbs: self.limbs,
        }
    }

    pub fn to_base_be(&self, base: u64) -> impl Iterator<Item = u64> {
        // TODO: Alloc free method?
        OwnedVecIterator {
            vec: self.to_base_le(base).collect(),
        }
    }
}

struct SpigotLittle<const LIMBS: usize> {
    base:  u64,
    limbs: [u64; LIMBS],
}

impl<const LIMBS: usize> Iterator for SpigotLittle<LIMBS> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // Knuth Algorithm S.
        let mut zero: u64 = 0_u64;
        let mut remainder = 0_u128;
        for limb in self.limbs.iter_mut().rev() {
            zero |= *limb;
            remainder <<= 64;
            remainder |= *limb as u128;
            *limb = (remainder / (self.base as u128)) as u64;
            remainder %= self.base as u128;
        }
        if zero != 0 {
            Some(remainder as u64)
        } else {
            None
        }
    }
}

struct OwnedVecIterator {
    vec: Vec<u64>,
}

impl Iterator for OwnedVecIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.vec.pop()
    }
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
#[allow(clippy::zero_prefixed_literal)]
mod tests {
    use super::*;

    // 90630363884335538722706632492458228784305343302099024356772372330524102404852
    const N: Uint<256, 4> = Uint::from_limbs([
        0xa8ec92344438aaf4_u64,
        0x9819ebdbd1faaab1_u64,
        0x573b1a7064c19c1a_u64,
        0xc85ef7d79691fe79_u64,
    ]);

    #[test]
    fn test_base_le() {
        assert_eq!(
            Uint::<64, 1>::from(123456789)
                .to_base_le(10)
                .collect::<Vec<_>>(),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1]
        );
        assert_eq!(
            N.to_base_le(10000000000000000000_u64).collect::<Vec<_>>(),
            vec![
                2372330524102404852,
                0534330209902435677,
                7066324924582287843,
                0630363884335538722,
                9
            ]
        );
    }
    #[test]
    fn test_base_be() {
        assert_eq!(
            Uint::<64, 1>::from(123456789)
                .to_base_be(10)
                .collect::<Vec<_>>(),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
        assert_eq!(
            N.to_base_be(10000000000000000000_u64).collect::<Vec<_>>(),
            vec![
                9,
                0630363884335538722,
                7066324924582287843,
                0534330209902435677,
                2372330524102404852
            ]
        );
    }
}
