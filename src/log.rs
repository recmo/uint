use crate::Uint;

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    #[must_use]
    pub fn checked_log(self, base: u64) -> Option<u64> {
        if base < 2 || self == Self::ZERO {
            return None;
        }
        Some(self.log(base))
    }

    #[must_use]
    pub fn checked_log10(self) -> Option<u64> {
        self.checked_log(10)
    }

    /// Returns the base 2 logarithm of the number, rounded down.
    ///
    /// This is equivalent to the index of the highest set bit.
    ///
    /// Returns None if the number is zero.
    #[must_use]
    pub fn checked_log2(self) -> Option<u64> {
        self.checked_log(2)
    }

    /// # Panics
    ///
    /// Panics if the `base` is less than 2 or if the number is zero.
    #[must_use]
    pub fn log(self, base: u64) -> u64 {
        assert!(base >= 2);
        assert!(self != Self::ZERO);
        if base == 2 {
            return self.bit_len() as u64 - 1;
        }
        if self < Self::from(base) {
            return 0;
        }

        // Find approximate result
        // f64 can hold integer values up to 2^53 exactly. With the smallest
        // possible base (2) `self` would have to be more than a petabyte long
        // to get into the non-exact integer domain.
        #[allow(clippy::cast_precision_loss)]
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        let mut result = {
            // Ideally we'd use f64::from(self), but that quickly overflows.
            // So instead we take the highest bits and use
            // log_base(bits * 2^exp) = (log_2(bits) + exp) / log_2(base)
            let (bits, exp) = self.most_significant_bits();
            // Convert to floats
            let bits = bits as f64;
            let exp = exp as f64;
            let base = base as f64;
            let result = (bits.log2() + exp) / base.log2();
            assert!(result.is_finite());
            assert!(result > 0.0);
            result.trunc() as u64
        };

        // Adjust result to get the exact value. At most one of these should happen, but
        // we loop regardless.
        loop {
            if let Some(value) = Self::from(base).checked_pow(Self::from(result)) {
                if value > self {
                    assert!(result >= 1);
                    result -= 1;
                    continue;
                }
            }
            break;
        }
        loop {
            if let Some(value) = Self::from(base).checked_pow(Self::from(result + 1)) {
                if value <= self {
                    assert!(result < u64::MAX);
                    result += 1;
                    continue;
                }
            }
            break;
        }

        result
    }

    #[must_use]
    pub fn log10(self) -> u64 {
        self.log(10)
    }

    #[must_use]
    pub fn log2(self) -> u64 {
        self.log(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{aliases::U128, const_for, nlimbs};
    use proptest::{proptest, prop_assume};

    #[test]
    fn test_checked_log2() {
        assert_eq!(U128::from(0).checked_log2(), None);
        assert_eq!(U128::from(1).checked_log2(), Some(0));
        assert_eq!(U128::from(2).checked_log2(), Some(1));
        assert_eq!(U128::from(3).checked_log2(), Some(1));
        assert_eq!(U128::from(127).checked_log2(), Some(6));
        assert_eq!(U128::from(128).checked_log2(), Some(7));
    }

    #[test]
    fn test_pow_log() {
        const_for!(BITS in NON_ZERO if (BITS >= 64) {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(b in 2_u64..100, e in 0..BITS)| {
                if let Some(value) = U::from(b).checked_pow(U::from(e)) {
                    assert!(value > U::ZERO);
                    assert_eq!(value.log(b), e as u64);
                    // assert_eq!(value.log(b + U::from(1)), e as u64);
                }
            });
        });
    }

    #[test]
    fn test_log_pow() {
        const_for!(BITS in NON_ZERO if (BITS >= 64) {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(b in 2_u64..100, n: U)| {
                prop_assume!(n > U::ZERO);
                let e = n.log(b);
                assert!(U::from(b).pow(U::from(e)) <= n);
                if let Some(value) = U::from(b).checked_pow(U::from(e + 1)) {
                    assert!(value > n);
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
            bench_log::<BITS, LIMBS>(criterion);
        });
    }

    fn bench_log<const BITS: usize, const LIMBS: usize>(criterion: &mut Criterion) {
        let input = (Uint::<BITS, LIMBS>::arbitrary(), 2_u64..100);
        let mut runner = TestRunner::deterministic();
        criterion.bench_function(&format!("log/{}", BITS), move |bencher| {
            bencher.iter_batched(
                || input.new_tree(&mut runner).unwrap().current(),
                |(n, b)| black_box(black_box(n).checked_log(b)),
                BatchSize::SmallInput,
            );
        });
    }
}
