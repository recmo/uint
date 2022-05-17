//! Support for the [`postgres`](https://crates.io/crates/postgres) crate.
#![cfg(feature = "postgres")]

use crate::Uint;
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, IsNull, ToSql, Type, WrongType};
use std::error::Error;
use thiserror::Error;

type BoxedError = Box<dyn Error + Sync + Send + 'static>;

#[derive(Clone, PartialEq, Eq, Debug, Error)]
pub enum ToSqlError {
    #[error("Uint<{0}> value too large to fit target type {1}")]
    Overflow(usize, Type),
}

/// Convert to PostgreSQL types.
///
/// Compatible [PostgreSQL data types][dt] are:
///
/// * `BOOL`, `CHAR`, `SMALLINT`, `INTEGER`, `BIGINT` which are 1, 8, 16, 32 and
///   64 bit signed integers respectively.
/// * `SMALLSERIAL`, `SERIAL`, `BIGSERIAL` which are 16, 32 and 64 bit signed
///   integers respectively.
/// * `OID` which is a 32 bit unsigned integer.
/// * `DECIMAL` and `NUMERIC`, which are variable length.
/// * `MONEY` which is a 64 bit integer.
/// * `BYTEA`, `BIT`, `VARBIT` interpreted as a big-endian binary number.
/// * `CHAR`, `VARCHAR` as `0x`-prefixed big-endian hex strings.
/// * `JSON`, `JSONB` as a Serde compatible JSON value (requires `serde`
///   feature).
///
/// Note: [`Uint`]s are never null, use [`Option<Uint>`] instead.
///
/// # Errors
///
/// Returns an error when trying to convert to a value that is too small to fit
/// the number. Note that this depends on the value, not the type, so a
/// [`Uint<256>`] can be stored in a `SMALLINT` column, as long as the values
/// are less than $2^{16}$.
///
/// # Implementation details
///
/// The Postgres binary formats are used in the wire-protocol and the
/// the `COPY BINARY` command, but they have very little documentation. You are
/// pointed to the source code, for example this is the implementation of the
/// the `NUMERIC` type serializer: [`numeric.c`][numeric].
///
/// [dt]:https://www.postgresql.org/docs/9.5/datatype.html
/// [numeric]: https://github.com/postgres/postgres/blob/05a5a1775c89f6beb326725282e7eea1373cbec8/src/backend/utils/adt/numeric.c#L1082
impl<const BITS: usize, const LIMBS: usize> ToSql for Uint<BITS, LIMBS> {
    fn accepts(ty: &Type) -> bool {
        matches!(*ty, |Type::BOOL| Type::CHAR
            | Type::INT2
            | Type::INT4
            | Type::INT8
            | Type::OID
            | Type::NUMERIC
            | Type::MONEY
            | Type::BYTEA
            | Type::TEXT
            | Type::VARCHAR
            | Type::JSON
            | Type::JSONB
            | Type::BIT
            | Type::VARBIT)
    }

    // See <https://github.com/sfackler/rust-postgres/blob/38da7fa8fe0067f47b60c147ccdaa214ab5f5211/postgres-protocol/src/types/mod.rs>
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, BoxedError> {
        match *ty {
            // Big-endian simple types
            // Note `BufMut::put_*` methods write big-endian by default.
            Type::BOOL => out.put_u8(bool::try_from(*self)? as u8),
            Type::CHAR => out.put_i8(self.try_into()?),
            Type::INT2 => out.put_i16(self.try_into()?),
            Type::INT4 => out.put_i32(self.try_into()?),
            Type::INT8 => out.put_i64(self.try_into()?),
            Type::OID => out.put_u64(self.try_into()?),
            Type::MONEY => {
                // Like i64, but with two decimals.
                out.put_i64(
                    i64::try_from(self)?
                        .checked_mul(100)
                        .ok_or_else(|| ToSqlError::Overflow(BITS, ty.clone()))?,
                );
            }
            // TODO: Potentially lossy f32 and f64?

            // Binary strings
            Type::BIT => todo!(),
            Type::VARBIT => todo!(),
            Type::BYTEA => todo!(),

            // Hex strings
            Type::TEXT => todo!(),
            Type::VARCHAR => todo!(),
            Type::JSON => todo!(),
            Type::JSONB => todo!(),

            // Binary coded decimal types
            Type::NUMERIC => todo!(),

            // Unsupported types
            _ => {
                return Err(Box::new(WrongType::new::<Self>(ty.clone())));
            }
        };
        Ok(IsNull::No)
    }

    to_sql_checked!();
}

#[cfg(test)]
mod tests {}
