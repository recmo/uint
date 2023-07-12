# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- Section names: Added, Changed, Deprecated, Removed, Fixed, Security -->

## [Unreleased]

### Changed

- Make `serde::Deserialize` impl more permissive
- Use Ethereum `Quantity` encoding for serde serialization when human-readable
- Fix error in `from_base_be` that allowed instantiation of overflowing `Uint`.
- Updated `ark` to `0.4`, `fastrlp` to `0.3` and `pyo3` to `0.18`.

### Fixed

- `from_base_le` implementation by reversing the input iterator

## [1.8.0] — 2023-04-19

### Added

- Support `bn-rs`, `serde` and `uint!` for `Bits`

### Fixed

- Serde human readable now encodes the empty bitstring as `0x0` and rejects zero prefixes.

## [1.7.0] — 2022-10-31

### Added

- Support `rlp` for `Bits`

### Fixed

- Edge case in which an overflow occurs when parsing a `Uint` with `uint!` ([#199]).

[#199]: https://github.com/recmo/uint/issues/199

## [1.6.0] — 2022-10-28

### Added

- `TryFrom<Uint> for usize`
- Bit type aliases (`B128`, `B160`, `B256`, `B512`, ...)
- `From` and `Into` trait implementations for `primitive-types` bit types
- Support for `bn-rs`
- Derive `Default` for `Bits`

### Changed

- (Breaking) Changed the arguments of `pow` and `log` to `Uint`.
- More efficient `wrapping_pow` implementation.

## [1.5.1] — 2022-10-24

### Changed

- Performance improvements in `wrapping_mul` and `from_bytes_be`.

## [1.5.0] — 2022-10-24

### Added

- Add `parity-scale-codec` support.
- Added unstable `algorithms::div` module and improved div algorithm.

## [1.4.1] — 2022-10-15

### Changed

- Made `primitive-types` version flexible.

## [1.4.0] — 2022-10-02

### Added

- Add `Pyo3` support.
- `from` now supports `Uint` arguments.
- `saturating_from`, `wrapping_from`, `to`, `wrapping_to`, `saturating_to`.
- `wrapping_from_limbs_slice`, `overflowing_from_limbs_slice`, `saturating_from_limbs_slice`.
- Add `zeroize` and `valuable` support.

### Changed

- `ToUintError` and `FromUintError` now contain wrapped value and other context.
- `from_uint` and `checked_from_uint` are now deprecated.

## [1.3.0] — 2022-06-08

### Added

- Added `inv_mod`, `mul_redc`, `gcd`, `gcd_extended`, `lcm`.
- Added `sqlx` support.

### Changed

- Renamed `ring_inverse` to `inv_ring`.

## [1.2.0] — 2022-06-03

### Added

- Added `reduce_mod`, `add_mod`, `mul_mod`, `pow_mod`.
- Added `num-bigint` and `ark-ff` support.
- Made `algorithms` public, but with unstable API for now.

### Changed

- Marked `Uint::as_limbs_mut` as unsafe.
- Unified `mul` implementations and move to `algorithms`.

### Fixed

- `uint!` macro incorrectly accepting hex digits in decimal.

## [1.1.0] — 2022-05-31

### Added

- Added `saturating_shl`.
- Added `approx_log`, `approx_log2`, `approx_log10` for `f64` log approximations.
- Added `approx_pow2` to construct from `f64` log2 approximation.
- Added `root` computing integer roots.

### Changed

- Made logarithms `usize` to match `BITS` in `pow`, `log` functions.
- Applied `track_caller` to div/rem ops to track div-by-zero easier.

## [1.0.0] — 2022-05-28

### Added

- Added comparison.
- Added add, sub, neg and sum functions.
- Added mul functions.
- Added division and remainder functions.
- Added pow and log functions.
- Added `next_power_of_two` and `next_multiple_of` functions.
- Added `checked_from_limb_slice` and `from_uint`.

### Changed

- `from_limb_slice` now handles arbitrary length slices.

## [0.3.0] — 2022-05-23

### Added

- All the binary operations (not, and, or, xor, shifts, rotate, etc)
- `Bits`, a newtype wrapped `Uint` restricted to non-numeric operations.
- Postgres `FromSql` support and JSON column support.
- `from_base_le` and `from_base_be` base conversion.
- `from_str_radix` string base conversion up to base64.
- `FromStr` trait with decimal, hex, octal and binary support.

### Changed

- `reverse_bits` is now by value instead of `&mut self`.

## [0.2.1] — 2022-05-18

### Added

- Extensive Postgres ToSql support supporting many column types.
- `TryFrom<Uint>` for primitive integer types.
- Added `From` conversions to `f32` and `f64`.
- Implement all rust formatting: `Debug`, `Display`, decimal, hex, etc.
- `>>=` operator.
- `to_base_le` and `to_base_le` base conversion spigots
- `reverse_bits`, `most_sigificant_bits` bit methods.
- Optimized `as_le_{slice,bytes)_*` accessors.

### Changed

- Rewrote `to_{be,le}_bytes_*` to use optimized methods. This has trickle-down
  effects for a lot of conversions and formatting.

## [0.2.0] — 2022-05-16

### Added

- Changelog
- CI test on stable Rust.
- Common bit-size aliases and nightly-only `Uint<BITS>` alias.
- Added `to_{be/le}_bytes_vec` and made `try_from_le_byte_iter` public.
- Added `rlp` and `fastrlp` support.
- Added `into_limbs`, `leading_zeros`, `bit_len`, `byte_len`, `checked_log2`.
- Added `primitive-types` support.

### Changed

- Changed to `Uint<BITS, LIMBS>` to get stable compatibility!
- Added generic `BYTES` parameter to `to_{be/le}_bytes`.
- Renamed `try_from_{be/le}_slice`.

## [0.1.0] — 2022-05-15

### Added

- Const-generic `Uint` structure.
- Basic `overflowing_add` implementation.
- Algorithms for division and gcd (currently unused).
- `uint!` and `const_for!` macros.
- Documentation with examples.
- Support for rand, arbitrary, quickcheck, proptest, serde
- Github actions for linting, testing, code coverage, cargo-audit.
- Pushed to crates.io.

<!-- links to version -->

[unreleased]: https://github.com/recmo/uint/compare/v1.8.0...HEAD
[1.8.0]: https://github.com/recmo/uint/releases/tag/v1.6.0
[1.6.0]: https://github.com/recmo/uint/releases/tag/v1.6.0
[1.5.1]: https://github.com/recmo/uint/releases/tag/v1.5.1
[1.5.0]: https://github.com/recmo/uint/releases/tag/v1.5.0
[1.4.1]: https://github.com/recmo/uint/releases/tag/v1.4.1
[1.4.0]: https://github.com/recmo/uint/releases/tag/v1.4.0
[1.3.0]: https://github.com/recmo/uint/releases/tag/v1.3.0
[1.2.0]: https://github.com/recmo/uint/releases/tag/v1.2.0
[1.1.0]: https://github.com/recmo/uint/releases/tag/v1.1.0
[1.0.0]: https://github.com/recmo/uint/releases/tag/v1.0.0
[0.3.0]: https://github.com/recmo/uint/releases/tag/v0.3.0
[0.2.1]: https://github.com/recmo/uint/releases/tag/v0.2.1
[0.2.0]: https://github.com/recmo/uint/releases/tag/v0.2.0
[0.1.0]: https://github.com/recmo/uint/releases/tag/v0.1.0
