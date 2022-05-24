use crate::{algorithms::div::divrem, impl_bin_op, Uint};
use core::ops::{Div, DivAssign, Rem, RemAssign};

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Computes `self / rhs`, returning [`None`] if `rhs == 0`.
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs == Self::ZERO {
            return None;
        }
        todo!() // Some(self / rhs)
    }
    /// Computes `self % rhs`, returning [`None`] if `rhs == 0`.
    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        if rhs == Self::ZERO {
            return None;
        }
        todo!() // Some(self % rhs)
    }

    pub fn div_ceil(self, rhs: Self) -> Self {
        assert!(rhs != Self::ZERO);
        let (q, r) = self.div_rem(rhs);
        if r != Self::ZERO {
            q + Self::from(1)
        } else {
            q
        }
    }

    pub fn div_rem(self, rhs: Self) -> (Self, Self) {
        assert!(rhs != Self::ZERO);
        let mut result = vec![0_u64; LIMBS + 1];
        let mut divisor = [0_u64; LIMBS];
        result[..LIMBS].copy_from_slice(&self.limbs);
        divisor[..LIMBS].copy_from_slice(&rhs.limbs);
        divrem(&mut result, &mut divisor);
        (
            Self::from_limbs_slice(&result[..LIMBS]),
            Self::from_limbs(divisor),
        )
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{const_for, nlimbs};
    use proptest::{prop_assume, proptest};

    #[test]
    fn test_divrem() {
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(n: U, mut d: U)| {
                d >>= BITS / 2; // make d small
                prop_assume!(d != U::ZERO);
                let (q, r) = n.div_rem(d);
                assert!(r < d);
                assert_eq!(q * d + r, n);
            });
            proptest!(|(n: U, d: U)| {
                prop_assume!(d != U::ZERO);
                let (q, r) = n.div_rem(d);
                assert!(r < d);
                assert_eq!(q * d + r, n);
            });
        });
    }
}
