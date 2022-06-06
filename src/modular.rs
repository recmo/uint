use crate::{algorithms, nlimbs, Uint};

// FEATURE: sub_mod, neg_mod, inv_mod, div_mod, root_mod
// FEATURE: mul_mod_redc
// FEATURE: Modular wrapper class, like Wrapping.

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    #[allow(clippy::doc_markdown)] // False positive
    /// ⚠️ Compute $\mod{\mathtt{self}}_{\mathtt{modulus}}$.
    ///
    /// **Warning.** This function is not part of the stable API.
    ///
    /// Returns zero if the modulus is zero.
    // FEATURE: Reduce larger bit-sizes to smaller ones.
    #[must_use]
    pub fn reduce_mod(mut self, modulus: Self) -> Self {
        if modulus == Self::ZERO {
            return Self::ZERO;
        }
        if self >= modulus {
            self %= modulus;
        }
        self
    }

    #[allow(clippy::doc_markdown)] // False positive
    /// Compute $\mod{\mathtt{self} + \mathtt{rhs}}_{\mathtt{modulus}}$.
    ///
    /// Returns zero if the modulus is zero.
    #[must_use]
    pub fn add_mod(self, rhs: Self, modulus: Self) -> Self {
        // Reduce inputs
        let lhs = self.reduce_mod(modulus);
        let rhs = rhs.reduce_mod(modulus);

        // Compute the sum and conditionaly subtract modulus once.
        let (mut result, overflow) = lhs.overflowing_add(rhs);
        if overflow || result >= modulus {
            result -= modulus;
        }
        result
    }

    #[allow(clippy::doc_markdown)] // False positive
    /// Compute $\mod{\mathtt{self} ⋅ \mathtt{rhs}}_{\mathtt{modulus}}$.
    ///
    /// Returns zero if the modulus is zero.
    ///
    /// See [`mul_redc`](Self::mul_redc) for a faster variant at the cost of
    /// some pre-computation.
    #[must_use]
    pub fn mul_mod(self, rhs: Self, mut modulus: Self) -> Self {
        if modulus == Self::ZERO {
            return Self::ZERO;
        }
        // Compute full product.
        // The challenge here is that Rust doesn't allow us to create a
        // `Uint<2 * BITS, _>` for the intermediate result. Otherwise
        // we could just use a `widening_mul`. So instead we allocate from heap.
        // Alternatively we could use `alloca`, but that is blocked on
        // See <https://github.com/rust-lang/rust/issues/48055>
        let mut product = vec![0; nlimbs(2 * BITS)];
        let overflow = algorithms::mul_inline(&self.limbs, &rhs.limbs, &mut product);
        debug_assert!(!overflow);

        // Compute modulus using `div_rem`.
        // This stores the remainder in the divisor, `modulus`.
        algorithms::div_rem(&mut product, &mut modulus.limbs);

        modulus
    }

    #[allow(clippy::doc_markdown)] // False positive
    /// Compute $\mod{\mathtt{self}^{\mathtt{rhs}}}_{\mathtt{modulus}}$.
    ///
    /// Returns zero if the modulus is zero.
    #[must_use]
    pub fn pow_mod(mut self, mut exp: Self, modulus: Self) -> Self {
        if modulus == Self::ZERO || modulus <= Self::from(1) {
            // Also covers Self::BITS == 0
            return Self::ZERO;
        }

        // Exponentiation by squaring
        let mut result = Self::from(1);
        while exp > Self::ZERO {
            // Multiply by base
            if exp.limbs[0] & 1 == 1 {
                result = result.mul_mod(self, modulus);
            }

            // Square base
            self = self.mul_mod(self, modulus);
            exp >>= 1;
        }
        result
    }

    /// Compute $\mod{\mathtt{self}^{-1}}_{\mathtt{modulus}}$.
    ///
    /// Returns `None` if the inverse does not exist.
    #[must_use]
    pub fn inv_mod(self, modulus: Self) -> Option<Self> {
        algorithms::inv_mod(self, modulus)
    }

    #[allow(clippy::doc_markdown)] // False positive
    /// Montgomery multiplication.
    ///
    /// Computes
    ///
    /// $$
    /// \mod{\frac{\mathtt{self} ⋅ \mathtt{other}}{ 2^{64 ·
    /// \mathtt{LIMBS}}}}_{\mathtt{modulus}} $$
    ///
    /// This is useful because it can be computed notably faster than
    /// [`mul_mod`](Self::mul_mod). Many computations can be done by
    /// pre-multiplying values with $R = 2^{64 · \mathtt{LIMBS}}$
    /// and then using [`mul_redc`](Self::mul_redc) instead of
    /// [`mul_mod`](Self::mul_mod).
    ///
    /// For this algorithm to work, it needs an extra parameter `inv` which must
    /// be set to
    ///
    /// $$
    /// \mathtt{inv} = \mod{\frac{-1}{\mathtt{modulus}} }_{2^{64}}
    /// $$
    ///
    /// The `inv` value only exists for odd values of `modulus`. It can be
    /// computed using [`inv_ring`](Self::inv_ring) from `U64`.
    ///
    /// ```
    /// # use ruint::{uint, Uint, aliases::*};
    /// # uint!{
    /// # let modulus = 21888242871839275222246405745257275088548364400416034343698204186575808495617_U256;
    /// let inv = (-U64::from(modulus.as_limbs()[0]).inv_ring().unwrap()).as_limbs()[0];
    /// # assert_eq!(inv.wrapping_mul(modulus.as_limbs()[0]), u64::MAX);
    /// # assert_eq!(inv, 0xc2e1f593efffffff);
    /// # }
    /// ```
    // TODO: Improve the conversion dev-ex.
    pub fn mul_redc(self, other: Self, modulus: Self, inv: u64) -> Self {
        if BITS == 0 {
            return Self::ZERO;
        }
        assert_eq!(inv.wrapping_mul(modulus.limbs[0]), u64::MAX);
        let mut result = Self::ZERO;
        algorithms::mul_redc(
            &self.limbs,
            &other.limbs,
            &mut result.limbs,
            &modulus.limbs,
            inv,
        );
        debug_assert!(result < modulus);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{aliases::U64, const_for, nlimbs};
    use core::cmp::min;
    use proptest::{prop_assume, proptest, test_runner::Config};

    #[test]
    fn test_commutative() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(a: U, b: U, m: U)| {
                assert_eq!(a.mul_mod(b, m), b.mul_mod(a, m));
            });
        });
    }

    #[test]
    fn test_associative() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(a: U, b: U, c: U, m: U)| {
                assert_eq!(a.mul_mod(b.mul_mod(c, m), m), a.mul_mod(b, m).mul_mod(c, m));
            });
        });
    }

    #[test]
    fn test_distributive() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(a: U, b: U, c: U, m: U)| {
                assert_eq!(a.mul_mod(b.add_mod(c, m), m), a.mul_mod(b, m).add_mod(a.mul_mod(c, m), m));
            });
        });
    }

    #[test]
    fn test_add_identity() {
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(value: U, m: U)| {
                assert_eq!(value.add_mod(U::from(0), m), value.reduce_mod(m));
            });
        });
    }

    #[test]
    fn test_mul_identity() {
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(value: U, m: U)| {
                assert_eq!(value.mul_mod(U::from(0), m), U::ZERO);
                assert_eq!(value.mul_mod(U::from(1), m), value.reduce_mod(m));
            });
        });
    }

    #[test]
    fn test_pow_identity() {
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(a: U, m: U)| {
                assert_eq!(a.pow_mod(U::from(0), m), U::from(1).reduce_mod(m));
                assert_eq!(a.pow_mod(U::from(1), m), a.reduce_mod(m));
            });
        });
    }

    #[test]
    fn test_pow_rules() {
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            // TODO: Increase cases when perf is better.
            let mut config = Config::default();
            config.cases = min(config.cases, if BITS > 500 { 3 } else { 10 });
            proptest!(config, |(a: U, b: U, c: U, m: U)| {
                // TODO: a^(b+c) = a^b * a^c. Which requires carmichael fn.
                // TODO: (a^b)^c = a^(b * c). Which requires carmichael fn.
                assert_eq!(a.mul_mod(b, m).pow_mod(c, m), a.pow_mod(c, m).mul_mod(b.pow_mod(c, m), m));
            });
        });
    }

    #[test]
    fn test_inv() {
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(a: U, m: U)| {
                if let Some(inv) = a.inv_mod(m) {
                    assert_eq!(a.mul_mod(inv, m), U::from(1));
                }
            });
        });
    }

    #[test]
    fn test_mul_redc() {
        const_for!(BITS in NON_ZERO if (BITS >= 16) {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(a: U, b: U, m: U)| {
                prop_assume!(m >= U::from(2));
                if let Some(inv) = U64::from(m.as_limbs()[0]).inv_ring() {
                    let inv = (-inv).as_limbs()[0];

                    let r = U::from(2).pow_mod(U::from(64 * LIMBS), m);
                    let ar = a.mul_mod(r, m);
                    let br = b.mul_mod(r, m);
                    // TODO: Test for larger (>= m) values of a, b.

                    let expected = a.mul_mod(b, m).mul_mod(r, m);

                    assert_eq!(ar.mul_redc(br, m, inv), expected);
                }
            });
        });
    }
}

