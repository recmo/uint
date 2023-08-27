//! Support for the [`num-traits`](https://crates.io/crates/num-traits) crate.
#![cfg(feature = "num-traits")]
#![cfg_attr(docsrs, doc(cfg(feature = "num-traits")))]
// This is a particularly big risk with these traits. Make sure
// to call functions on the `Uint::` type.
#![deny(unconditional_recursion)]
use crate::Uint;
use core::ops::{Shl, Shr};
use num_traits::{
    bounds::*,
    ops::{bytes::*, checked::*, overflowing::*, saturating::*, wrapping::*, *},
    *,
};

// TODO: cast::* PrimInt

// Note. We can not implement `NumBytes` as it requires T to be `AsMut<[u8]>`.
// This is not safe for `Uint` when `BITS % 8 != 0`.

impl<const BITS: usize, const LIMBS: usize> Zero for Uint<BITS, LIMBS> {
    #[inline(always)]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self == &Self::ZERO
    }
}

impl<const BITS: usize, const LIMBS: usize> One for Uint<BITS, LIMBS> {
    #[inline(always)]
    fn one() -> Self {
        Uint::from(1)
    }
}

impl<const BITS: usize, const LIMBS: usize> Bounded for Uint<BITS, LIMBS> {
    #[inline(always)]
    fn min_value() -> Self {
        Self::ZERO
    }

    #[inline(always)]
    fn max_value() -> Self {
        Self::MAX
    }
}

impl<const BITS: usize, const LIMBS: usize> FromBytes for Uint<BITS, LIMBS> {
    type Bytes = [u8];

    #[inline(always)]
    fn from_le_bytes(bytes: &[u8]) -> Self {
        Self::try_from_le_slice(bytes).unwrap()
    }

    #[inline(always)]
    fn from_be_bytes(bytes: &[u8]) -> Self {
        Self::try_from_be_slice(bytes).unwrap()
    }
}

impl<const BITS: usize, const LIMBS: usize> ToBytes for Uint<BITS, LIMBS> {
    type Bytes = Vec<u8>;

    #[inline(always)]
    fn to_le_bytes(&self) -> Self::Bytes {
        self.to_le_bytes_vec()
    }

    #[inline(always)]
    fn to_be_bytes(&self) -> Self::Bytes {
        self.to_be_bytes_vec()
    }
}

impl<const BITS: usize, const LIMBS: usize> CheckedAdd for Uint<BITS, LIMBS> {
    #[inline(always)]
    fn checked_add(&self, other: &Self) -> Option<Self> {
        <Self>::checked_add(*self, *other)
    }
}

impl<const BITS: usize, const LIMBS: usize> CheckedDiv for Uint<BITS, LIMBS> {
    #[inline(always)]
    fn checked_div(&self, other: &Self) -> Option<Self> {
        <Self>::checked_div(*self, *other)
    }
}

impl<const BITS: usize, const LIMBS: usize> CheckedMul for Uint<BITS, LIMBS> {
    #[inline(always)]
    fn checked_mul(&self, other: &Self) -> Option<Self> {
        <Self>::checked_mul(*self, *other)
    }
}

impl<const BITS: usize, const LIMBS: usize> CheckedNeg for Uint<BITS, LIMBS> {
    #[inline(always)]
    fn checked_neg(&self) -> Option<Self> {
        <Self>::checked_neg(*self)
    }
}

impl<const BITS: usize, const LIMBS: usize> CheckedRem for Uint<BITS, LIMBS> {
    #[inline(always)]
    fn checked_rem(&self, other: &Self) -> Option<Self> {
        <Self>::checked_rem(*self, *other)
    }
}

// TODO: Move out of support.
impl<const BITS: usize, const LIMBS: usize> Shl<u32> for Uint<BITS, LIMBS> {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: u32) -> Self::Output {
        <Self>::shl(self, rhs as usize)
    }
}

// TODO: Move out of support lib into.
impl<const BITS: usize, const LIMBS: usize> Shr<u32> for Uint<BITS, LIMBS> {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: u32) -> Self::Output {
        <Self>::shr(self, rhs as usize)
    }
}

impl<const BITS: usize, const LIMBS: usize> CheckedShl for Uint<BITS, LIMBS> {
    fn checked_shl(&self, other: u32) -> Option<Self> {
        Uint::checked_shl(*self, other as usize)
    }
}

impl<const BITS: usize, const LIMBS: usize> CheckedShr for Uint<BITS, LIMBS> {
    fn checked_shr(&self, other: u32) -> Option<Self> {
        Uint::checked_shr(*self, other as usize)
    }
}

impl<const BITS: usize, const LIMBS: usize> CheckedSub for Uint<BITS, LIMBS> {
    #[inline(always)]
    fn checked_sub(&self, other: &Self) -> Option<Self> {
        <Self>::checked_sub(*self, *other)
    }
}

impl<const BITS: usize, const LIMBS: usize> CheckedEuclid for Uint<BITS, LIMBS> {
    #[inline(always)]
    fn checked_div_euclid(&self, v: &Self) -> Option<Self> {
        <Self>::checked_div(*self, *v)
    }

