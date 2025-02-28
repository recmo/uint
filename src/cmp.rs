use crate::Uint;
use core::cmp::Ordering;

impl<const BITS: usize, const LIMBS: usize> PartialOrd for Uint<BITS, LIMBS> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const BITS: usize, const LIMBS: usize> Ord for Uint<BITS, LIMBS> {
    #[inline]
    fn cmp(&self, rhs: &Self) -> Ordering {
        crate::algorithms::cmp(self.as_limbs(), rhs.as_limbs())
    }
}

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Returns true if the value is zero.
    #[inline]
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.const_eq(Self::ZERO)
    }

    /// Returns `true` if `self` equals `other`.
    ///
    /// Note that this currently performs worse than the derived `PartialEq`
    /// implementation when one of the numbers is known at compile-time due to
    /// language limitations.
    // https://godbolt.org/z/aM1nE1Pfh
    #[inline]
    #[must_use]
    pub const fn const_eq(self, other: Self) -> bool {
        // TODO: Replace with `self == other` and deprecate once `PartialEq` is const.
        // let a = self.as_bytes();
        // let b = other.as_bytes();
        let a = self.as_limbs();
        let b = other.as_limbs();
        let mut i = 0;
        let mut r = true;
        while i < a.len() {
            r &= a[i] == b[i];
            i += 1;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use crate::Uint;

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
}
