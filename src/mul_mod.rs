use crate::{algorithms, nlimbs, Uint};

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Compute $\mod{\mathtt{self}}_{\mathtt{modulus}}$.
    #[must_use]
    pub fn reduce_mod(mut self, modulus: Self) -> Self {
        if modulus == Self::ZERO {
            return Self::ZERO;
        }
        if self >= modulus {
            self %= modulus;
        }
        self
    }

    /// Compute $\mod{\mathtt{self} + \mathtt{rhs}}_{\mathtt{modulus}}$.
    #[must_use]
    pub fn add_mod(self, rhs: Self, modulus: Self) -> Self {
        // Reduce inputs
        let lhs = self.reduce_mod(modulus);
        let rhs = rhs.reduce_mod(modulus);

        // Compute the sum and conditionaly subtract modulus once.
        let (mut result, overflow) = lhs.overflowing_add(rhs);
        if overflow || result >= modulus {
            result -= modulus;
        }
        result
    }

    /// Compute $\mod{\mathtt{self} ⋅ \mathtt{rhs}}_{\mathtt{modulus}}$.
    #[must_use]
    pub fn mul_mod(self, rhs: Self, mut modulus: Self) -> Self {
        if modulus == Self::ZERO {
            return Self::ZERO;
        }
        // Compute full product.
        // The challenge here is that Rust doesn't allow us to create a
        // `Uint<2 * BITS, _>` for the intermediate result. Otherwise
        // we could just use a `widening_mul`. So instead we allocate from heap.
        // Alternatively we could use `alloca`, but that is blocked on
        // See <https://github.com/rust-lang/rust/issues/48055>
        let mut product = vec![0; nlimbs(2 * BITS)];
        let overflow = algorithms::mul_inline(&self.limbs, &rhs.limbs, &mut product);
        debug_assert!(!overflow);

        // Compute modulus using `div_rem`.
        // This stores the remainder in the divisor, `modulus`.
        algorithms::div_rem(&mut product, &mut modulus.limbs);

        modulus
    }
}

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
            proptest!(|(a: U, b: U, m: U)| {
                assert_eq!(a.mul_mod(b, m), b.mul_mod(a, m));
            });
        });
    }

    #[test]
    fn test_associative() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(a: U, b: U, c: U, m: U)| {
                assert_eq!(a.mul_mod(b.mul_mod(c, m), m), a.mul_mod(b, m).mul_mod(c, m));
            });
        });
    }

    #[test]
    fn test_distributive() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(a: U, b: U, c: U, m: U)| {
                assert_eq!(a.mul_mod(b.add_mod(c, m), m), a.mul_mod(b, m).add_mod(a.mul_mod(c, m), m));
            });
        });
    }

    #[test]
    fn test_add_identity() {
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(value: U, m: U)| {
                assert_eq!(value.add_mod(U::from(0), m), value.reduce_mod(m));
            });
        });
    }

    #[test]
    fn test_mul_identity() {
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(value: U, m: U)| {
                assert_eq!(value.mul_mod(U::from(0), m), U::ZERO);
                assert_eq!(value.mul_mod(U::from(1), m), value.reduce_mod(m));
            });
        });
    }

    // #[test]
    // fn test_inverse() {
    //     const_for!(BITS in NON_ZERO {
    //         const LIMBS: usize = nlimbs(BITS);
    //         type U = Uint<BITS, LIMBS>;
    //         proptest!(|(mut a: U)| {
    //             a |= U::from(1); // Make sure a is invertible
    //             assert_eq!(a * a.ring_inverse().unwrap(), U::from(1));
    //             assert_eq!(a.ring_inverse().unwrap().ring_inverse().unwrap(),
    // a);         });
    //     });
    // }
}
