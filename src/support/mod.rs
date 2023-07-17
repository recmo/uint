//! Support for external crates.

pub mod alloy_rlp;
pub mod arbitrary;
pub mod ark_ff;
pub mod bn_rs;
pub mod fastrlp;
pub mod num_bigint;
pub mod postgres;
pub mod primitive_types;
pub mod proptest;
pub mod pyo3;
pub mod quickcheck;
pub mod rand;
pub mod rlp;
pub mod scale;
pub mod serde;
pub mod sqlx;
pub mod valuable;
pub mod zeroize;

// FEATURE: Support for many more traits and crates.
// * https://crates.io/crates/num-traits
// * https://crates.io/crates/der
// * https://crates.io/crates/bitvec

// * open-fastrlp

// * https://docs.rs/bytemuck/1.9.1/bytemuck/trait.Zeroable.html
// * https://docs.rs/bytemuck/1.9.1/bytemuck/trait.Pod.html

// Big int types:
// * https://crates.io/crates/crypto-bigint
// * https://crates.io/crates/rug
// * https://crates.io/crates/bigdecimal
// * https://crates.io/crates/rust_decimal

// * wasm-bindgen `JsValue` bigint: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html#method.bigint_from_str
//   or from_f64.
// * Neon `JsBigInt` once it lands: https://github.com/neon-bindings/neon/pull/861

// More databases:
// * https://crates.io/crates/diesel

// FEATURE: Make sure `Bits` has the same level of support.

// TODO: Add more support for `Bits`, for example `rand` and `quickcheck`.
