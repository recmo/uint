# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- Section names: Added, Changed, Deprecated, Removed, Fixed, Security -->

## [Unreleased]

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

[unreleased]: https://github.com/recmo/uint/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/recmo/uint/releases/tag/v1.0.0
[0.3.0]: https://github.com/recmo/uint/releases/tag/v0.3.0
[0.2.1]: https://github.com/recmo/uint/releases/tag/v0.2.1
[0.2.0]: https://github.com/recmo/uint/releases/tag/v0.2.0
[0.1.0]: https://github.com/recmo/uint/releases/tag/v0.1.0
