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
