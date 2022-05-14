# Rust `uint` crate using const-generics

Implements `Uint<N: usize>` where `N` is the number of bits. That is, it implements the ring of numbers modulo 2ⁿ.

```rust
use uint::{Uint, OverflowingAdd};

let a: Uint<256> = Uint::one();
let b: Uint<256> = 42_u64.try_into().unwrap();
let c = a.overflowing_add(b);
dbg!(c);
```

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
code generic over the value of `BITS`. Users 

* Rust issue [#79778](<https://github.com/rust-lang/rust/issues/79778>)


## References

* Rust [integer methods](https://doc.rust-lang.org/stable/std/primitive.u64.html)

