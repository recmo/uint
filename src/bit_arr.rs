// TODO: Forward `const fn` as `const fn`.
#![allow(clippy::missing_const_for_fn)]

use crate::Uint;
use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not, Shl, ShlAssign,
    Shr, ShrAssign,
};
use std::borrow::Cow;

/// A newtype wrapper around [`Uint`] that restricts operations to those
/// relevant for bit arrays.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct Bits<const BITS: usize, const LIMBS: usize>(Uint<BITS, LIMBS>);

impl<const BITS: usize, const LIMBS: usize> Bits<BITS, LIMBS> {
    /// The size of this integer type in 64-bit limbs.
    pub const LIMBS: usize = Uint::<BITS, LIMBS>::LIMBS;

    /// The size of this integer type in bits.
    pub const BITS: usize = Uint::<BITS, LIMBS>::BITS;

    /// The size of this integer type in bits.
    pub const BYTES: usize = Uint::<BITS, LIMBS>::BYTES;

    /// The value zero. This is the only value that exists in all [`Uint`]
    /// types.
    pub const ZERO: Self = Self(Uint::<BITS, LIMBS>::ZERO);

    #[must_use]
    pub const fn into_inner(self) -> Uint<BITS, LIMBS> {
        self.0
    }

    #[must_use]
    pub const fn as_uint(&self) -> &Uint<BITS, LIMBS> {
        &self.0
    }

    #[must_use]
    pub fn as_uint_mut(&mut self) -> &mut Uint<BITS, LIMBS> {
        &mut self.0
    }
}

impl<const BITS: usize, const LIMBS: usize> From<Uint<BITS, LIMBS>> for Bits<BITS, LIMBS> {
    fn from(x: Uint<BITS, LIMBS>) -> Self {
        Self(x)
    }
}

impl<const BITS: usize, const LIMBS: usize> From<Bits<BITS, LIMBS>> for Uint<BITS, LIMBS> {
    fn from(x: Bits<BITS, LIMBS>) -> Self {
        x.0
    }
}

// Limitations of declarative macro matching force us to break down on argument
// patterns.
macro_rules! forward {
    ($(fn $fnname:ident(self) -> $res:ty;)*) => {
        $(
            #[doc = concat!("See [`Uint::", stringify!($fnname),"`] for documentation.")]
            #[allow(clippy::inline_always)]
            #[inline(always)]
            #[must_use]
            pub fn $fnname(self) -> $res {
                Uint::$fnname(self.0).into()
            }
        )*
    };
    ($(fn $fnname:ident$(<$(const $generic_arg:ident:$generic_ty:ty),+>)?(&self) -> $res:ty;)*) => {
        $(
            #[doc = concat!("See [`Uint::", stringify!($fnname),"`] for documentation.")]
            #[allow(clippy::inline_always)]
            #[inline(always)]
            #[must_use]
            pub fn $fnname$(<$(const $generic_arg:$generic_ty),+>)?(&self) -> $res {
                Uint::$fnname(&self.0).into()
            }
        )*
    };
    ($(unsafe fn $fnname:ident(&mut self) -> $res:ty;)*) => {
        $(
            #[doc = concat!("See [`Uint::", stringify!($fnname),"`] for documentation.")]
            #[allow(clippy::inline_always)]
            #[inline(always)]
            #[must_use]
            pub unsafe fn $fnname(&mut self) -> $res {
                Uint::$fnname(&mut self.0).into()
            }
        )*
    };
    ($(fn $fnname:ident(self, $arg:ty) -> Option<Self>;)*) => {
        $(
            #[doc = concat!("See [`Uint::", stringify!($fnname),"`] for documentation.")]
            #[allow(clippy::inline_always)]
            #[inline(always)]
            #[must_use]
            pub fn $fnname(self, a: $arg) -> Option<Self> {
                Uint::$fnname(self.0, a).map(Bits::from)
            }
        )*
    };
    ($(fn $fnname:ident(self, $arg:ty) -> (Self, bool);)*) => {
        $(
            #[doc = concat!("See [`Uint::", stringify!($fnname),"`] for documentation.")]
            #[allow(clippy::inline_always)]
            #[inline(always)]
            #[must_use]
            pub fn $fnname(self, a: $arg) -> (Self, bool) {
                let (value, flag) = Uint::$fnname(self.0, a);
                (value.into(), flag)
            }
        )*
    };
    ($(fn $fnname:ident(self, $arg:ty) -> $res:ty;)*) => {
        $(
            #[doc = concat!("See [`Uint::", stringify!($fnname),"`] for documentation.")]
            #[allow(clippy::inline_always)]
            #[inline(always)]
            #[must_use]
            pub fn $fnname(self, a: $arg) -> $res {
                Uint::$fnname(self.0, a).into()
            }
        )*
    };
    ($(fn $fnname:ident$(<$(const $generic_arg:ident:$generic_ty:ty),+>)?($($arg:ident:$arg_ty:ty),+) -> $res:ty;)*) => {
        $(
            #[doc = concat!("See [`Uint::", stringify!($fnname),"`] for documentation.")]
            #[allow(clippy::inline_always)]
            #[inline(always)]
            #[must_use]
            pub fn $fnname$(<$(const $generic_arg: $generic_ty),+>)?($($arg: $arg_ty),+) -> $res {
                Uint::$fnname($($arg),+).into()
            }
        )*
    };
}

