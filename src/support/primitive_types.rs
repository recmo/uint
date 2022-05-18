//! Support for the [`primitive-types`](https://crates.io/crates/primitive-types) crate.
#![cfg(feature = "primitive-types")]

use crate::aliases as ours;
use primitive_types::{U128, U256, U512};

// TODO: H160, H256..

macro_rules! impl_froms {
    ($ours:ty, $theirs:ident) => {
        impl From<$theirs> for $ours {
            #[inline]
            fn from(value: $theirs) -> Self {
                Self::from_limbs(value.0)
            }
        }

        impl From<$ours> for $theirs {
            fn from(value: $ours) -> Self {
                $theirs(value.into_limbs())
            }
        }
    };
}

impl_froms!(ours::U128, U128);
impl_froms!(ours::U256, U256);
impl_froms!(ours::U512, U512);

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{arbitrary::Arbitrary, proptest};

    fn test_roundtrip<Ours, Theirs>()
    where
        Ours: Clone + PartialEq + Arbitrary + From<Theirs>,
        Theirs: From<Ours>,
    {
        proptest!(|(value: Ours)| {
            let theirs: Theirs = value.clone().into();
            let ours: Ours = theirs.into();
            assert_eq!(ours, value);
        });
    }

    #[test]
    fn test_roundtrips() {
        test_roundtrip::<ours::U128, U128>();
        test_roundtrip::<ours::U256, U256>();
        test_roundtrip::<ours::U512, U512>();
    }
}
