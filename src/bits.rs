use crate::Uint;

use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign,
};

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Reverses the order of bits in the integer. The least significant bit
    /// becomes the most significant bit, second least-significant bit becomes
    /// second most-significant bit, etc.
    pub fn reverse_bits(mut self) -> Self {
        self.limbs.reverse();
        for limb in &mut self.limbs {
            *limb = limb.reverse_bits();
        }
        if BITS % 64 != 0 {
            self >>= 64 - BITS % 64;
        }
        self
    }

    /// Returns the number of leading zeros in the binary representation of
    /// `self`.
    #[must_use]
    pub fn leading_zeros(&self) -> usize {
        self.as_limbs()
            .iter()
            .rev()
            .position(|&limb| limb != 0)
            .map_or(BITS, |n| {
                let fixed = Self::MASK.leading_zeros() as usize;
                let skipped = n * 64;
                let top = self.as_limbs()[LIMBS - n - 1].leading_zeros() as usize;
                skipped + top - fixed
            })
    }

    /// Returns the number of leading zeros in the binary representation of
    /// `self`.
    #[must_use]
    pub fn leading_ones(&self) -> usize {
        self.as_limbs()
            .iter()
            .rev()
            .position(|&limb| limb != u64::MAX)
            .map_or(BITS, |n| {
                let fixed = Self::MASK.leading_zeros() as usize;
                let skipped = n * 64;
                let top = self.as_limbs()[LIMBS - n - 1].leading_ones() as usize;
                skipped + top - fixed
            })
    }

    /// Returns the number of trailing zeros in the binary representation of
    /// `self`.
    #[must_use]
    pub fn trailing_zeros(&self) -> usize {
        self.as_limbs()
            .iter()
            .position(|&limb| limb != 0)
            .map_or(BITS, |n| {
                n * 64 + self.as_limbs()[n].trailing_zeros() as usize
            })
    }

    /// Returns the number of trailing ones in the binary representation of
    /// `self`.
    #[must_use]
    pub fn trailing_ones(&self) -> usize {
        self.as_limbs()
            .iter()
            .position(|&limb| limb != u64::MAX)
            .map_or(BITS, |n| {
                n * 64 + self.as_limbs()[n].trailing_ones() as usize
            })
    }

    /// Returns the number of ones in the binary representation of `self`.
    #[must_use]
    pub fn count_ones(&self) -> usize {
        self.as_limbs()
            .iter()
            .map(|limb| limb.count_ones() as usize)
            .sum()
    }

    /// Returns the number of zeros in the binary representation of `self`.
    #[must_use]
    pub fn count_zeros(&self) -> usize {
        Self::BITS - self.count_ones()
    }

    /// Length of the number in bits ignoring leading zeros.
    #[must_use]
    pub fn bit_len(&self) -> usize {
        BITS - self.leading_zeros()
    }

    /// Length of the number in bytes ignoring leading zeros.
    #[must_use]
    pub fn byte_len(&self) -> usize {
        (self.bit_len() + 7) / 8
    }

    /// Returns the base 2 logarithm of the number, rounded down.
    ///
    /// This is equivalent to the index of the highest set bit.
    ///
    /// Returns None if the number is zero.
    #[must_use]
    pub fn checked_log2(&self) -> Option<usize> {
        self.bit_len().checked_sub(1)
    }

    /// Returns the most significant 64 bits of the number and the exponent.
    ///
    /// Given return value $(\mathtt{bits}, \mathtt{exponent})$, the `self` can
    /// be approximated as
    ///
    /// $$
    /// \mathtt{self} ≈ \mathtt{bits} ⋅ 2^\mathtt{exponent}
    /// $$
    ///
    /// If `self` is $<≥> 2^{63}$, then `exponent` will be zero and `bits` will
    /// have leading zeros.
    #[must_use]
    pub fn most_significant_bits(&self) -> (u64, usize) {
        let first_set_limb = self
            .as_limbs()
            .iter()
            .rposition(|&limb| limb != 0)
            .unwrap_or(0);
        if first_set_limb == 0 {
            (self.as_limbs().first().copied().unwrap_or(0), 0)
        } else {
            let hi = self.as_limbs()[first_set_limb];
            let lo = self.as_limbs()[first_set_limb - 1];
            let leading_zeros = hi.leading_zeros();
            let bits = if leading_zeros > 0 {
                (hi << leading_zeros) | (lo >> (64 - leading_zeros))
            } else {
                hi
            };
            let exponent = first_set_limb * 64 - leading_zeros as usize;
            (bits, exponent)
        }
    }

    // TODO: is_power_of_two, next_power_of_two

    /// Checked left shift by `rhs` bits.
    ///
    /// Returns $\mathtt{self} ⋅ 2^{\mathtt{rhs}}$ or [`None`] if the result
    /// would $≥ 2^{\mathtt{BITS}}$. That is, it returns [`None`] if the bits
    /// shifted out would be non-zero.
    ///
    /// Note: This differs from [`u64::checked_shl`] which returns `None` if the
    /// shift is larger than BITS (which is IMHO not very useful).
    #[allow(clippy::inline_always)]
    #[inline(always)]
    #[must_use]
    pub fn checked_shl(mut self, rhs: usize) -> Option<Self> {
        match self.overflowing_shl(rhs) {
            (value, false) => Some(value),
            _ => None,
        }
    }

    /// Left shift by `rhs` bits with overflow detection.
    ///
    /// Returns $\mod{\mathtt{value} ⋅ 2^{\mathtt{rhs}}}_{2^{\mathtt{BITS}}}$.
    /// If the product is $≥ 2^{\mathtt{BITS}}$ it returns `true`. That is, it
    /// returns true if the bits shifted out are non-zero.
    ///
    /// Note: This differs from [`u64::overflowing_shl`] which returns `true` if
    /// the shift is larger than `BITS` (which is IMHO not very useful).
    #[must_use]
    pub fn overflowing_shl(mut self, rhs: usize) -> (Self, bool) {
        let (limbs, bits) = (rhs / 64, rhs % 64);
        if limbs >= LIMBS {
            return (Self::ZERO, self != Self::ZERO);
        }
        if bits == 0 {
            // Check for overflow
            let mut overflow = false;
            for i in (LIMBS - limbs)..LIMBS {
                overflow |= self.limbs[i] != 0;
            }
            if self.limbs[LIMBS - limbs - 1] > Self::MASK {
                overflow = true;
            }

            // Shift
            for i in (limbs..LIMBS).rev() {
                self.limbs[i] = self.limbs[i - limbs];
            }
            for i in 0..limbs {
                self.limbs[i] = 0;
            }
            self.limbs[LIMBS - 1] &= Self::MASK;
            return (self, overflow);
        }

        // Check for overflow
        let mut overflow = false;
        for i in (LIMBS - limbs)..LIMBS {
            overflow |= self.limbs[i] != 0;
        }
        if self.limbs[LIMBS - limbs - 1] >> (64 - bits) != 0 {
            overflow = true;
        }
        if self.limbs[LIMBS - limbs - 1] << bits > Self::MASK {
            overflow = true;
        }

        // Shift
        for i in (limbs + 1..LIMBS).rev() {
            self.limbs[i] = self.limbs[i - limbs] << bits;
            self.limbs[i] |= self.limbs[i - limbs - 1] >> (64 - bits);
        }
        self.limbs[limbs] = self.limbs[0] << bits;
        for i in 0..limbs {
            self.limbs[i] = 0;
        }
        self.limbs[LIMBS - 1] &= Self::MASK;
        (self, overflow)
    }

    /// Left shift by `rhs` bits.
    ///
    /// Returns $\mod{\mathtt{value} ⋅ 2^{\mathtt{rhs}}}_{2^{\mathtt{BITS}}}$.
    ///
    /// Note: This differs from [`u64::wrapping_shl`] which first reduces `rhs`
    /// by `BITS` (which is IMHO not very useful).
    #[allow(clippy::inline_always)]
    #[inline(always)]
    #[must_use]
    pub fn wrapping_shl(mut self, rhs: usize) -> Self {
        self.overflowing_shl(rhs).0
    }

    /// Checked right shift by `rhs` bits.
    ///
    /// $$
    /// \frac{\mathtt{self}}{2^{\mathtt{rhs}}}
    /// $$
    ///
    /// Returns the above or [`None`] if the division is not exact. This is the
    /// same as
    ///
    /// Note: This differs from [`u64::checked_shr`] which returns `None` if the
    /// shift is larger than BITS (which is IMHO not very useful).
    #[allow(clippy::inline_always)]
    #[inline(always)]
    #[must_use]
    pub fn checked_shr(mut self, rhs: usize) -> Option<Self> {
        match self.overflowing_shr(rhs) {
            (value, false) => Some(value),
            _ => None,
        }
    }

    /// Right shift by `rhs` bits with underflow detection.
    ///
    /// $$
    /// \floor{\frac{\mathtt{self}}{2^{\mathtt{rhs}}}}
    /// $$
    ///
    /// Returns the above and `false` if the division was exact, and `true` if
    /// it was rounded down. This is the same as non-zero bits being shifted
    /// out.
    ///
    /// Note: This differs from [`u64::overflowing_shl`] which returns `true` if
    /// the shift is larger than `BITS` (which is IMHO not very useful).
    #[must_use]
    pub fn overflowing_shr(mut self, rhs: usize) -> (Self, bool) {
        let (limbs, bits) = (rhs / 64, rhs % 64);
        if limbs >= LIMBS {
            return (Self::ZERO, self != Self::ZERO);
        }
        if bits == 0 {
            // Check for overflow
            let mut overflow = false;
            for i in 0..limbs {
                overflow |= self.limbs[i] != 0;
            }

            // Shift
            for i in 0..(LIMBS - limbs) {
                self.limbs[i] = self.limbs[i + limbs];
            }
            for i in (LIMBS - limbs)..LIMBS {
                self.limbs[i] = 0;
            }
            return (self, overflow);
        }

        // Check for overflow
        let mut overflow = false;
        for i in 0..limbs {
            overflow |= self.limbs[i] != 0;
        }
        overflow |= self.limbs[limbs] >> bits != 0;

        // Shift
        for i in 0..(LIMBS - limbs - 1) {
            self.limbs[i] = self.limbs[i + limbs] >> bits;
            self.limbs[i] |= self.limbs[i + limbs + 1] << (64 - bits);
        }
        self.limbs[LIMBS - limbs - 1] = self.limbs[LIMBS - 1] >> bits;
        for i in (LIMBS - limbs)..LIMBS {
            self.limbs[i] = 0;
        }
        (self, overflow)
    }

    /// Right shift by `rhs` bits.
    ///
    /// $$
    /// \mathtt{wrapping\\_shr}(\mathtt{self}, \mathtt{rhs}) =
    /// \floor{\frac{\mathtt{self}}{2^{\mathtt{rhs}}}}
    /// $$
    ///
    /// Note: This differs from [`u64::wrapping_shr`] which first reduces `rhs`
    /// by `BITS` (which is IMHO not very useful).
    #[allow(clippy::inline_always)]
    #[inline(always)]
    #[must_use]
    pub fn wrapping_shr(mut self, rhs: usize) -> Self {
        self.overflowing_shr(rhs).0
    }

    /// Shifts the bits to the left by a specified amount, `rhs`, wrapping the
    /// truncated bits to the end of the resulting integer.
    #[must_use]
    pub fn rotate_left(self, rhs: usize) -> Self {
        if BITS == 0 {
            return Self::ZERO;
        }
        let rhs = rhs % BITS;
        self << rhs | self >> (BITS - rhs)
    }

    #[allow(clippy::inline_always)]
    #[inline(always)]
    #[must_use]
    pub fn rotate_right(self, rhs: usize) -> Self {
        if BITS == 0 {
            return Self::ZERO;
        }
        let rhs = rhs % BITS;
        self.rotate_left(BITS - rhs)
    }
}

