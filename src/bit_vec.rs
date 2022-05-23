use crate::Uint;
use std::borrow::Cow;

/// Bit vector
///
/// This is a newtype wrapper around [`Uint<BITS, LIMBS>`] that restricts
/// operations to those relevant for bit vectors.
pub struct Bits<const BITS: usize, const LIMBS: usize>(Uint<BITS, LIMBS>);

impl<const BITS: usize, const LIMBS: usize> Bits<BITS, LIMBS> {
    /// The size of this integer type in 64-bit limbs.
    pub const LIMBS: usize = Uint::<BITS, LIMBS>::LIMBS;

    /// Bit mask for the last limb.
    const MASK: u64 = Uint::<BITS, LIMBS>::MASK;

    /// The size of this integer type in bits.
    pub const BITS: usize = Uint::<BITS, LIMBS>::BITS;

    /// The size of this integer type in bits.
    pub const BYTES: usize = Uint::<BITS, LIMBS>::BYTES;

    /// The value zero. This is the only value that exists in all [`Uint`]
    /// types.
    pub const ZERO: Self = Self(Uint::<BITS, LIMBS>::ZERO);
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
            #[must_use]
            pub fn $fnname(self) -> $res {
                Uint::$fnname(self.0).into()
            }
        )*
    };
    ($(fn $fnname:ident(&self) -> $res:ty;)*) => {
        $(
            #[doc = concat!("See [`Uint::", stringify!($fnname),"`] for documentation.")]
            #[must_use]
            pub fn $fnname(&self) -> $res {
                Uint::$fnname(&self.0).into()
            }
        )*
    };
    ($(fn $fnname:ident(&mut self) -> $res:ty;)*) => {
        $(
            #[doc = concat!("See [`Uint::", stringify!($fnname),"`] for documentation.")]
            #[must_use]
            pub fn $fnname(&mut self) -> $res {
                Uint::$fnname(&mut self.0).into()
            }
        )*
    };
    ($(fn $fnname:ident(self, $arg:ty) -> Option<Self>;)*) => {
        $(
            #[doc = concat!("See [`Uint::", stringify!($fnname),"`] for documentation.")]
            #[must_use]
            pub fn $fnname(self, a: $arg) -> Option<Self> {
                Uint::$fnname(self.0, a).map(Bits::from)
            }
        )*
    };
    ($(fn $fnname:ident(self, $arg:ty) -> (Self, bool);)*) => {
        $(
            #[doc = concat!("See [`Uint::", stringify!($fnname),"`] for documentation.")]
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
            #[must_use]
            pub fn $fnname(self, a: $arg) -> $res {
                Uint::$fnname(self.0, a).into()
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
        fn as_le_bytes_trimmed(&self) -> Cow<'_, [u8]>;
        fn as_limbs(&self) -> &[u64; LIMBS];
        fn bit_len(&self) -> usize;
        fn byte_len(&self) -> usize;
        fn checked_log2(&self) -> Option<usize>;
        fn leading_zeros(&self) -> usize;
        fn leading_ones(&self) -> usize;
        fn trailing_zeros(&self) -> usize;
        fn trailing_ones(&self) -> usize;
    }
    forward! {
        fn as_limbs_mut(&mut self) -> &mut [u64; LIMBS];
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
}
