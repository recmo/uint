use crate::{impl_bin_op, Uint};
use core::{
    iter::Product,
    num::Wrapping,
    ops::{Mul, MulAssign},
};

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Computes `self * rhs`, returning [`None`] if overflow occurred.
    #[allow(clippy::inline_always)]
    #[inline(always)]
    #[must_use]
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        match self.overflowing_mul(rhs) {
            (value, false) => Some(value),
            _ => None,
        }
    }

    /// Calculates the multiplication of self and rhs.
    ///
    /// Returns a tuple of the multiplication along with a boolean indicating
    /// whether an arithmetic overflow would occur. If an overflow would have
    /// occurred then the wrapped value is returned.
    #[must_use]
    pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        if BITS == 0 {
            return (Self::ZERO, false);
        }
        let mut result = Self::ZERO;
        let mut overflow = false;
        for (i, &rhs) in rhs.limbs.iter().enumerate() {
            let mut carry = 0_u128;
            for (res, &lhs) in result.limbs[i..].iter_mut().zip(self.limbs.iter()) {
                carry += u128::from(*res) + u128::from(lhs) * u128::from(rhs);
                *res = carry as u64;
                carry >>= 64;
            }
            overflow |= carry != 0;
        }
        overflow |= result.limbs[LIMBS - 1] > Self::MASK;
        result.limbs[LIMBS - 1] &= Self::MASK;
        (result, overflow)
    }

    /// Computes `self * rhs`, saturating at the numeric bounds instead of
    /// overflowing.
    #[allow(clippy::inline_always)]
    #[inline(always)]
    #[must_use]
    pub fn saturating_mul(self, rhs: Self) -> Self {
        match self.overflowing_mul(rhs) {
            (value, false) => value,
            _ => Self::MAX,
        }
    }

    /// Computes `self * rhs`, saturating at the numeric bounds instead of
    /// overflowing.
    #[allow(clippy::inline_always)]
    #[inline(always)]
    #[must_use]
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        self.overflowing_mul(rhs).0
    }

    /// Computes the inverse modulo $2^{\mathtt{BITS}}$ of `self`, returning
    /// [`None`] if the inverse does not exist.
    #[must_use]
    pub fn ring_inverse(self) -> Option<Self> {
        if BITS == 0 || self.limbs[0] & 1 == 0 {
            return None;
        }

        // Compute inverse of first limb
        let mut result = Self::ZERO;
        result.limbs[0] = {
            const W2: Wrapping<u64> = Wrapping(2);
            const W3: Wrapping<u64> = Wrapping(3);
            let n = Wrapping(self.limbs[0]);
            let mut inv = n * W3 ^ W2; // Correct on 4 bits.
            inv *= W2 - n * inv; // Correct on 8 bits.
            inv *= W2 - n * inv; // Correct on 16 bits.
            inv *= W2 - n * inv; // Correct on 32 bits.
            inv *= W2 - n * inv; // Correct on 64 bits.
            debug_assert_eq!(n.0.wrapping_mul(inv.0), 1);
            inv.0
        };

        // Continue with rest of limbs
        let mut correct_limbs = 1;
        while correct_limbs < LIMBS {
            result *= Self::from(2) - self * result;
            correct_limbs *= 2;
        }
        result.limbs[LIMBS - 1] &= Self::MASK;

        Some(result)
    }

    /// Calculates the complete product `self * rhs` without the possibility to
    /// overflow.
    ///
    /// This returns the low-order (wrapping) bits and the high-order (overflow)
    /// bits of the result as two separate values, in that order.
    ///
    /// # Panics
    ///
    /// Panics if `LIMBS2` does not equal `LIMBS * 2`.
    #[allow(clippy::inline_always)]
    #[inline(always)]
    #[must_use]
    pub fn widening_mul(self, rhs: Self) -> (Self, Self) {
        self.carrying_mul(rhs, Self::ZERO)
    }

    /// Calculates the “full multiplication” `self * rhs + carry` without the
    /// possibility to overflow.
    ///
    /// This returns the low-order (wrapping) bits and the high-order (overflow)
    /// bits of the result as two separate values, in that order.
    #[must_use]
    pub fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self) {
        todo!()
    }
}

impl<const BITS: usize, const LIMBS: usize> Product<Self> for Uint<BITS, LIMBS> {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut result = Self::ZERO;
        for item in iter {
            result *= item;
        }
        result
    }
}

impl<'a, const BITS: usize, const LIMBS: usize> Product<&'a Self> for Uint<BITS, LIMBS> {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        let mut result = Self::ZERO;
        for item in iter {
            result *= item;
        }
        result
    }
}

impl_bin_op!(Mul, mul, MulAssign, mul_assign, wrapping_mul);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{const_for, nlimbs};
    use proptest::proptest;

    #[test]
    fn test_commutative() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(a: U, b: U)| {
                assert_eq!(a * b, b * a);
            });
        });
    }

    #[test]
    fn test_associative() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(a: U, b: U, c: U)| {
                assert_eq!(a * (b * c), (a * b) * c);
            });
        });
    }

    #[test]
    fn test_distributive() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(a: U, b: U, c: U)| {
                assert_eq!(a * (b + c), (a * b) + (a *c));
            });
        });
    }

    #[test]
    fn test_identity() {
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(value: U)| {
                assert_eq!(value * U::from(0), U::ZERO);
                assert_eq!(value * U::from(1), value);
            });
        });
    }

    #[test]
    fn test_inverse() {
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(mut a: U)| {
                a |= U::from(1); // Make sure a is invertible
                assert_eq!(a * a.ring_inverse().unwrap(), U::from(1));
                assert_eq!(a.ring_inverse().unwrap().ring_inverse().unwrap(), a);
            });
        });
    }
}
