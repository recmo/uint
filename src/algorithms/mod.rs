//! Collection of bignum algorithms.
//!
//! **Warning.** Most functions in this module are currently not considered part
//! of the stable API and may be changed or removed in future minor releases.

mod div;
mod mul;

pub use self::{
    div::div_rem,
    mul::{mul, mul_inline},
};
