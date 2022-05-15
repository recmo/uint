#![cfg(feature = "quickcheck")]
use crate::{nlimbs, Uint};
use quickcheck::{Arbitrary, Gen};

impl<const BITS: usize> Arbitrary for Uint<BITS>
where
    [(); nlimbs(BITS)]:,
{
    fn arbitrary(g: &mut Gen) -> Self {
        let mut limbs = [0; nlimbs(BITS)];
        if let Some((last, rest)) = limbs.split_last_mut() {
            for limb in rest.iter_mut() {
                *limb = u64::arbitrary(g);
            }
            *last = u64::arbitrary(g) & Self::MASK;
        }
        Self::from_limbs(limbs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repeat;
    use quickcheck::quickcheck;

    fn test_quickcheck_inner<const BITS: usize>(_n: Uint<BITS>) -> bool
    where
        [(); nlimbs(BITS)]:,
    {
        true
    }

    #[test]
    fn test_quickcheck() {
        repeat!({
            quickcheck(test_quickcheck_inner::<N> as fn(Uint<N>) -> bool);
        });
    }
}
