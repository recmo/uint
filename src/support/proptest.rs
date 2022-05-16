#![cfg(feature = "proptest")]
use crate::{nlimbs, Uint};
use proptest::{
    arbitrary::Arbitrary,
    collection::{vec, VecStrategy},
    num::u64::Any,
    strategy::{BoxedStrategy, Strategy},
};

impl<const BITS: usize, const LIMBS: usize> Arbitrary for Uint<BITS, LIMBS> {
    // TODO: Would be nice to have a value range as parameter
    // and/or a choice between uniform and 'exponential' distribution.
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> BoxedStrategy<Self> {
        // TODO: Copy [`UniformArrayStrategy`] to avoid heap allocations
        let limbs: VecStrategy<Any> = vec(u64::arbitrary(), nlimbs(BITS));
        limbs
            .prop_map(|mut limbs| {
                if Self::LIMBS > 0 {
                    limbs[Self::LIMBS - 1] &= Self::MASK;
                }
                Self::from_limbs_slice(&limbs)
            })
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::const_for;
    use proptest::proptest;

    #[test]
    fn test_arbitrary() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            proptest!(|(n in Uint::<BITS, LIMBS>::arbitrary())| {
                let _ = n;
            });
        });
    }
}
