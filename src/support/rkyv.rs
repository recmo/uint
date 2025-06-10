//! Support for the [rkyv](https://github.com/rkyv/rkyv) crate.
#![cfg(feature = "rkyv")]
#![cfg_attr(docsrs, doc(cfg(feature = "rkyv")))]

use core::fmt;
use std::hash::{Hash, Hasher};
use crate::{ArchivedUint, Uint};

impl<'a, const BITS: usize, const LIMBS: usize> From<&'a ArchivedUint<BITS, LIMBS>>
    for Uint<BITS, LIMBS>
{
    fn from(archived: &'a ArchivedUint<BITS, LIMBS>) -> Self {
        Self {
            limbs: archived.limbs.map(Into::into),
        }
    }
}

impl<const BITS: usize, const LIMBS: usize> From<ArchivedUint<BITS, LIMBS>> for Uint<BITS, LIMBS> {
    fn from(archived: ArchivedUint<BITS, LIMBS>) -> Self {
        (&archived).into()
    }
}

impl<const BITS: usize, const LIMBS: usize> Clone for ArchivedUint<BITS, LIMBS> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<const BITS: usize, const LIMBS: usize> Copy for ArchivedUint<BITS, LIMBS> {}

impl<const BITS: usize, const LIMBS: usize> PartialEq for ArchivedUint<BITS, LIMBS> {
    fn eq(&self, other: &Self) -> bool {
        self.limbs == other.limbs
    }
}

impl<const BITS: usize, const LIMBS: usize> Eq for ArchivedUint<BITS, LIMBS> {}

impl<const BITS: usize, const LIMBS: usize> Hash for ArchivedUint<BITS, LIMBS> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.limbs.hash(state);
    }
}

impl<const BITS: usize, const LIMBS: usize> fmt::Display for ArchivedUint<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&Into::<Uint<BITS, LIMBS>>::into(self), f)
    }
}

impl<const BITS: usize, const LIMBS: usize> fmt::Debug for ArchivedUint<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&Into::<Uint<BITS, LIMBS>>::into(self), f)
    }
}

impl<const BITS: usize, const LIMBS: usize> fmt::Binary for ArchivedUint<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&Into::<Uint<BITS, LIMBS>>::into(self), f)
    }
}

impl<const BITS: usize, const LIMBS: usize> fmt::Octal for ArchivedUint<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Octal::fmt(&Into::<Uint<BITS, LIMBS>>::into(self), f)
    }
}

impl<const BITS: usize, const LIMBS: usize> fmt::LowerHex for ArchivedUint<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&Into::<Uint<BITS, LIMBS>>::into(self), f)
    }
}

impl<const BITS: usize, const LIMBS: usize> fmt::UpperHex for ArchivedUint<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&Into::<Uint<BITS, LIMBS>>::into(self), f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{const_for, nlimbs, Uint};
    use proptest::proptest;
    use rkyv::rancor;

    #[test]
    fn test_rkyv() {
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            proptest!(|(n: Uint<BITS, LIMBS>)| {
                let s = rkyv::to_bytes::<rancor::Error>(&n).unwrap();
                let a = rkyv::access::<ArchivedUint<BITS, LIMBS>, rancor::Error>(&s).unwrap();
                assert_eq!(n, a.into());
                let d = rkyv::deserialize::<_, rancor::Error>(a).unwrap();
                assert_eq!(n, d);
            });
        });
    }
}
