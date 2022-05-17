# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- Section names: Added, Changed, Deprecated, Removed, Fixed, Security -->

## [Unreleased]

### Added

- Changelog
- CI test on stable Rust.
- Common bit-size aliases and nightly-only `Uint<BITS>` alias.
- Added `to_{be/le}_bytes_vec` and made `try_from_le_byte_iter` public.
- Added `rlp` and `fastrlp` support.
- Added `leading_zeros`.

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

[unreleased]: https://github.com/recmo/uint/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/recmo/uint/releases/tag/v0.1.0
