mod arbitrary;
mod ark_ff;
mod fastrlp;
mod num_bigint;
mod postgres;
mod primitive_types;
mod proptest;
mod quickcheck;
mod rand;
mod rlp;
mod serde;
mod sqlx;
mod zeroize;

// FEATURE: Support for many more traits and crates.
// * https://docs.rs/tracing/latest/tracing/trait.Value.html
// * https://crates.io/crates/num-traits
// * https://crates.io/crates/der
// * https://crates.io/crates/bitvec
//
// Big int types:
// * https://crates.io/crates/crypto-bigint
// * https://crates.io/crates/rug
// * https://crates.io/crates/bigdecimal
// * https://crates.io/crates/rust_decimal
//
// More databases:
// * https://crates.io/crates/diesel
