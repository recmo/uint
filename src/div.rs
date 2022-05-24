use crate::{impl_bin_op, Uint};
use core::{
    ops::{Div, DivAssign, Rem, RemAssign},
};
use crate::algorihtms

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Computes `self / rhs`, returning [`None`] if `rhs == 0`.
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs == Self::ZERO {
            return None;
        }
        Some(self / rhs)
    }
    /// Computes `self % rhs`, returning [`None`] if `rhs == 0`.
    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        if rhs == Self::ZERO {
            return None;
        }
        Some(self % rhs)
    }

    pub fn div_ceil(self, rhs: Self) -> Self {
        assert!(rhs != Self::ZERO);
        let (q, r) = self.div_rem(rhs);
        if r != Self::ZERO {
            q + Self::ONE
        } else {
            q
        }
    }

    pub fn div_rem(self, rhs: Self) -> (Self, Self) {
        assert!(rhs != Self::ZERO);
        let mut q = Self::ZERO;
        let mut r = self;

    }
}

