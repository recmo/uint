#![cfg(feature = "proptest")]
use super::{num_limbs, Uint};
use proptest::{
    collection::{vec, VecStrategy},
    num::u64::Any,
    prelude::*,
};

pub fn arb_uint<const BITS: usize>() -> impl Strategy<Value = Uint<BITS>>
where
    [(); num_limbs(BITS)]:,
{
    // TODO: Copy [`UniformArrayStrategy`] to avoid heap allocations
    let limb: Any = any::<u64>();
    let limbs: VecStrategy<Any> = vec(limb, num_limbs(BITS));
    limbs.prop_map(|limbs| Uint::from_limbs_slice(&limbs))
}
