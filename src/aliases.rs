//! Type aliases for common bit sizes of [`Uint`].
use crate::{Bits, Uint};

/// [`Uint`] for `0` bits. Always zero. Similar to `()`.
pub type U0 = Uint<0, 0>;

/// [`Uint`] for `1` bit. Similar to [`bool`].
pub type U1 = Uint<1, 1>;

/// [`Uint`] for `8` bits. Similar to [`u8`].
pub type U8 = Uint<8, 1>;

/// [`Uint`] for `16` bits. Similar to [`u16`].
pub type U16 = Uint<16, 1>;

/// [`Uint`] for `32` bits. Similar to [`u32`].
pub type U32 = Uint<32, 1>;

/// [`Uint`] for `64` bits. Similar to [`u64`].
pub type U64 = Uint<64, 1>;

/// [`Uint`] for `128` bits. Similar to [`u128`].
pub type U128 = Uint<128, 2>;

/// [`Uint`] for `192` bits.
pub type U192 = Uint<192, 3>;

/// [`Uint`] for `256` bits.
pub type U256 = Uint<256, 4>;

/// [`Uint`] for `320` bits.
pub type U320 = Uint<320, 5>;

/// [`Uint`] for `384` bits.
pub type U384 = Uint<384, 6>;

/// [`Uint`] for `448` bits.
pub type U448 = Uint<448, 7>;

/// [`Uint`] for `512` bits.
pub type U512 = Uint<512, 8>;

/// [`Uint`] for `1024` bits.
pub type U1024 = Uint<1024, 16>;

/// [`Uint`] for `2048` bits.
pub type U2048 = Uint<2048, 32>;

/// [`Uint`] for `4096` bits.
pub type U4096 = Uint<4096, 64>;

/// [`Bits`] for `128` bits.
pub type H128 = Bits<128, 2>;

/// [`Bits`] for `160` bits.
pub type H160 = Bits<160, 3>;

/// [`Bits`] for `256` bits.
pub type H256 = Bits<256, 4>;

/// [`Bits`] for `512` bits.
pub type H512 = Bits<512, 8>;

// TODO: B0, B1, B8, ... B4096
// TODO: I0, I1, I8, ... I4096
