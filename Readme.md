# Rust `uint` crate using const-generics

[![crates.io](https://buildstats.info/crate/ruint)](https://crates.io/crates/ruint)
[![docs.rs](https://img.shields.io/docsrs/ruint)](https://docs.rs/ruint)
[![MIT License](https://img.shields.io/github/license/recmo/uint)](https://github.com/recmo/uint/blob/main/mit-license.md)
[![dependency status](https://deps.rs/repo/github/recmo/uint/status.svg)](https://deps.rs/repo/github/recmo/uint)
[![codecov](https://codecov.io/gh/recmo/uint/branch/main/graph/badge.svg?token=WBPZ9U4TTO)](https://codecov.io/gh/recmo/uint)
[![CI](https://github.com/recmo/uint/actions/workflows/ci.yml/badge.svg)](https://github.com/recmo/uint/actions/workflows/ci.yml)

Implements [`Uint<BITS, LIMBS>`], the ring of numbers modulo $2^{\mathtt{BITS}}$. It requires two
generic arguments: the number of bits and the number of 64-bit 'limbs' required to store those bits.

```rust
# use ruint::Uint;
let answer: Uint<256, 4> = Uint::from(42);
```

You can compute `LIMBS` yourself using $\mathtt{LIMBS} = \ceil{\mathtt{BITS} / 64}$,
i.e.`LIMBS` equals `BITS` divided by $64$ rounded up. [`Uint`] will `panic!` if you try to
construct it with incorrect arguments. Ideally this would be a compile time error, but
that is blocked by Rust issue [#60551][r60551].

[r60551]: https://github.com/rust-lang/rust/issues/60551

A more convenient method on stable is to use the [`uint!`] macro, which constructs the right
[`Uint`] for you.

```rust
# use ruint::{Uint, uint};
let answer = uint!(42_U256);
```

You can also use one of the pre-computed type [`aliases`]:

```rust
# use ruint::Uint;
use ruint::aliases::*;

let answer: U256 = Uint::from(42);
```

You can of course also create your own type alias if you need a funny size:

```rust
# use ruint::Uint;
type U1337 = Uint<1337, 21>;

let answer: U1337 = Uint::from(42);
```

## Rust nightly

If you are on nightly, you can use [`Uint<BITS>`][nightly::Uint] which will
compute the number of limbs for you. Unfortunately this can not be made stable
without `generic_const_exprs` support (Rust issue [#76560][r76560]).

[r76560]: https://github.com/rust-lang/rust/issues/76560

```rust
# #[cfg(has_generic_const_exprs)] {
use ruint::nightly::Uint;

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
use ruint::Uint;

let a: Uint<256, 4> = Uint::from(0xf00f_u64);
let b: Uint<256, 4> = Uint::from(42_u64);
let c  = a + b;
assert_eq!(c, Uint::from(0xf039_u64));
```

There is a convenient macro [`uint!`] to create constants for you. It allows
for arbitrary length constants using standard Rust integer syntax. The size of
the [`Uint`] is specified with a `U` suffix followed by the number of bits.
The standard Rust syntax of decimal, hexadecimal and even binary and octal is
supported using their prefixes `0x`, `0b` and `0o`. Literals can have
underscores `_` added for readability.

```rust
# use ruint::uint;
let cow = uint!(0xc85ef7d79691fe79573b1a7064c19c1a9819ebdbd1faaab1a8ec92344438aaf4_U256);
```

In fact, this macro recurses down the parse tree, so you can apply it to entire
source files:

```rust
# use ruint::uint;
uint!{

let a = 42_U256;
let b = 0xf00f_1337_c0d3_U256;
let c = a + b;
assert_eq!(c, 263947537596669_U256);

}
```

## Feature flags

There is support for a number of crates. These are enabled by setting the identically
named feature flag.

* [`rand`](https://docs.rs/rand): Implements sampling from the [`Standard`](https://docs.rs/rand/latest/rand/distributions/struct.Standard.html) distribution, i.e. [`rng.gen()`](https://docs.rs/rand/latest/rand/trait.Rng.html#method.gen).
* [`arbitrary`](https://docs.rs/arbitrary): Implements the [`Arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html) trait, allowing [`Uint`]s to be generated for fuzz testing.
* [`quickcheck`](https://docs.rs/quickcheck): Implements the [`Arbitrary`](https://docs.rs/quickcheck/latest/quickcheck/trait.Arbitrary.html) trait, allowing [`Uint`]s to be generated for property based testing.
* [`proptest`](https://docs.rs/proptest): Implements the [`Arbitrary`](https://docs.rs/proptest/latest/proptest/arbitrary/trait.Arbitrary.html) trait, allowing [`Uint`]s to be generated for property based testing. Proptest is used for the `uint`s own test suite.
* [`serde`](https://docs.rs/serde): Implements the [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) traits for [`Uint`] using big-endian hex in human readable formats and big-endian byte strings in machine readable formats.
* [`rlp`](https://docs.rs/rlp): Implements the [`Encodable`](https://docs.rs/rlp/latest/rlp/trait.Encodable.html) and [`Decodable`](https://docs.rs/rlp/latest/rlp/trait.Decodable.html) traits for [`Uint`] to allow serialization to/from RLP.
* [`fastrlp`](https://docs.rs/fastrlp): Implements the [`Encodable`](https://docs.rs/fastrlp/latest/fastrlp/trait.Encodable.html) and [`Decodable`](https://docs.rs/fastrlp/latest/fastrlp/trait.Decodable.html) traits for [`Uint`] to allow serialization to/from RLP.
* [`primitive-types`](https://docs.rs/primitive-types): Implements the [`From<_>`] conversions between corresponding types.
* [`postgres`](https://docs.rs/postgres): Implements the [`ToSql`](https://docs.rs/postgres/latest/postgres/types/trait.ToSql.html) trait supporting many column types.

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

## To do

Goals:

* All the quality of life features one could want.
* Compatible with std `u64`, etc types. See Rust's [integer methods](https://doc.rust-lang.org/stable/std/primitive.u64.html).
* Builds `no-std` and `wasm`.
* Fast platform agnostic generic algorithms.
* Target specific assembly optimizations (where available).
* Optional num-traits, etc, support.
* Adhere to [Rust API Guidelines](https://rust-lang.github.io/api-guidelines)

Maybe:

* Run-time sized type with compatible interface.
* Montgomery REDC and other algo's for implementing prime fields.

---

[![lines of code](https://img.shields.io/tokei/lines/github/recmo/uint)](https://github.com/recmo/uint)
[![GitHub contributors](https://img.shields.io/github/contributors/recmo/uint)](https://github.com/recmo/uint/graphs/contributors)
[![GitHub issues](https://img.shields.io/github/issues/recmo/uint)](https://github.com/recmo/uint/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/recmo/uint?label=PRs)](https://github.com/recmo/uint/pulls)
[![GitHub Repo stars](https://img.shields.io/github/stars/recmo/uint)](https://star-history.com/#recmo/uint&Date)
[![crates.io](https://img.shields.io/crates/d/ruint)](https://crates.io/crates/ruint)
