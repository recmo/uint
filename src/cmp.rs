use crate::{Bits, Uint};
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

impl<const BITS: usize, const LIMBS: usize> Ord for Bits<BITS, LIMBS> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.as_uint().cmp(rhs.as_uint())
    }
}

impl<const BITS: usize, const LIMBS: usize> PartialOrd for Bits<BITS, LIMBS> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
