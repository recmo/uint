#![cfg(feature = "dyn")]
use smallvec::SmallVec;

/// Dynamically sized unsigned integer type.
pub struct UintDyn {
    limbs: SmallVec<[u64; 2]>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Uint;

    #[test]
    fn test_sizes() {
        dbg!(std::mem::size_of::<Uint<64, 1>>());
        dbg!(std::mem::size_of::<Vec<u64>>());
        dbg!(std::mem::size_of::<UintDyn>());
    }
}
