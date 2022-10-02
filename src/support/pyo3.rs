//! Support for the [`pyo3`](https://crates.io/crates/pyo3) crate.
//!
//! Conversion is to/from Python native integers. Beware that Python native
//! integers are.
//!
//! See the Pyo3 num-bigint [implementation](https://github.com/PyO3/pyo3/blob/4a68273b173ef86dac059106cc0b5b3c2c9830e2/src/conversions/num_bigint.rs#L80) for reference.
//! [uint128](https://github.com/PyO3/pyo3/blob/4a68273b173ef86dac059106cc0b5b3c2c9830e2/src/types/num.rs#L176)
#![cfg(feature = "pyo3")]
#![cfg_attr(has_doc_cfg, doc(cfg(feature = "pyo3")))]

use crate::Uint;
use pyo3::{ffi, FromPyObject, IntoPy, PyObject, Python, ToPyObject};

impl<const BITS: usize, const LIMBS: usize> ToPyObject for Uint<BITS, LIMBS> {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        // Fast path for small ints
        if BITS <= 64 {
            if let Some(limb) = self.as_limbs().first() {
                return limb.into_py(py);
            } else {
                return 0.into_py(py);
            }
        }

        // Convert using little endian bytes (trivial on LE archs).
        let bytes = self.as_le_bytes();
        unsafe {
            let obj = ffi::_PyLong_FromByteArray(
                bytes.as_ptr() as *const std::os::raw::c_uchar,
                bytes.len(),
                1,
                0,
            );
            PyObject::from_owned_ptr(py, obj)
        }
    }
}

impl<const BITS: usize, const LIMBS: usize> IntoPy<PyObject> for Uint<BITS, LIMBS> {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}
