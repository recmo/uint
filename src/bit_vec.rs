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

// Limitations of declarative macro matching force us to break down on argument
// patterns.
macro_rules! forward {
    ($(fn $fnname:ident(&self) -> $res:ty;)*) => {
        $(
            #[doc = concat!("See [`Uint::", stringify!($fnname),"`] for documentation.")]
            #[must_use]
            pub fn $fnname(&self) -> $res {
                Uint::$fnname(&self.0)
            }
        )*
    };
    ($(fn $fnname:ident(&mut self) -> $res:ty;)*) => {
        $(
            #[doc = concat!("See [`Uint::", stringify!($fnname),"`] for documentation.")]
            pub fn $fnname(&mut self) -> $res {
                Uint::$fnname(&mut self.0)
            }
        )*
    };
}

impl<const BITS: usize, const LIMBS: usize> Bits<BITS, LIMBS> {
    forward! {
        fn as_le_bytes(&self) -> Cow<'_, [u8]>;
        fn as_le_bytes_trimmed(&self) -> Cow<'_, [u8]>;
        fn as_limbs(&self) -> &[u64; LIMBS];
        fn bit_len(&self) -> usize;
        fn byte_len(&self) -> usize;
        fn leading_zeros(&self) -> usize;
    }
    forward! {
        fn as_limbs_mut(&mut self) -> &mut [u64; LIMBS];
    }
}
