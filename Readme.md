# Rust `uint` crate using const-generics

[![crates.io](https://buildstats.info/crate/ruint)](https://crates.io/crates/ruint)
[![docs.rs](https://img.shields.io/docsrs/ruint)](https://docs.rs/ruint)
[![MIT License](https://img.shields.io/github/license/recmo/uint)](https://github.com/recmo/uint/blob/main/mit-license.md)
[![dependency status](https://deps.rs/repo/github/recmo/uint/status.svg)](https://deps.rs/repo/github/recmo/uint)
[![codecov](https://codecov.io/gh/recmo/uint/branch/main/graph/badge.svg?token=WBPZ9U4TTO)](https://codecov.io/gh/recmo/uint)
[![CI](https://github.com/recmo/uint/actions/workflows/ci.yml/badge.svg)](https://github.com/recmo/uint/actions/workflows/ci.yml)

Implements [`Uint<BITS>`], the ring of numbers modulo $2^{\mathtt{BITS}}$.

```rust
# #![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use ruint::{Uint, OverflowingAdd};

let a: Uint<256> = Uint::from(0xf00f_u64);
let b: Uint<256> = Uint::from(42_u64);
let (c, _carry) = a.overflowing_add(b);
assert_eq!(c, Uint::from(0xf039_u64));
```

Or equivalently using the convenient [`uint!`] macro:

```rust
# #![allow(incomplete_features)]
# #![feature(generic_const_exprs)]
use ruint::{uint, OverflowingAdd};
uint!{

let a = 0xf00f_U256;
let b = 42_U256;
let (c, _carry) = a.overflowing_add(b);
assert_eq!(c, 0xf039_U256);

}
```

It can also be used in a more natural expression form if that is preferred

```rust
# #![allow(incomplete_features)]
# #![feature(generic_const_exprs)]
# use ruint::uint;
#
let cow = uint!(0xf039_U42);
```

## Feature flags

There is support for a number of crates. These are enabled by setting the identically
named feature flag.

* [`rand`](https://docs.rs/rand): Implements sampling from the [`Standard`](https://docs.rs/rand/latest/rand/distributions/struct.Standard.html) distribution, i.e. [`rng.gen()`](https://docs.rs/rand/latest/rand/trait.Rng.html#method.gen).
* [`arbitrary`](https://docs.rs/arbitrary): Implements the [`Arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html) trait, allowing [`Uint`]s to be generated for fuzz testing. 
* [`quickcheck`](https://docs.rs/quickcheck): Implements the [`Arbitrary`](https://docs.rs/quickcheck/latest/quickcheck/trait.Arbitrary.html) trait, allowing [`Uint`]s to be generated for property based testing.
* [`proptest`](https://docs.rs/proptest): Implements the [`Arbitrary`](https://docs.rs/proptest/latest/proptest/arbitrary/trait.Arbitrary.html) trait, allowing [`Uint`]s to be generated for property based testing. Proptest is used for the `uint`s own test suite.
* [`serde`](https://docs.rs/serde): Implements the [`Seralize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) traits for [`Uint`] using big-endian hex in human readable formats and big-endian byte strings in machine readable formats.

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

Maybe:

* Run-time sized type with compatible interface.
* Montgomery REDC and other algo's for implementing prime fields.

## FAQ

> Why does it need nightly Rust>


It makes critical use of the `generic_const_exprs` feature to compute the number
of limbs required for a given bit size.

* Rust issues [#60551](https://github.com/rust-lang/rust/issues/60551) and [#76560](https://github.com/rust-lang/rust/issues/76560).


> What's up with all the
> 
> ```rust,ignore
> where
>     [(); num_limbs(BITS)]:,
> ```
> 
> trait bounds everywhere?

Const generics are still pretty unfinished in rust. This is to work around current limitations. Finding a less invasive workaround is high priority. Fortunately, this is only needed when writing
code generic over the value of `BITS`. But this only affects you if you write code generic over the bit size. If you use a specific size like `Uint<256>` you do not need these bounds.

<>

More information:

* [Working group](https://rust-lang.github.io/project-const-generics/) const generics working group.
* [RFC2000](https://rust-lang.github.io/rfcs/2000-const-generics.html) const generics.
* [#60551](https://github.com/rust-lang/rust/issues/60551) associated constants in const generics.
* [#76560](https://github.com/rust-lang/rust/issues/76560) tracking issue for `generic_const_exprs`.
* [Rust blog](https://blog.rust-lang.org/inside-rust/2021/09/06/Splitting-const-generics.html) 2021-09-06 Splitting const generics.

<https://github.com/RustCrypto/traits/issues/970>

<https://docs.rs/crypto-bigint/latest/crypto_bigint/struct.UInt.html>

---

[![lines of code](https://img.shields.io/tokei/lines/github/recmo/uint)](https://github.com/recmo/uint)
[![GitHub contributors](https://img.shields.io/github/contributors/recmo/uint)](https://github.com/recmo/uint/graphs/contributors)
[![GitHub issues](https://img.shields.io/github/issues/recmo/uint)](https://github.com/recmo/uint/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/recmo/uint?label=PRs)](https://github.com/recmo/uint/pulls)
[![GitHub Repo stars](https://img.shields.io/github/stars/recmo/uint)](https://github.com/recmo/uint/stargazers)
[![crates.io](https://img.shields.io/crates/d/ruint)](https://crates.io/crates/ruint)
