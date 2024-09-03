//! Support for the [rkyv](https://github.com/rkyv/rkyv) crate.

#![cfg(feature = "rkyv")]
#![cfg_attr(docsrs, doc(cfg(feature = "rkyv")))]

use crate::Uint;
use rkyv::{
    bytecheck::CheckBytes,
    rancor::{Fallible, Trace},
    rend::u64_le,
    Archive, Archived, Deserialize, Place, Portable, Serialize,
};

/// An archived [`Uint`]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct ArchivedUint<const BITS: usize, const LIMBS: usize>([u64_le; LIMBS]);

unsafe impl<const BITS: usize, const LIMBS: usize, C: Fallible + ?Sized> CheckBytes<C>
    for ArchivedUint<BITS, LIMBS>
where
    <C as Fallible>::Error: Trace,
{
    unsafe fn check_bytes(
        value: *const Self,
        context: &mut C,
    ) -> Result<(), <C as Fallible>::Error> {
        unsafe {
            <[u64_le; LIMBS]>::check_bytes(value.cast(), context)?;
        }
        Ok(())
    }
}

impl<const BITS: usize, const LIMBS: usize> Archive for Uint<BITS, LIMBS> {
    type Archived = ArchivedUint<BITS, LIMBS>;
    type Resolver = [(); LIMBS];

    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        self.limbs
            .resolve(resolver, unsafe { out.cast_unchecked() });
    }
}

unsafe impl<const BITS: usize, const LIMBS: usize> Portable for ArchivedUint<BITS, LIMBS> {}

impl<S: Fallible + ?Sized, const BITS: usize, const LIMBS: usize> Serialize<S>
    for Uint<BITS, LIMBS>
{
    fn serialize(&self, serializer: &mut S) -> Result<[(); LIMBS], <S as Fallible>::Error> {
        self.limbs.serialize(serializer)
    }
}

impl<D: Fallible + ?Sized, const BITS: usize, const LIMBS: usize> Deserialize<Uint<BITS, LIMBS>, D>
    for Archived<Uint<BITS, LIMBS>>
{
    fn deserialize(
        &self,
        deserializer: &mut D,
    ) -> Result<Uint<BITS, LIMBS>, <D as Fallible>::Error> {
        Ok(Uint {
            limbs: <[u64_le; LIMBS]>::deserialize(&self.0, deserializer)?,
        })
    }
}

impl<'a, const BITS: usize, const LIMBS: usize> From<&'a ArchivedUint<BITS, LIMBS>>
    for Uint<BITS, LIMBS>
{
    fn from(archived: &'a ArchivedUint<BITS, LIMBS>) -> Self {
        Self {
            limbs: archived.0.map(u64_le::to_native),
        }
    }
}

impl<const BITS: usize, const LIMBS: usize> From<ArchivedUint<BITS, LIMBS>> for Uint<BITS, LIMBS> {
    fn from(archived: ArchivedUint<BITS, LIMBS>) -> Self {
        (&archived).into()
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
