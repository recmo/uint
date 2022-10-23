//! Two's complement signed operations.

/// A newtype wrapper around [`Uint`] that modifies operations to match
/// two's complement signed operations.
pub struct Int<const BITS: usize, const LIMBS: usize>(Uint<BITS, LIMBS>);

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
