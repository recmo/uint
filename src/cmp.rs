use crate::{algorithms, Uint};
use core::cmp::Ordering;

macro_rules! cmp_fns {
    ($($name:ident, $op:tt => |$a:ident, $b:ident| $impl:expr),* $(,)?) => {
        $(
            #[inline]
            fn $name(&self, $b: &Self) -> bool {
                let $a = self;
                as_primitives!($a, $b; {
                    u64(x, y) => return x $op y,
                    u128(x, y) => return x $op y,
                });

                $impl
            }
        )*
    };
}

impl<const BITS: usize, const LIMBS: usize> PartialOrd for Uint<BITS, LIMBS> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    cmp_fns! {
        lt, <  => |a, b| algorithms::lt(a.as_limbs(), b.as_limbs()),
        gt, >  => |a, b| Self::lt(b, a),
        ge, >= => |a, b| !Self::lt(a, b),
        le, <= => |a, b| !Self::lt(b, a),
    }
}

impl<const BITS: usize, const LIMBS: usize> Ord for Uint<BITS, LIMBS> {
    #[inline]
    fn cmp(&self, rhs: &Self) -> Ordering {
        as_primitives!(self, rhs; {
            u64(x, y) => return x.cmp(&y),
            u128(x, y) => return x.cmp(&y),
        });

        // NOTE: This currently uses `overflowing_sub` instead of `algorithms::cmp`
        // to make use of `r.is_zero()`. The accumulator version in `algorithms::cmp`
        // includes the hack in `algorithms::cmp::sub` which is slower. This should be
        // switched in the future once the hack is fixed.
        let (r, o) = self.overflowing_sub(*rhs);
        if r.is_zero() {
            Ordering::Equal
        } else if o {
            Ordering::Less
        } else {
            Ordering::Greater
        }
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

    fn exhaustive_proptest<T, U>(a: T, b: T) -> Result<(), proptest::prelude::TestCaseError>
    where
        T: Copy + Ord + Eq + core::fmt::Debug,
        U: Copy + Ord + Eq + core::fmt::Debug + TryFrom<T>,
        U::Error: core::fmt::Debug,
    {
        let x = U::try_from(a).unwrap();
        let y = U::try_from(b).unwrap();
        exhaustive_proptest_impl(a, b, x, y)
    }

    fn exhaustive_proptest_impl<T, U>(
        a: T,
        b: T,
        x: U,
        y: U,
    ) -> Result<(), proptest::prelude::TestCaseError>
    where
        T: Copy + Ord + Eq + core::fmt::Debug,
        U: Copy + Ord + Eq + core::fmt::Debug + TryFrom<T>,
        U::Error: core::fmt::Debug,
    {
        prop_assert_eq!(x == y, a == b);
        prop_assert_eq!(y == x, b == a);

        prop_assert_eq!(x != y, a != b);
        prop_assert_eq!(y != x, b != a);

        prop_assert_eq!(x.cmp(&y), a.cmp(&b));
        prop_assert_eq!(x < y, a < b);
        prop_assert_eq!(x > y, a > b);
        prop_assert_eq!(x >= y, a >= b);
        prop_assert_eq!(x <= y, a <= b);

        prop_assert_eq!(y.cmp(&x), b.cmp(&a));
        prop_assert_eq!(y < x, b < a);
        prop_assert_eq!(y > x, b > a);
        prop_assert_eq!(y >= x, b >= a);
        prop_assert_eq!(y <= x, b <= a);

        Ok(())
    }

    proptest::proptest! {
        #[test]
        fn test_cmp_u64(a: u64, b: u64) {
            exhaustive_proptest::<u64, Uint<64, 1>>(a, b)?;
        }

        #[test]
        fn test_cmp_u128_half(a: u64, b: u64) {
            exhaustive_proptest::<u64, Uint<128, 2>>(a, b)?;
        }

        #[test]
        fn test_cmp_u128_full(a: u128, b: u128) {
            exhaustive_proptest::<u128, Uint<128, 2>>(a, b)?;
        }

        #[test]
        fn test_cmp_u192(a: u128, b: u128) {
            exhaustive_proptest::<u128, Uint<192, 3>>(a, b)?;
        }

        #[test]
        fn test_cmp_u256(a: u128, b: u128) {
            exhaustive_proptest::<u128, Uint<256, 4>>(a, b)?;
        }
    }

    #[test]
    fn test_cmp_all() {
        crate::const_for!(BITS in SIZES {
            const LIMBS: usize = crate::nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            if BITS > 128 {
                return;
            }
            proptest::proptest!(|(x: U, y: U)| {
                let Ok(a) = u128::try_from(x) else {
                    proptest::prop_assume!(false);
                    return Ok(());
                };
                let Ok(b) = u128::try_from(y) else {
                    proptest::prop_assume!(false);
                    return Ok(());
                };
                exhaustive_proptest_impl::<u128, U>(a, b, x, y)?;
            });
        });
    }
}