macro_rules! impl_bit_op {
    ($trait:ident, $fn:ident, $trait_assign:ident, $fn_assign:ident) => {
        impl<const BITS: usize, const LIMBS: usize> $trait_assign<Uint<BITS, LIMBS>>
            for Uint<BITS, LIMBS>
        {
            #[allow(clippy::inline_always)]
            #[inline(always)]
            fn $fn_assign(&mut self, rhs: Uint<BITS, LIMBS>) {
                self.$fn_assign(&rhs);
            }
        }
        impl<const BITS: usize, const LIMBS: usize> $trait_assign<&Uint<BITS, LIMBS>>
            for Uint<BITS, LIMBS>
        {
            fn $fn_assign(&mut self, rhs: &Uint<BITS, LIMBS>) {
                for (limb, rhs) in self.as_limbs_mut().iter_mut().zip(rhs.as_limbs()) {
                    u64::$fn_assign(limb, rhs);
                }
            }
        }
        impl<const BITS: usize, const LIMBS: usize> $trait<Uint<BITS, LIMBS>>
            for Uint<BITS, LIMBS>
        {
            type Output = Uint<BITS, LIMBS>;

            #[allow(clippy::inline_always)]
            #[inline(always)]
            fn $fn(mut self, rhs: Uint<BITS, LIMBS>) -> Self::Output {
                self.$fn_assign(rhs);
                self
            }
        }
        impl<const BITS: usize, const LIMBS: usize> $trait<&Uint<BITS, LIMBS>>
            for Uint<BITS, LIMBS>
        {
            type Output = Uint<BITS, LIMBS>;

            #[allow(clippy::inline_always)]
            #[inline(always)]
            fn $fn(mut self, rhs: &Uint<BITS, LIMBS>) -> Self::Output {
                self.$fn_assign(rhs);
                self
            }
        }
        impl<const BITS: usize, const LIMBS: usize> $trait<Uint<BITS, LIMBS>>
            for &Uint<BITS, LIMBS>
        {
            type Output = Uint<BITS, LIMBS>;

            #[allow(clippy::inline_always)]
            #[inline(always)]
            fn $fn(self, mut rhs: Uint<BITS, LIMBS>) -> Self::Output {
                rhs.$fn_assign(self);
                rhs
            }
        }
        impl<const BITS: usize, const LIMBS: usize> $trait<&Uint<BITS, LIMBS>>
            for &Uint<BITS, LIMBS>
        {
            type Output = Uint<BITS, LIMBS>;

            #[allow(clippy::inline_always)]
            #[inline(always)]
            fn $fn(self, rhs: &Uint<BITS, LIMBS>) -> Self::Output {
                self.clone().$fn(rhs)
            }
        }
    };
}

