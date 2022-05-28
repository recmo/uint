#![cfg(feature = "dyn")]
#![cfg(feature = "unstable")]
use smallvec::SmallVec;

/// Dynamically sized unsigned integer type.
///
/// *Warning* This type is unstable can break in semver-compatible minor
/// releases.
pub struct UintDyn {
    _limbs: SmallVec<[u64; 2]>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Uint;

    #[test]
    fn test_sizes() {
        // UintDyn has one word overhead, but two words free compared to plain Vec.
        dbg!(std::mem::size_of::<Uint<64, 2>>());
        dbg!(std::mem::size_of::<Uint<64, 3>>());
        dbg!(std::mem::size_of::<Vec<u64>>());
        dbg!(std::mem::size_of::<UintDyn>());
    }
}
