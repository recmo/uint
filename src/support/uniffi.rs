#![cfg(feature = "uniffi")]

use crate::aliases::U256;
use thiserror::Error;

#[derive(Debug, Error, uniffi::Error)]
pub enum UintError {
    #[error("Invalid number")]
    InvalidNumber,
}

#[derive(uniffi::Record)]
pub struct U256F {
    limb0: u64,
    limb1: u64,
    limb2: u64,
    limb3: u64,
}

impl From<U256> for U256F {
    fn from(value: U256) -> Self {
        let limbs = value.into_limbs();
        Self {
            limb0: limbs[0],
            limb1: limbs[1],
            limb2: limbs[2],
            limb3: limbs[3],
        }
    }
}

impl From<U256F> for U256 {
    fn from(value: U256F) -> Self {
        Self::from_limbs([value.limb0, value.limb1, value.limb2, value.limb3])
    }
}

uniffi::custom_type!(U256, U256F);

/// Given a `U256`, return its hex string ("0x"-prefixed and 32 byte–padded).
#[uniffi::export]
#[must_use]
pub fn u256_to_hex_string(num: U256) -> String {
    format!("{num:#066x}")
}

/// For Foreign Code use. Attempts to initialize a `U256` from a hex-encoded string.
///
/// May be 0x-prefixed.
///
/// # Errors
/// - Returns `UintError::InvalidNumber` if the provided string is not a validly hex-encoded U256.
#[uniffi::export]
#[uniffi::constructor]
pub fn u256_from_hex_string(s: &str) -> Result<U256, UintError> {
    let hex_string = s.trim().trim_start_matches("0x");
    U256::from_str_radix(hex_string, 16).map_err(|_| UintError::InvalidNumber)
}