impl_bit_op!(BitOr, bitor, BitOrAssign, bitor_assign);
impl_bit_op!(BitAnd, bitand, BitAndAssign, bitand_assign);
impl_bit_op!(BitXor, bitxor, BitXorAssign, bitxor_assign);

impl<const BITS: usize, const LIMBS: usize> ShlAssign<usize> for Uint<BITS, LIMBS> {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shl_assign(&mut self, rhs: usize) {
        *self = self.wrapping_shl(rhs);
    }
}

impl<const BITS: usize, const LIMBS: usize> ShlAssign<&usize> for Uint<BITS, LIMBS> {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shl_assign(&mut self, rhs: &usize) {
        *self = self.wrapping_shl(*rhs);
    }
}

impl<const BITS: usize, const LIMBS: usize> Shl<usize> for Uint<BITS, LIMBS> {
    type Output = Self;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shl(mut self, rhs: usize) -> Self {
        self.wrapping_shl(rhs)
    }
}

impl<const BITS: usize, const LIMBS: usize> Shl<usize> for &Uint<BITS, LIMBS> {
    type Output = Uint<BITS, LIMBS>;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shl(self, rhs: usize) -> Self::Output {
        self.wrapping_shl(rhs)
    }
}

impl<const BITS: usize, const LIMBS: usize> Shl<&usize> for Uint<BITS, LIMBS> {
    type Output = Self;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shl(mut self, rhs: &usize) -> Self {
        self.wrapping_shl(*rhs)
    }
}

