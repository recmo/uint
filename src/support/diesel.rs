//! Support for the [`diesel`](https://crates.io/crates/diesel) crate.
//!
//! Currently only encodes to/from a big-endian byte array.

#![cfg(feature = "diesel")]
#![cfg_attr(docsrs, doc(cfg(feature = "diesel")))]

use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    query_builder::bind_collector::RawBytesBindCollector,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Binary,
};
use std::io::Write;
use thiserror::Error;

use crate::Uint;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Value too large for target type")]
    Overflow,
}

impl<const BITS: usize, const LIMBS: usize, DB: Backend> ToSql<Binary, DB> for Uint<BITS, LIMBS>
where
    for<'c> DB: Backend<BindCollector<'c> = RawBytesBindCollector<DB>>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        out.write_all(&self.to_be_bytes_vec())?;
        Ok(IsNull::No)
    }
}

impl<const BITS: usize, const LIMBS: usize, DB: Backend> FromSql<Binary, DB> for Uint<BITS, LIMBS>
where
    *const [u8]: FromSql<Binary, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let bytes: *const [u8] = FromSql::<Binary, DB>::from_sql(bytes)?;
        let bytes: &[u8] = unsafe { &*bytes };
        Self::try_from_be_slice(bytes).ok_or_else(|| DecodeError::Overflow.into())
    }
}
