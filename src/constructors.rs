use crate::{nlimbs, Uint};

impl<const BITS: usize> Uint<BITS>
where
    [(); nlimbs(BITS)]:,
{
    #[must_use]
    pub const fn zero() -> Self {
        Self::from_limbs([0; nlimbs(BITS)])
    }

    /// # Panics
    /// Panics if the bit size is zero.
    #[must_use]
    #[track_caller]
    pub const fn one() -> Self {
        assert!(Self::BITS > 0, "Can not represent one in Uint<0>");
        let mut result = Self::zero();
        result.limbs[0] = 1;
        result
    }

    #[must_use]
    pub fn from_limbs_slice(slice: &[u64]) -> Self {
        let mut limbs = [0; nlimbs(BITS)];
        limbs.copy_from_slice(slice);
        Self { limbs }
    }
}