impl<const BITS: usize, const LIMBS: usize> Shl<&usize> for &Uint<BITS, LIMBS> {
    type Output = Uint<BITS, LIMBS>;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shl(self, rhs: &usize) -> Self::Output {
        self.wrapping_shl(*rhs)
    }
}

impl<const BITS: usize, const LIMBS: usize> ShrAssign<usize> for Uint<BITS, LIMBS> {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shr_assign(&mut self, rhs: usize) {
        *self = self.wrapping_shr(rhs);
    }
}

impl<const BITS: usize, const LIMBS: usize> ShrAssign<&usize> for Uint<BITS, LIMBS> {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shr_assign(&mut self, rhs: &usize) {
        *self = self.wrapping_shr(*rhs);
    }
}

impl<const BITS: usize, const LIMBS: usize> Shr<usize> for Uint<BITS, LIMBS> {
    type Output = Self;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shr(self, rhs: usize) -> Self {
        self.wrapping_shr(rhs)
    }
}

impl<const BITS: usize, const LIMBS: usize> Shr<usize> for &Uint<BITS, LIMBS> {
    type Output = Uint<BITS, LIMBS>;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shr(self, rhs: usize) -> Self::Output {
        self.wrapping_shr(rhs)
    }
}

impl<const BITS: usize, const LIMBS: usize> Shr<&usize> for Uint<BITS, LIMBS> {
    type Output = Self;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shr(self, rhs: &usize) -> Self {
        self.wrapping_shr(*rhs)
    }
}

impl<const BITS: usize, const LIMBS: usize> Shr<&usize> for &Uint<BITS, LIMBS> {
    type Output = Uint<BITS, LIMBS>;

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn shr(self, rhs: &usize) -> Self::Output {
        self.wrapping_shr(*rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{aliases::U128, const_for, nlimbs};
    use proptest::proptest;

    #[test]
    fn test_leading_zeros() {
        assert_eq!(Uint::<0, 0>::ZERO.leading_zeros(), 0);
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            assert_eq!(Uint::<BITS, LIMBS>::ZERO.leading_zeros(), BITS);
            assert_eq!(Uint::<BITS, LIMBS>::MAX.leading_zeros(), 0);
            assert_eq!(Uint::<BITS, LIMBS>::from(1).leading_zeros(), BITS - 1);
            proptest!(|(value: Uint<BITS, LIMBS>)| {
                let zeros = value.leading_zeros();
                assert!(zeros <= BITS);
                // TODO: Check with bitshift operators.
                // assert!(value << zeros >= Uint::MAX >> 1);
                // assert_eq!(value >> (BITS - zeros), Uint::ZERO);
            });
        });
        proptest!(|(value: u128)| {
            let uint = U128::from(value);
            assert_eq!(uint.leading_zeros(), value.leading_zeros() as usize);
        });
    }

