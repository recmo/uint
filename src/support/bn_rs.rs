//! Support for the [`bn-rs`](https://crates.io/crates/bn-rs) crate.
#![cfg(feature = "bn-rs")]
#![cfg_attr(has_doc_cfg, doc(cfg(feature = "bn-rs")))]

use crate::{from::ToUintError, BaseConvertError, ParseError, Uint};
use bn_rs::{BigNumber, BN};

impl<const BITS: usize, const LIMBS: usize> TryFrom<&BN> for Uint<BITS, LIMBS> {
    type Error = ToUintError<Self>;

    // FIXME: Return wrapped values.
    fn try_from(value: &BN) -> Result<Self, Self::Error> {
        if value.negative() == 1 {
            return Err(ToUintError::ValueNegative(BITS, Self::ZERO));
        }
        if value.byte_length() as usize > Self::BYTES {
            return Err(ToUintError::ValueTooLarge(BITS, Self::ZERO));
        }
        // Binding for `toArray`
        // `a.toArray(endian, length)` - convert to byte array, and optionally zero pad
        // to length, throwing if already exceeding. <https://github.com/indutny/bn.js/blob/5df40f81ea8afb835b909bb7c21e0833cdeb6a30/lib/bn.js#L573>
        value.to_array("le".into(), 0).map_or_else(
            |_| Err(ToUintError::NotANumber(BITS)),
            |bytes| {
                Self::try_from_le_slice(&bytes).ok_or(ToUintError::ValueTooLarge(BITS, Self::ZERO))
            },
        )
    }
}

impl<const BITS: usize, const LIMBS: usize> TryFrom<BN> for Uint<BITS, LIMBS> {
    type Error = ToUintError<Self>;

    fn try_from(value: BN) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl<const BITS: usize, const LIMBS: usize> From<&Uint<BITS, LIMBS>> for BN {
    fn from(value: &Uint<BITS, LIMBS>) -> Self {
        Self::new_from_array(&value.as_le_bytes(), 256)
    }
}

impl<const BITS: usize, const LIMBS: usize> From<Uint<BITS, LIMBS>> for BN {
    fn from(value: Uint<BITS, LIMBS>) -> Self {
        (&value).into()
    }
}

impl<const BITS: usize, const LIMBS: usize> TryFrom<&BigNumber> for Uint<BITS, LIMBS> {
    type Error = ToUintError<Self>;

    // FIXME: Return wrapped values.
    fn try_from(value: &BigNumber) -> Result<Self, Self::Error> {
        let hex = value.hex();
        Self::from_str_radix(&hex, 16).map_err(|e| match e {
            ParseError::BaseConvertError(BaseConvertError::Overflow) => {
                ToUintError::ValueTooLarge(BITS, Self::ZERO)
            }
            _ => ToUintError::NotANumber(BITS),
        })
    }
}

impl<const BITS: usize, const LIMBS: usize> TryFrom<BigNumber> for Uint<BITS, LIMBS> {
    type Error = ToUintError<Self>;

    fn try_from(value: BigNumber) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl<const BITS: usize, const LIMBS: usize> From<&Uint<BITS, LIMBS>> for BigNumber {
    fn from(value: &Uint<BITS, LIMBS>) -> Self {
        Self::new(format!("{value:#x}"))
    }
}

impl<const BITS: usize, const LIMBS: usize> From<Uint<BITS, LIMBS>> for BigNumber {
    fn from(value: Uint<BITS, LIMBS>) -> Self {
        (&value).into()
    }
}

#[cfg(test)]
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))] // Tests require wasm
mod test {
    use super::*;
    use crate::{const_for, nlimbs};
    use proptest::proptest;

    #[test]
    fn test_bn_roundtrip() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(value: U)| {
                let obj: BN = value.into();
                let native = obj.try_into().unwrap();
                assert_eq!(value, native);
            });
        });
    }

    #[test]
    fn test_bignumber_roundtrip() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(value: U)| {
                let obj: BigNumber = value.into();
                let native = obj.try_into().unwrap();
                assert_eq!(value, native);
            });
        });
    }
}