impl<const BITS: usize, const LIMBS: usize> Bits<BITS, LIMBS> {
    forward! {
        fn reverse_bits(self) -> Self;
    }
    forward! {
        fn as_le_bytes(&self) -> Cow<'_, [u8]>;
        fn to_le_bytes<const BYTES: usize>(&self) -> [u8; BYTES];
        fn to_be_bytes<const BYTES: usize>(&self) -> [u8; BYTES];
        fn as_limbs(&self) -> &[u64; LIMBS];
        fn leading_zeros(&self) -> usize;
        fn leading_ones(&self) -> usize;
        fn trailing_zeros(&self) -> usize;
        fn trailing_ones(&self) -> usize;
    }
    forward! {
        unsafe fn as_limbs_mut(&mut self) -> &mut [u64; LIMBS];
    }
    forward! {
        fn checked_shl(self, usize) -> Option<Self>;
        fn checked_shr(self, usize) -> Option<Self>;
    }
    forward! {
        fn overflowing_shl(self, usize) -> (Self, bool);
        fn overflowing_shr(self, usize) -> (Self, bool);
    }
    forward! {
        fn wrapping_shl(self, usize) -> Self;
        fn wrapping_shr(self, usize) -> Self;
        fn rotate_left(self, usize) -> Self;
        fn rotate_right(self, usize) -> Self;
    }
    forward! {
        fn from_be_bytes<const BYTES: usize>(bytes: [u8; BYTES]) -> Self;
    }
}

impl<const BITS: usize, const LIMBS: usize> Index<usize> for Bits<BITS, LIMBS> {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        if self.0.bit(index) {
            &true
        } else {
            &false
        }
    }
}

impl<const BITS: usize, const LIMBS: usize> Not for Bits<BITS, LIMBS> {
    type Output = Self;

    fn not(self) -> Self {
        self.0.not().into()
    }
}

impl<const BITS: usize, const LIMBS: usize> Not for &Bits<BITS, LIMBS> {
    type Output = Bits<BITS, LIMBS>;

    fn not(self) -> Bits<BITS, LIMBS> {
        self.0.not().into()
    }
}