    #[inline(always)]
    fn checked_rem_euclid(&self, v: &Self) -> Option<Self> {
        <Self>::checked_rem(*self, *v)
    }
}

impl<const BITS: usize, const LIMBS: usize> Euclid for Uint<BITS, LIMBS> {
    #[inline(always)]
    fn div_euclid(&self, v: &Self) -> Self {
        <Self>::wrapping_div(*self, *v)
    }

    #[inline(always)]
    fn rem_euclid(&self, v: &Self) -> Self {
        <Self>::wrapping_rem(*self, *v)
    }
}

impl<const BITS: usize, const LIMBS: usize> Inv for Uint<BITS, LIMBS> {
    type Output = Option<Self>;

    #[inline(always)]
    fn inv(self) -> Self::Output {
        <Self>::inv_ring(self)
    }
}

impl<const BITS: usize, const LIMBS: usize> MulAdd for Uint<BITS, LIMBS> {
    type Output = Self;

    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self::Output {
        // OPT: Expose actual merged mul_add algo.
        (self * a) + b
    }
}

impl<const BITS: usize, const LIMBS: usize> MulAddAssign for Uint<BITS, LIMBS> {
    #[inline(always)]
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        *self *= a;
        *self += b;
    }
}

impl<const BITS: usize, const LIMBS: usize> Saturating for Uint<BITS, LIMBS> {
    #[inline(always)]
    fn saturating_add(self, v: Self) -> Self {
        <Self>::saturating_add(self, v)
    }

    #[inline(always)]
    fn saturating_sub(self, v: Self) -> Self {
        <Self>::saturating_sub(self, v)
    }
}

macro_rules! binary_op {
    ($($trait:ident $fn:ident)*) => {$(
        impl<const BITS: usize, const LIMBS: usize> $trait for Uint<BITS, LIMBS> {
            #[inline(always)]
            fn $fn(&self, v: &Self) -> Self {
                <Self>::$fn(*self, *v)
            }
        }
    )*};
}

binary_op! {
    SaturatingAdd saturating_add
    SaturatingSub saturating_sub
    SaturatingMul saturating_mul
    WrappingAdd wrapping_add
    WrappingSub wrapping_sub
    WrappingMul wrapping_mul
}

impl<const BITS: usize, const LIMBS: usize> WrappingNeg for Uint<BITS, LIMBS> {
    #[inline(always)]
    fn wrapping_neg(&self) -> Self {
        <Self>::wrapping_neg(*self)
    }
}

impl<const BITS: usize, const LIMBS: usize> WrappingShl for Uint<BITS, LIMBS> {
    #[inline(always)]
    fn wrapping_shl(&self, rhs: u32) -> Self {
        <Self>::wrapping_shl(*self, rhs as usize)
    }
}

impl<const BITS: usize, const LIMBS: usize> WrappingShr for Uint<BITS, LIMBS> {
    #[inline(always)]
    fn wrapping_shr(&self, rhs: u32) -> Self {
        <Self>::wrapping_shr(*self, rhs as usize)
    }
}

impl<const BITS: usize, const LIMBS: usize> Num for Uint<BITS, LIMBS> {
    type FromStrRadixErr = crate::ParseError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        <Self>::from_str_radix(str, radix as u64)
    }
}

impl<const BITS: usize, const LIMBS: usize> Pow<Self> for Uint<BITS, LIMBS> {
    type Output = Self;

    fn pow(self, rhs: Self) -> Self::Output {
        <Self>::pow(self, rhs)
    }
}

impl<const BITS: usize, const LIMBS: usize> Unsigned for Uint<BITS, LIMBS> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aliases::U256;

    macro_rules! assert_impl{
        ($type:ident, $($trait:tt),*) => {
            $({
                fn assert_impl<T: $trait>() {}
                assert_impl::<$type>();
            })*
        }
    }

    #[test]
    fn test_assert_impl() {
        // All applicable traits from num-traits
        assert_impl!(U256, Bounded, LowerBounded, UpperBounded);
        // assert_impl!(U256, AsPrimitive, FromPrimitive, NumCast, ToPrimitive);
        assert_impl!(U256, One, Zero);
        // assert_impl!(U256, PrimInt);
        assert_impl!(U256, FromBytes, ToBytes);
        assert_impl!(
            U256, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedSub,
            CheckedShl, CheckedShr, CheckedSub
        );
        assert_impl!(U256, CheckedEuclid, Euclid);
        assert_impl!(U256, Inv);
        assert_impl!(U256, MulAdd, MulAddAssign);
        // assert_impl!(U256, OverflowingAdd, OverflowingMul, OverflowingSub);
        assert_impl!(
            U256,
            Saturating,
            SaturatingAdd,
            SaturatingMul,
            SaturatingSub
        );
        assert_impl!(
            U256,
            WrappingAdd,
            WrappingMul,
            WrappingNeg,
            WrappingShl,
            WrappingShr,
            WrappingSub
        );
        assert_impl!(U256, (Pow<U256>));
        assert_impl!(U256, Unsigned);
    }
}
