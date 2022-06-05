use crate::{algorithms, nlimbs, Uint};

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {

    pub fn gcd(self, other: Self) -> Self {
        algorithms::gcd(self, other)
    }

    pub fn gcd_extended(self, other: Self) -> (Self, Self, Self) {
        algorithms::gcd_extended(self, other)
    }
}
