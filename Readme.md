# Rust `uint` crate using const-generics

```rust
use uint::{Uint, OverflowingAdd};

let a: Uint<256> = Uint::one();
let b: Uint<256> = Uint::one();
let c = a.overflowing_add(b);
dbg!(c);
```

## References

<https://courses.cs.washington.edu/courses/cse469/18wi/Materials/arm64.pdf>

