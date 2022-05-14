# Rust `uint` crate using const-generics

Implements `Uint<N: usize>` where `N` is the number of bits. That is, it implements the ring of numbers modulo 2ⁿ.

```rust
use uint::{Uint, OverflowingAdd};

let a: Uint<256> = Uint::one();
let b: Uint<256> = 42_u64.try_into().unwrap();
let c = a.overflowing_add(b);
dbg!(c);
```

## The `uint` macro

The above can also be written using the [`uint`] macro. Within the macro arguments,
you can write [`Uint`] literals using the [same syntax][rust-syntax] as Rust integer
literals, but using a capital `U` in the suffix instead of lowercase.

[rust-syntax]: https://doc.rust-lang.org/stable/reference/tokens.html#integer-literals

To use it simply import it in scope:

```rust
use uint::uint;
```

Now constants can be created in decimal, hex, binary and even octal:

```rust
# use uint::uint;
let avogadro = uint!(602_214_076_000_000_000_000_000_U256);
let cow_key = uint!(0xee79b5f6e221356af78cf4c36f4f7885a11b67dfcc81c34d80249947330c0f82_U256);
let bender = uint!(0b1010011010_U10);
```

The [`uint`] macro recurses through the parse tree, so the above can equivalently be written

```rust
# use uint::uint;
uint!{
let avogadro = 602_214_076_000_000_000_000_000_U256;
let cow_key = 0xee79b5f6e221356af78cf4c36f4f7885a11b67dfcc81c34d80249947330c0f82_U256;
let bender = 0b1010011010_U10;
}
```

This latter form is particularly useful for lookup tables:

```rust
# use uint::{Uint, uint};
const PRIMES: [Uint<128>; 3] = uint!([
    170141183460469231731687303715884105757_U128,
    170141183460469231731687303715884105773_U128,
    170141183460469231731687303715884105793_U128,
]);
```

The macro will throw a compile time error if you try to create a constant that
does not fit the type:

```rust,compile_fail
# use uint::uint;
# uint!{
let sparta = 300_U8;
# }
```

```text,ignore
error: Value too large for Uint<8>: 300
 --> src/example.rs:1:14
  |
1 | let sparta = 300_U8;
  |              ^^^^^^
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

