use crate::Uint;

use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    #[must_use]
    pub const fn signed_neg(self) -> Self
        self.overflowing_neg().0
    }
}

