//! Support for the [`pyo3`](https://crates.io/crates/pyo3) crate.
//!
//! Conversion is to/from Python native integers. Beware that Python native
//! integers are unbounded and not a ring modulo a power of two like [`Uint`].
//!
//! This uses the not-so-public `_PyLong_FromByteArray`, `_PyLong_AsByteArray`
//! ABI, which according to this [Stackoverflow answer][so] is the accepted way
//! to do efficient bigint conversions. It is supported by [CPython][cpython]
//! and [PyPy][pypy].
//!
//! [so]: https://stackoverflow.com/a/18326068
//! [cpython]: https://github.com/python/cpython/blob/e8165d47b852e933c176209ddc0b5836a9b0d5f4/Include/cpython/longobject.h#L47
//! [pypy]: https://foss.heptapod.net/pypy/pypy/-/blob/branch/default/pypy/module/cpyext/longobject.py#L238
//!
//! The implementation uses Pyo3 builtin `u64` conversion when $\mathtt{BITS} ≤
//! 64$ and otherwise uses similar conversion to Pyo3 builtin `num-bigint`
//! support. See Pyo3's [`num.rs`][num] and [`num_bigint.rs`][bigint] for
//! reference.
//!
//! [num]: https://github.com/PyO3/pyo3/blob/caaf7bbda74f873297d277733c157338f5492580/src/types/num.rs#L81
//! [bigint]: https://github.com/PyO3/pyo3/blob/4a68273b173ef86dac059106cc0b5b3c2c9830e2/src/conversions/num_bigint.rs#L80

#![cfg(feature = "pyo3")]
#![cfg_attr(docsrs, doc(cfg(feature = "pyo3")))]

use crate::{nbytes, Uint};
use pyo3::{
    exceptions::{PyAssertionError, PyOverflowError},
    types::PyAnyMethods,
    Bound, FromPyObject, IntoPyObject, IntoPyObjectExt, PyAny, PyErr, PyResult, Python,
};

impl<'py, const BITS: usize, const LIMBS: usize> IntoPyObject<'py> for Uint<BITS, LIMBS> {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        // // Fast path for small ints
        if BITS <= 64 {
            let value = self.as_limbs().first().copied().unwrap_or(0);
            return value.into_bound_py_any(py);
        }

        // Convert using little endian bytes (trivial on LE archs)
        let bytes = self.as_le_bytes();

        bytes.into_pyobject(py)
    }
}

impl<'py, const BITS: usize, const LIMBS: usize> FromPyObject<'py> for Uint<BITS, LIMBS> {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        // Fast path for small ints
        if BITS <= 64 {
            let limb: u64 = obj.extract()?;
            let (res, overflow) = Self::overflowing_from_limbs_slice(&[limb]);
            if overflow {
                return Err(PyOverflowError::new_err(format!(
                    "Number too large to fit Uint<{}>",
                    Self::BITS
                )));
            }
            return Ok(res);
        }

        let expected_bytes = nbytes(BITS);
        let slice: &[u8] = obj.extract()?;
        if slice.len() != expected_bytes {
            return Err(PyAssertionError::new_err(format!(
                "Uint: Number of bytes does not match, expected: {}, got: {}",
                expected_bytes,
                slice.len()
            )));
        }
        return Ok(Self::from_le_slice(slice));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        aliases::{U0, U256, U512, U64, U8},
        const_for, nlimbs,
    };
    use proptest::proptest;

    #[test]
    fn test_roundtrip() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            const_for!(BITS in SIZES {
                const LIMBS: usize = nlimbs(BITS);
                type U = Uint<BITS, LIMBS>;
                proptest!(|(value: U)| {
                    let obj = value.into_pyobject(py).unwrap();
                    let native = obj.extract().unwrap();
                    assert_eq!(value, native);
                });
            });
        });
    }

    #[test]
    fn test_errors() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let obj = (-1_i64).into_pyobject(py).unwrap();
            assert!(obj.extract::<U0>().is_err());
            assert!(obj.extract::<U256>().is_err());

            let obj = (1000_i64).into_pyobject(py).unwrap();
            assert!(obj.extract::<U0>().is_err());
            assert!(obj.extract::<U8>().is_err());

            let obj = U512::MAX.into_pyobject(py).unwrap();
            assert!(obj.extract::<U0>().is_err());
            assert!(obj.extract::<U64>().is_err());
            assert!(obj.extract::<U256>().is_err());
        });
    }
}
