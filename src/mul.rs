use crate::Uint;
use core::{
    iter::Product,
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
    }

    /// Computes `self * rhs`, saturating at the numeric bounds instead of overflowing.
    #[allow(clippy::inline_always)]
    #[inline(always)]
    #[must_use]
    pub fn saturating_mul(self, rhs: Self) -> Self {
        match self.overflowing_mul(rhs) {
            (value, false) => value,
            _ => Self::MAX,
        }
    }

    /// Calculates the complete product `self * rhs` without the possibility to
    /// overflow.
    /// 
    /// This returns the low-order (wrapping) bits and the high-order (overflow)
    /// bits of the result as two separate values, in that order.
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

    /// Computes the inverse modulo $2^{\mathtt{BITS}}$ of `self`, returning
    /// [`None`] if the inverse does not exist.
    #[must_use]
    pub fn ring_inverse(self) -> Option<Self> {
        if BITS == 0 || self.limbs[0] & 1 == 0 {
            return None;
        }
        todo!()
    }
}
