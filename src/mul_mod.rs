use crate::{algorithms, impl_bin_op, nlimbs, Uint};
use core::{
    iter::{zip, Product},
    num::Wrapping,
    ops::{Mul, MulAssign},
};

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    #[must_use]
    pub fn mul_mod(self, rhs: Self, mut modulus: Self) -> Self {
        // Compute full product.
        // The challenge here is that Rust doesn't allow us to create a
        // `Uint<2 * BITS, _>` for the intermediate result. Otherwise
        // we could just use a `widening_mul`. So instead we allocate from heap.
        let mut product = vec![0; nlimbs(2 * BITS)];
        let overflow = algorithms::mul_inline(&self.limbs, &rhs.limbs, &mut product);
        debug_assert!(!overflow);

        // Compute modulus using `div_rem`.
        // This stores the remainder in the divisor, `modulus`.
        algorithms::div_rem(&mut product, &mut modulus.limbs);

        modulus
    }
}
