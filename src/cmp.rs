use crate::Uint;
use core::cmp::{Ord, Ordering, PartialOrd};

impl<const BITS: usize, const LIMBS: usize> Ord for Uint<BITS, LIMBS> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        for (lhs, rhs) in self
            .as_limbs()
            .iter()
            .rev()
            .zip(rhs.as_limbs().iter().rev())
        {
            match lhs.cmp(rhs) {
                Ordering::Equal => continue,
                other => return other,
            }
        }
        Ordering::Equal
    }
}

impl<const BITS: usize, const LIMBS: usize> PartialOrd for Uint<BITS, LIMBS> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Check if this uint is zero
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self == &Self::ZERO
    }
}

#[test]
fn test_is_zero() {
    assert!(Uint::<0, 0>::ZERO.is_zero());
    assert!(Uint::<1, 1>::ZERO.is_zero());
    assert!(Uint::<7, 1>::ZERO.is_zero());
    assert!(Uint::<64, 1>::ZERO.is_zero());

    assert!(!Uint::<1, 1>::from_limbs([1]).is_zero());
    assert!(!Uint::<7, 1>::from_limbs([1]).is_zero());
    assert!(!Uint::<64, 1>::from_limbs([1]).is_zero());
}
