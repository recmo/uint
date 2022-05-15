# Rust `uint` crate using const-generics

Implements [`Uint<BITS>`], the ring of numbers modulo $2^{\mathtt{BITS}}$.

```rust
#![feature(generic_const_exprs)]
use uint::{Uint, OverflowingAdd};

let a: Uint<256> = Uint::from(0xf00f_u64);
let b: Uint<256> = Uint::from(42_u64);
let (c, _carry) = a.overflowing_add(b);
assert_eq!(c, Uint::from(0xf039_u64));
```

Or equivalently using the convenient [`uint!`] macro:

```rust
# #![feature(generic_const_exprs)]
use uint::{uint, OverflowingAdd};
uint!{

let a = 0xf00f_U256;
let b = 42_U256;
let (c, _carry) = a.overflowing_add(b);
assert_eq!(c, 0xf039_U256);

}
```

It can also be used in a more natural expression form if that is preferred

```rust
# #![feature(generic_const_exprs)]
# use uint::uint;
#
let cow = uint!(0xf039_U42);
```

## Feature flags

* [`rand`](https://docs.rs/rand), [`arbitrary`](https://docs.rs/arbitrary), [`quickcheck`](https://docs.rs/quickcheck), [`proptest`](https://docs.rs/proptest). For all these optional dependencies support for generating values is build in. Support is automatically activated when the crate is in the dependency tree.

## Benchmarks and tests

Run benchmarks with

```sh
cargo criterion
```

Goals:

* All the quality of life features one could want.
* Compatible with std `u64`, etc types.
* Builds `no-std` and `wasm`.
* Fast platform agnostic generic algorithms.
* Target specific assembly optimizations (where available).
* Macro to create constants from long literals.
* Optional rand, proptest, serde, num-traits, etc, support.

Maybe:

* Run-time sized type with compatible interface.
* Montgomery REDC and other algo's for implementing prime fields.

## FAQ

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

* Rust issue [#79778](<https://github.com/rust-lang/rust/issues/79778>)


## References

* Rust [integer methods](https://doc.rust-lang.org/stable/std/primitive.u64.html)

