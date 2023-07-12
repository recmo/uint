# Rust `uint` crate using const-generics

[![crates.io](https://buildstats.info/crate/ruint)](https://crates.io/crates/ruint2)
[![docs.rs](https://img.shields.io/docsrs/ruint)](https://docs.rs/ruint2)
[![MIT License](https://img.shields.io/github/license/alloy-rs/uint)](https://github.com/alloy-rs/uint/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/alloy-rs/uint/status.svg)](https://deps.rs/repo/github/alloy-rs/uint)
[![codecov](https://codecov.io/gh/alloy-rs/uint/branch/main/graph/badge.svg?token=WBPZ9U4TTO)](https://codecov.io/gh/alloy-rs/uint)
[![CI](https://github.com/alloy-rs/uint/actions/workflows/ci.yml/badge.svg)](https://github.com/alloy-rs/uint/actions/workflows/ci.yml)

Implements [`Uint<BITS, LIMBS>`], the ring of numbers modulo $2^{\mathsf{BITS}}$. It requires two
generic arguments: the number of bits and the number of 64-bit 'limbs' required to store those bits.

```rust
# use ruint2::Uint;
let answer: Uint<256, 4> = Uint::from(42);
```

You can compute `LIMBS` yourself using $\mathsf{LIMBS} = \left\lceil{\mathsf{BITS} / 64}\right\rceil$,
i.e.`LIMBS` equals `BITS` divided by $64$ rounded up. [`Uint`] will `panic!` if you try to
construct it with incorrect arguments. Ideally this would be a compile time error, but
that is blocked by Rust issue [#60551][r60551].

[r60551]: https://github.com/rust-lang/rust/issues/60551

A more convenient method on stable is to use the [`uint!`] macro, which constructs the right
[`Uint`] for you.

```rust
# use ruint2::{Uint, uint};
let answer = uint!(42_U256);
```

You can also use one of the pre-computed type [`aliases`]:

```rust
# use ruint2::Uint;
use ruint2::aliases::*;

let answer: U256 = Uint::from(42);
```

You can of course also create your own type alias if you need a funny size:

```rust
# use ruint2::Uint;
type U1337 = Uint<1337, 21>;

let answer: U1337 = Uint::from(42);
```

## Rust nightly

If you are on nightly, you can use [`Uint<BITS>`][nightly::Uint] which will
compute the number of limbs for you. Unfortunately this can not be made stable
without `generic_const_exprs` support (Rust issue [#76560][r76560]).

[r76560]: https://github.com/rust-lang/rust/issues/76560

```rust
# #[cfg(all(has_generic_const_exprs, feature = "generic_const_exprs"))] {
use ruint2::nightly::Uint;

let answer: Uint<256> = Uint::<256>::from(42);
# }
```

Even on nightly, the ergonomics of Rust are limited. In the example above Rust
requires explicit type annotation for [`Uint::from`], where it did not require
it in the stable version. There are a few more subtle issues that make this
less ideal than it appears. It also looks like it may take some time before
these nightly features are stabilized.

## Examples

```rust
use ruint2::Uint;

let a: Uint<256, 4> = Uint::from(0xf00f_u64);
let b: Uint<256, 4> = Uint::from(42_u64);
let c  = a + b;
assert_eq!(c, Uint::from(0xf039_u64));
```

There is a convenient macro [`uint!`] to create constants for you. It allows
for arbitrary length constants using standard Rust integer syntax. The size of
the [`Uint`] or [`Bits`] is specified with a `U` or `B` suffix followed by the
number of bits. The standard Rust syntax of decimal, hexadecimal and even binary and octal is
supported using their prefixes `0x`, `0b` and `0o`. Literals can have
underscores `_` added for readability.

```rust
# use ruint2::uint;
let cow = uint!(0xc85ef7d79691fe79573b1a7064c19c1a9819ebdbd1faaab1a8ec92344438aaf4_U256);
```

In fact, this macro recurses down the parse tree, so you can apply it to entire
source files:

```rust
# use ruint2::uint;
uint!{

let a = 42_U256;
let b = 0xf00f_1337_c0d3_U256;
let c = a + b;
assert_eq!(c, 263947537596669_U256);

}
```

Note that since `B` is a valid hexadecimal digit there can be ambiguity. To lessen the impact an underscore separator `_B` is required in this case.

## Supported Rust Versions

<!--
When updating this, also update:
- .clippy.toml
- Cargo.toml
- .github/workflows/ci.yml
-->

Uint will keep a rolling MSRV (minimum supported rust version) policy of **at
least** 6 months. When increasing the MSRV, the new Rust version must have been
released at least six months ago. The current MSRV is 1.65.0.

Note that the MSRV is not increased automatically, and only as part of a minor
release.

## Feature flags

There is support for a number of crates. These are enabled by setting the identically
named feature flag.

* `unstable` Enable sem-ver unstable features.
* [`rand`](https://docs.rs/rand): Implements sampling from the [`Standard`](https://docs.rs/rand/latest/rand/distributions/struct.Standard.html) distribution, i.e. [`rng.gen()`](https://docs.rs/rand/latest/rand/trait.Rng.html#method.gen).
* [`arbitrary`](https://docs.rs/arbitrary): Implements the [`Arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html) trait, allowing [`Uint`]s to be generated for fuzz testing.
* [`quickcheck`](https://docs.rs/quickcheck): Implements the [`Arbitrary`](https://docs.rs/quickcheck/latest/quickcheck/trait.Arbitrary.html) trait, allowing [`Uint`]s to be generated for property based testing.
* [`proptest`](https://docs.rs/proptest): Implements the [`Arbitrary`](https://docs.rs/proptest/latest/proptest/arbitrary/trait.Arbitrary.html) trait, allowing [`Uint`]s to be generated for property based testing. Proptest is used for the `uint`s own test suite.
* [`serde`](https://docs.rs/serde): Implements the [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) traits for [`Uint`] and [`Bits`].Serialization uses big-endian hex in human readable formats and big-endian byte strings in machine readable formats. [`Uint`] uses ethereum `Quantity` format (0x-prefixed minimal string) when serializing in a human readable format.
* [`rlp`](https://docs.rs/rlp): Implements the [`Encodable`](https://docs.rs/rlp/latest/rlp/trait.Encodable.html) and [`Decodable`](https://docs.rs/rlp/latest/rlp/trait.Decodable.html) traits for [`Uint`] to allow serialization to/from RLP.
* [`fastrlp`](https://docs.rs/fastrlp): Implements the [`Encodable`](https://docs.rs/fastrlp/latest/fastrlp/trait.Encodable.html) and [`Decodable`](https://docs.rs/fastrlp/latest/fastrlp/trait.Decodable.html) traits for [`Uint`] to allow serialization to/from RLP.
* [`primitive-types`](https://docs.rs/primitive-types): Implements the [`From<_>`] conversions between corresponding types.
* [`postgres`](https://docs.rs/postgres): Implements the [`ToSql`](https://docs.rs/postgres/latest/postgres/types/trait.ToSql.html) trait supporting many column types.
* [`num-bigint`](https://docs.rs/num-bigint): Implements conversion to/from [`BigUint`](https://docs.rs/num-bigint/latest/num_bigint/struct.BigUint.html) and [`BigInt`](https://docs.rs/num-bigint/latest/num_bigint/struct.BigInt.html).
* [`ark-ff`](https://docs.rs/ark-ff): Implements conversion to/from [`BigInt`](https://docs.rs/ark-ff/0.4.2/ark_ff/biginteger/struct.BigInt.html) and [`Fp`](https://docs.rs/ark-ff/0.4.2/ark_ff/fields/models/fp/struct.Fp.html) types.
* [`sqlx`](https://docs.rs/sqlx): Implements database agnostic storage as byte array. Requires
  `sqlx` to be used with the `tokio-native-tls` runtime, due to issue [sqlx#1627](https://github.com/launchbadge/sqlx/issues/1627).
* [`zeroize`](https://docs.rs/zeroize): Implements the [`Zeroize`](https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html) trait. This makes [`Uint`] and [`Bits`] compatible with the [`secrecy`](https://crates.io/crates/secrecy) crate.
* [`valuable`](https://docs.rs/valuable): Implements the [`Valuable`](https://docs.rs/valuable/0.1.0/valuable/trait.Valuable.html) trait.
* [`pyo3`](https://docs.rs/pyo3): Implements the [`ToPyObject`](https://docs.rs/pyo3/latest/pyo3/conversion/trait.ToPyObject.html), [`IntoPy`](https://docs.rs/pyo3/latest/pyo3/conversion/trait.IntoPy.html) and [`FromPyObject`](https://docs.rs/pyo3/latest/pyo3/conversion/trait.FromPyObject.html) traits.
* [`parity-scale-codec`](https://docs.rs/parity-scale-codec): Implements the [`Encode`](https://docs.rs/parity-scale-codec/latest/parity_scale_codec/trait.Encode.html), [`Decode`](https://docs.rs/parity-scale-codec/latest/parity_scale_codec/trait.Decode.html), [`MaxEncodedLen`](https://github.com/paritytech/parity-scale-codec/blob/47d98a1c23dabc890fdb548d115a18070082c66e/src/max_encoded_len.rs) and [`HasCompact`](https://docs.rs/parity-scale-codec/latest/parity_scale_codec/trait.HasCompact.html) traits.
* [`bn-rs`](https://docs.rs/bn-rs/latest/bn_rs/): Implements conversion to/from the [`BN`](https://docs.rs/bn-rs/latest/bn_rs/struct.BN.html) and [`BigNumber`](https://docs.rs/bn-rs/latest/bn_rs/struct.BigNumber.html).

## Building and testing

Format, lint, build and test everything (I recommend creating a shell alias for this):

```sh
cargo fmt &&\
cargo clippy --all-features --all-targets &&\
cargo test --workspace --all-features --doc -- --nocapture &&\
cargo test --workspace --all-features --all-targets -- --nocapture &&\
cargo doc --workspace --all-features --no-deps
```

Run benchmarks with the provided `.cargo/config.toml` alias

```sh
cargo criterion
```

Check documentation coverage

```sh
RUSTDOCFLAGS="-Z unstable-options --show-coverage"  cargo doc --workspace --all-features --no-deps
```

## Features

* All the quality of life features one could want.
* Compatible with std `u64`, etc types. See Rust's [integer methods](https://doc.rust-lang.org/stable/std/primitive.u64.html).
* Adhere to [Rust API Guidelines](https://rust-lang.github.io/api-guidelines)
* Montgomery REDC and other algo's for implementing prime fields.

## To do

* Builds `no-std` and `wasm`.
* Fast platform agnostic generic algorithms.
* Target specific assembly optimizations (where available).
* Optional num-traits, etc, support.
* Run-time sized type with compatible interface.

---

[![lines of code](https://img.shields.io/tokei/lines/github/alloy-rs/uint)](https://github.com/alloy-rs/uint)
[![GitHub contributors](https://img.shields.io/github/contributors/alloy-rs/uint)](https://github.com/alloy-rs/uint/graphs/contributors)
[![GitHub issues](https://img.shields.io/github/issues/alloy-rs/uint)](https://github.com/alloy-rs/uint/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/alloy-rs/uint?label=PRs)](https://github.com/alloy-rs/uint/pulls)
[![GitHub Repo stars](https://img.shields.io/github/stars/alloy-rs/uint)](https://star-history.com/#alloy-rs/uint&Date)
[![crates.io](https://img.shields.io/crates/d/ruint2)](https://crates.io/crates/ruint2)
