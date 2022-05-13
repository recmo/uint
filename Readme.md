# Rust `uint` crate using const-generics

Implements `Uint<N: usize>` where `N` is the number of bits. That is, it implements the ring of numbers modulo 2ⁿ.

```rust
use uint::{Uint, OverflowingAdd};

let a: Uint<256> = Uint::one();
let b: Uint<256> = Uint::one();
let c = a.overflowing_add(b);
dbg!(c);
```

Run benchmarks with

```sh
cargo criterion
```

Goals:

* All the quality of life features one could want.
* Builds `no-std` and `wasm`.
* Fast platform agnostic generic algorithms.
* Target specific assembly optimizations (where available).

Maybe:

* Run-time sized type with compatible interface.
* Montgomery REDC and other algo's for implementing prime fields.

## References

<https://courses.cs.washington.edu/courses/cse469/18wi/Materials/arm64.pdf>

