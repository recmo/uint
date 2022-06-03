use crate::{impl_bin_op, nlimbs, Uint};
use core::{
    iter::{zip, Product},
    num::Wrapping,
    ops::{Mul, MulAssign},
};

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    pub fn mul_mod(self, rhs: Self, modulus: Self) -> Self {
        // The challenge here is that Rust doesn't allow us to create a
        // `Uint<2 * BITS, _>` for the intermediate result. Otherwise
        // we could just use a `widening_mul`.

        todo!()
    }
}
