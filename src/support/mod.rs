mod arbitrary;
mod fastrlp;
mod postgres;
mod primitive_types;
mod proptest;
mod quickcheck;
mod rand;
mod rlp;
mod serde;

// FEATURE: Support for many more traits and crates.
// * https://docs.rs/tracing/latest/tracing/trait.Value.html
// * https://crates.io/crates/num-traits
// * https://crates.io/crates/der
// * https://crates.io/crates/zeroize
// * https://crates.io/crates/ff
// * https://crates.io/crates/ff_ce
// * https://crates.io/crates/ark-ff
// * https://crates.io/crates/bitvec
//
// Big int types:
// * https://crates.io/crates/num-bigint
// * https://crates.io/crates/crypto-bigint
// * https://crates.io/crates/rug
// * https://crates.io/crates/bigdecimal
// * https://crates.io/crates/rust_decimal
//
// More databases:
// * https://crates.io/crates/diesel
// * https://crates.io/crates/sqlx