macro_rules! impl_bit_op {
    ($trait:ident, $fn:ident, $trait_assign:ident, $fn_assign:ident) => {
        impl<const BITS: usize, const LIMBS: usize> $trait_assign<Bits<BITS, LIMBS>>
            for Bits<BITS, LIMBS>
        {
            #[allow(clippy::inline_always)]
            #[inline(always)]
            fn $fn_assign(&mut self, rhs: Bits<BITS, LIMBS>) {
                self.0.$fn_assign(&rhs.0);
            }
        }
        impl<const BITS: usize, const LIMBS: usize> $trait_assign<&Bits<BITS, LIMBS>>
            for Bits<BITS, LIMBS>
        {
            #[allow(clippy::inline_always)]
            #[inline(always)]
            fn $fn_assign(&mut self, rhs: &Bits<BITS, LIMBS>) {
                self.0.$fn_assign(rhs.0);
            }
        }
        impl<const BITS: usize, const LIMBS: usize> $trait<Bits<BITS, LIMBS>>
            for Bits<BITS, LIMBS>
        {
            type Output = Bits<BITS, LIMBS>;

            #[allow(clippy::inline_always)]
            #[inline(always)]
            fn $fn(mut self, rhs: Bits<BITS, LIMBS>) -> Self::Output {
                self.0.$fn_assign(rhs.0);
                self
            }
        }
        impl<const BITS: usize, const LIMBS: usize> $trait<&Bits<BITS, LIMBS>>
            for Bits<BITS, LIMBS>
        {
            type Output = Bits<BITS, LIMBS>;

            #[allow(clippy::inline_always)]
            #[inline(always)]
            fn $fn(mut self, rhs: &Bits<BITS, LIMBS>) -> Self::Output {
                self.0.$fn_assign(rhs.0);
                self
            }
        }
        impl<const BITS: usize, const LIMBS: usize> $trait<Bits<BITS, LIMBS>>
            for &Bits<BITS, LIMBS>
        {
            type Output = Bits<BITS, LIMBS>;

            #[allow(clippy::inline_always)]
            #[inline(always)]
            fn $fn(self, mut rhs: Bits<BITS, LIMBS>) -> Self::Output {
                rhs.0.$fn_assign(self.0);
                rhs
            }
        }
        impl<const BITS: usize, const LIMBS: usize> $trait<&Bits<BITS, LIMBS>>
            for &Bits<BITS, LIMBS>
        {
            type Output = Bits<BITS, LIMBS>;

            #[allow(clippy::inline_always)]
            #[inline(always)]
            fn $fn(self, rhs: &Bits<BITS, LIMBS>) -> Self::Output {
                self.0.clone().$fn(rhs.0).into()
            }
        }
    };
}

impl_bit_op!(BitOr, bitor, BitOrAssign, bitor_assign);
impl_bit_op!(BitAnd, bitand, BitAndAssign, bitand_assign);
impl_bit_op!(BitXor, bitxor, BitXorAssign, bitxor_assign);

macro_rules! impl_shift {
    ($trait:ident, $fn:ident, $trait_assign:ident, $fn_assign:ident) => {
        impl<const BITS: usize, const LIMBS: usize> $trait_assign<usize> for Bits<BITS, LIMBS> {
            #[allow(clippy::inline_always)]
            #[inline(always)]
            fn $fn_assign(&mut self, rhs: usize) {
                self.0.$fn_assign(rhs);
            }
        }

        impl<const BITS: usize, const LIMBS: usize> $trait_assign<&usize> for Bits<BITS, LIMBS> {
            #[allow(clippy::inline_always)]
            #[inline(always)]
            fn $fn_assign(&mut self, rhs: &usize) {
                self.0.$fn_assign(rhs);
            }
        }

        impl<const BITS: usize, const LIMBS: usize> $trait<usize> for Bits<BITS, LIMBS> {
            type Output = Self;

            #[allow(clippy::inline_always)]
            #[inline(always)]
            fn $fn(self, rhs: usize) -> Self {
                self.0.$fn(rhs).into()
            }
        }

        impl<const BITS: usize, const LIMBS: usize> $trait<usize> for &Bits<BITS, LIMBS> {
            type Output = Bits<BITS, LIMBS>;

            #[allow(clippy::inline_always)]
            #[inline(always)]
            fn $fn(self, rhs: usize) -> Self::Output {
                self.0.$fn(rhs).into()
            }
        }

        impl<const BITS: usize, const LIMBS: usize> $trait<&usize> for Bits<BITS, LIMBS> {
            type Output = Self;

            #[allow(clippy::inline_always)]
            #[inline(always)]
            fn $fn(self, rhs: &usize) -> Self {
                self.0.$fn(rhs).into()
            }
        }

        impl<const BITS: usize, const LIMBS: usize> $trait<&usize> for &Bits<BITS, LIMBS> {
            type Output = Bits<BITS, LIMBS>;

            #[allow(clippy::inline_always)]
            #[inline(always)]
            fn $fn(self, rhs: &usize) -> Self::Output {
                self.0.$fn(rhs).into()
            }
        }
    };
}

impl_shift!(Shl, shl, ShlAssign, shl_assign);
impl_shift!(Shr, shr, ShrAssign, shr_assign);