    #[test]
    fn test_most_significant_bits() {
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint::<BITS, LIMBS>;
            proptest!(|(value: u64)| {
                let value = if U::LIMBS <= 1 { value & U::MASK } else { value };
                assert_eq!(U::from(value).most_significant_bits(), (value, 0));
            });
        });
        proptest!(|(mut limbs: [u64; 2])| {
            if limbs[1] == 0 {
                limbs[1] = 1;
            }
            let (bits, exponent) = U128::from_limbs(limbs).most_significant_bits();
            assert!(bits >= 1_u64 << 63);
            assert_eq!(exponent, 64 - limbs[1].leading_zeros() as usize);
        });
    }

    #[test]
    fn test_checked_log2() {
        assert_eq!(U128::from(0).checked_log2(), None);
        assert_eq!(U128::from(1).checked_log2(), Some(0));
        assert_eq!(U128::from(2).checked_log2(), Some(1));
        assert_eq!(U128::from(3).checked_log2(), Some(1));
        assert_eq!(U128::from(127).checked_log2(), Some(6));
        assert_eq!(U128::from(128).checked_log2(), Some(7));
    }

    #[test]
    fn test_checked_shl() {
        assert_eq!(
            Uint::<65, 2>::from_limbs([0x0010_0000_0000_0000, 0]).checked_shl(1),
            Some(Uint::<65, 2>::from_limbs([0x0020_0000_0000_0000, 0]))
        );
        assert_eq!(
            Uint::<127, 2>::from_limbs([0x0010_0000_0000_0000, 0]).checked_shl(64),
            Some(Uint::<127, 2>::from_limbs([0, 0x0010_0000_0000_0000]))
        );
    }

    #[test]
    fn test_small() {
        const_for!(BITS in [1, 2, 8, 16, 32, 63, 64] {
            type U = Uint::<BITS, 1>;
            proptest!(|(a: U, b: U)| {
                assert_eq!(a | b, U::from_limbs([a.limbs[0] | b.limbs[0]]));
                assert_eq!(a & b, U::from_limbs([a.limbs[0] & b.limbs[0]]));
                assert_eq!(a ^ b, U::from_limbs([a.limbs[0] ^ b.limbs[0]]));
            });
            proptest!(|(a: U, s in 0..BITS)| {
                assert_eq!(a << s, U::from_limbs([a.limbs[0] << s & U::MASK]));
                assert_eq!(a >> s, U::from_limbs([a.limbs[0] >> s]));
            });
        });
        proptest!(|(a: Uint::<32, 1>, s in 0_usize..=34)| {
            assert_eq!(a.reverse_bits(), Uint::from((a.limbs[0] as u32).reverse_bits() as u64));
            assert_eq!(a.rotate_left(s), Uint::from((a.limbs[0] as u32).rotate_left(s as u32) as u64));
            assert_eq!(a.rotate_right(s), Uint::from((a.limbs[0] as u32).rotate_right(s as u32) as u64));
        });
        proptest!(|(a: Uint::<64, 1>, s in 0_usize..=66)| {
            assert_eq!(a.reverse_bits(), Uint::from(a.limbs[0].reverse_bits()));
            assert_eq!(a.rotate_left(s), Uint::from(a.limbs[0].rotate_left(s as u32)));
            assert_eq!(a.rotate_right(s), Uint::from(a.limbs[0].rotate_right(s as u32)));
        });
    }

    #[test]
    fn test_shift_reverse() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint::<BITS, LIMBS>;
            proptest!(|(value: U, shift in 0..=BITS + 2)| {
                let left = (value << shift).reverse_bits();
                let right = value.reverse_bits() >> shift;
                assert_eq!(left, right);
            });
        });
    }

    #[test]
    fn test_rotate() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint::<BITS, LIMBS>;
            proptest!(|(value: U, shift in  0..=BITS + 2)| {
                let rotated = value.rotate_left(shift).rotate_right(shift);
                assert_eq!(value, rotated);
            });
        });
    }
}
