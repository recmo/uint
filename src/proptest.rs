#![cfg(feature = "proptest")]
use super::{nlimbs, Uint};
use proptest::{
    arbitrary::Arbitrary,
    collection::{vec, VecStrategy},
    num::u64::Any,
    strategy::{BoxedStrategy, Strategy},
};

impl<const BITS: usize> Arbitrary for Uint<BITS>
where
    [(); nlimbs(BITS)]:,
{
    // TODO: Would be nice to have a value range as parameter
    // and/or a choice between uniform and 'exponential' distribution.
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> BoxedStrategy<Self> {
        // TODO: Copy [`UniformArrayStrategy`] to avoid heap allocations
        let limbs: VecStrategy<Any> = vec(u64::arbitrary(), nlimbs(BITS));
        limbs
            .prop_map(|limbs| Self::from_limbs_slice(&limbs))
            .boxed()
    }
}