#[cfg(feature = "bench")]
pub mod bench {
    use super::*;
    use crate::{const_for, nlimbs};
    use ::proptest::{
        arbitrary::Arbitrary,
        strategy::{Strategy, ValueTree},
        test_runner::TestRunner,
    };
    use criterion::{black_box, BatchSize, Criterion};

    pub fn group(criterion: &mut Criterion) {
        const_for!(BITS in BENCH {
            const LIMBS: usize = nlimbs(BITS);
            bench_reduce::<BITS, LIMBS>(criterion);
            bench_add::<BITS, LIMBS>(criterion);
            bench_mul::<BITS, LIMBS>(criterion);
            bench_pow::<BITS, LIMBS>(criterion);
            bench_inv::<BITS, LIMBS>(criterion);
        });
    }

    fn bench_reduce<const BITS: usize, const LIMBS: usize>(criterion: &mut Criterion) {
        let input = (Uint::<BITS, LIMBS>::arbitrary(), Uint::arbitrary());
        let mut runner = TestRunner::deterministic();
        criterion.bench_function(&format!("reduce_mod/{}", BITS), move |bencher| {
            bencher.iter_batched(
                || input.new_tree(&mut runner).unwrap().current(),
                |(a, m)| black_box(black_box(a).reduce_mod(black_box(m))),
                BatchSize::SmallInput,
            );
        });
    }

