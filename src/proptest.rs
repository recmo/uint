#![cfg(feature = "proptest")]
use super::{nlimbs, Uint};
use proptest::{
    collection::{vec, VecStrategy},
    num::u64::Any,
    prelude::*,
};

impl<const BITS: usize> Uint<BITS>
where
    [(); nlimbs(BITS)]:,
{
    pub fn arb() -> impl Strategy<Value = Self> {
        // TODO: Copy [`UniformArrayStrategy`] to avoid heap allocations
        let limb: Any = any::<u64>();
        let limbs: VecStrategy<Any> = vec(limb, nlimbs(BITS));
        limbs.prop_map(|limbs| Uint::from_limbs_slice(&limbs))
    }
}
