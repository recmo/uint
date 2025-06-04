#![cfg(feature = "uniffi")]

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Deref,
};

use crate::aliases::U256;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A wrapper around a `U256` value for Foreign Bindings (through `uniffi-rs`).
///
/// The `F` suffix stands for "Foreign".
#[derive(
    uniffi::Object, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct U256F(pub U256);

#[derive(Debug, Error, uniffi::Error)]
pub enum UintError {
    #[error("Invalid number")]
    InvalidNumber,
}

#[uniffi::export]
impl U256F {
    /// Outputs a hex string representation of the `U256` value padded to 32 bytes (plus two bytes for the `0x` prefix).
    #[must_use]
    pub fn to_hex_string(self) -> String {
        format!("{:#066x}", self.0)
    }

    /// Attempts to parse a hex string as a `U256` value (wrapped).
    ///
    /// # Errors
    /// Will return an `Error::InvalidNumber` if the input is not a valid hex-string-presented number up to 256 bits.
    #[uniffi::constructor]
    pub fn try_from_hex_string(hex_string: &str) -> Result<Self, UintError> {
        let hex_string = hex_string.trim().trim_start_matches("0x");

        let number = U256::from_str_radix(hex_string, 16).map_err(|_| UintError::InvalidNumber)?;

        Ok(Self(number))
    }
}

impl From<U256F> for U256 {
    fn from(val: U256F) -> Self {
        val.0
    }
}

impl From<U256> for U256F {
    fn from(val: U256) -> Self {
        Self(val)
    }
}

impl Display for U256F {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.to_hex_string())
    }
}

impl Deref for U256F {
    type Target = U256;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