    fn bench_add<const BITS: usize, const LIMBS: usize>(criterion: &mut Criterion) {
        let input = (
            Uint::<BITS, LIMBS>::arbitrary(),
            Uint::arbitrary(),
            Uint::arbitrary(),
        );
        let mut runner = TestRunner::deterministic();
        criterion.bench_function(&format!("add_mod/{}", BITS), move |bencher| {
            bencher.iter_batched(
                || input.new_tree(&mut runner).unwrap().current(),
                |(a, b, m)| black_box(black_box(a).add_mod(black_box(b), black_box(m))),
                BatchSize::SmallInput,
            );
        });
    }

    fn bench_mul<const BITS: usize, const LIMBS: usize>(criterion: &mut Criterion) {
        let input = (
            Uint::<BITS, LIMBS>::arbitrary(),
            Uint::arbitrary(),
            Uint::arbitrary(),
        );
        let mut runner = TestRunner::deterministic();
        criterion.bench_function(&format!("mul_mod/{}", BITS), move |bencher| {
            bencher.iter_batched(
                || input.new_tree(&mut runner).unwrap().current(),
                |(a, b, m)| black_box(black_box(a).mul_mod(black_box(b), black_box(m))),
                BatchSize::SmallInput,
            );
        });
    }

    fn bench_pow<const BITS: usize, const LIMBS: usize>(criterion: &mut Criterion) {
        let input = (
            Uint::<BITS, LIMBS>::arbitrary(),
            Uint::arbitrary(),
            Uint::arbitrary(),
        );
        let mut runner = TestRunner::deterministic();
        criterion.bench_function(&format!("pow_mod/{}", BITS), move |bencher| {
            bencher.iter_batched(
                || input.new_tree(&mut runner).unwrap().current(),
                |(a, b, m)| black_box(black_box(a).pow_mod(black_box(b), black_box(m))),
                BatchSize::SmallInput,
            );
        });
    }

    fn bench_inv<const BITS: usize, const LIMBS: usize>(criterion: &mut Criterion) {
        let input = (Uint::<BITS, LIMBS>::arbitrary(), Uint::arbitrary());
        let mut runner = TestRunner::deterministic();
        criterion.bench_function(&format!("inv_mod/{}", BITS), move |bencher| {
            bencher.iter_batched(
                || input.new_tree(&mut runner).unwrap().current(),
                |(a, m)| black_box(black_box(a).inv_mod(black_box(m))),
                BatchSize::SmallInput,
            );
        });
    }
}
