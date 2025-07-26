use crate::{algorithms, Uint};
use core::cmp::Ordering;

impl<const BITS: usize, const LIMBS: usize> PartialOrd for Uint<BITS, LIMBS> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        as_primitives!(self, other; {
            u64(x, y) => return x < y,
            u128(x, y) => return x < y,
        });

        algorithms::lt(self.as_limbs(), other.as_limbs())
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        other.lt(self)
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        !self.lt(other)
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        !other.lt(self)
    }
}

impl<const BITS: usize, const LIMBS: usize> Ord for Uint<BITS, LIMBS> {
    #[inline]
    fn cmp(&self, rhs: &Self) -> Ordering {
        algorithms::cmp(self.as_limbs(), rhs.as_limbs())
    }
}

/// Implements `PartialEq` and `PartialOrd` for `Uint` and primitive integers.
///
/// This intentionally does not use `<$t>::try_from` to avoid unnecessary
/// checks for non-limb-sized primitive integers.
macro_rules! impl_for_primitives {
    ($($t:ty),* $(,)?) => {
        $(
            impl<const BITS: usize, const LIMBS: usize> PartialEq<$t> for Uint<BITS, LIMBS> {
                #[inline]
                #[allow(unused_comparisons)] // Both signed and unsigned integers use this.
                #[allow(clippy::cast_possible_truncation)] // Unreachable.
                fn eq(&self, &other: &$t) -> bool {
                    (other >= 0) & (if <$t>::BITS <= u64::BITS {
                        u64::try_from(self).ok() == Some(other as u64)
                    } else {
                        u128::try_from(self).ok() == Some(other as u128)
                    })
                }
            }

            impl<const BITS: usize, const LIMBS: usize> PartialOrd<$t> for Uint<BITS, LIMBS> {
                #[inline]
                #[allow(unused_comparisons)] // Both signed and unsigned integers use this.
                #[allow(clippy::cast_possible_truncation)] // Unreachable.
                fn partial_cmp(&self, &other: &$t) -> Option<Ordering> {
                    if other < 0 {
                        return Some(Ordering::Greater);
                    }

                    if <$t>::BITS <= u64::BITS {
                        let Ok(self_t) = u64::try_from(self) else {
                            return Some(Ordering::Greater);
                        };
                        self_t.partial_cmp(&(other as u64))
                    } else {
                        let Ok(self_t) = u128::try_from(self) else {
                            return Some(Ordering::Greater);
                        };
                        self_t.partial_cmp(&(other as u128))
                    }
                }
            }
        )*
    };
}

#[rustfmt::skip]
impl_for_primitives!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
);

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Returns `true` if the value is zero.
    #[inline]
    #[must_use]
    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }

    /// Returns `true` if the value is zero.
    ///
    /// Note that this currently might perform worse than
    /// [`is_zero`](Self::is_zero), so prefer that if possible.
    #[inline]
    #[must_use]
    pub const fn const_is_zero(&self) -> bool {
        as_primitives!(self; {
            u64(x) => return x == 0,
            u128(x) => return x == 0,
            u256((lo, hi)) => return (lo | hi) == 0,
        });

        self.const_eq(&Self::ZERO)
    }

    /// Returns `true` if `self` equals `other`.
    ///
    /// Note that this currently might perform worse than the derived
    /// `PartialEq` (`==` operator), so prefer that if possible.
    #[inline]
    #[must_use]
    pub const fn const_eq(&self, other: &Self) -> bool {
        as_primitives!(self, other; {
            u64(x, y) => return x == y,
            u128(x, y) => return x == y,
            u256((lo, hi), (lo2, hi2)) => return (lo == lo2) & (hi == hi2),
        });

        // TODO: Replace with `self == other` and deprecate once `PartialEq` is const.
        let a = self.as_limbs();
        let b = other.as_limbs();
        let mut i = 0;
        let mut r = 0;
        while i < LIMBS {
            r |= a[i] ^ b[i];
            i += 1;
        }
        r == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::Uint;
    use proptest::prop_assert_eq;

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

    proptest::proptest! {
        #[test]
        fn test_cmp_u64(a: u64, b: u64) {
            let x = Uint::<64, 1>::from(a);
            let y = Uint::<64, 1>::from(b);
            prop_assert_eq!(x.cmp(&y), a.cmp(&b));
            prop_assert_eq!(x.lt(&y), a < b);
            prop_assert_eq!(x.gt(&y), a > b);
            prop_assert_eq!(x.ge(&y), a >= b);
            prop_assert_eq!(x.le(&y), a <= b);
        }

        #[test]
        fn test_cmp_u128(a: u128, b: u128) {
            let x = Uint::<128, 2>::from(a);
            let y = Uint::<128, 2>::from(b);
            prop_assert_eq!(x.cmp(&y), a.cmp(&b));
            prop_assert_eq!(x.lt(&y), a < b);
            prop_assert_eq!(x.gt(&y), a > b);
            prop_assert_eq!(x.ge(&y), a >= b);
            prop_assert_eq!(x.le(&y), a <= b);
        }
    }
}
