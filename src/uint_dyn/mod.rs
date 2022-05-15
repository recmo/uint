#![cfg(feature = "dyn")]
use smallvec::SmallVec;

/// Dynamically sized unsigned integer type.
pub struct UintDyn {
    limbs: SmallVec<[u64; 2]>,
}
