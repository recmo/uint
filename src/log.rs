use crate::Uint;

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    #[must_use]
    pub fn checked_log(self, base: u64) -> Option<usize> {
        if base < 2 || self == Self::ZERO {
            return None;
        }
        Some(self.log(base))
    }

    #[must_use]
    pub fn checked_log10(self) -> Option<usize> {
        self.checked_log(10)
    }

    /// Returns the base 2 logarithm of the number, rounded down.
    ///
    /// This is equivalent to the index of the highest set bit.
    ///
    /// Returns None if the number is zero.
    #[must_use]
    pub fn checked_log2(self) -> Option<usize> {
        self.checked_log(2)
    }

    /// # Panics
    ///
    /// Panics if the `base` is less than 2 or if the number is zero.
    #[must_use]
    pub fn log(self, base: u64) -> usize {
        assert!(base >= 2);
        assert!(self != Self::ZERO);
        if base == 2 {
            return self.bit_len() - 1;
        }
        if self < Self::from(base) {
            return 0;
        }

        // Find approximate result
        #[allow(clippy::cast_precision_loss)] // Approximate is good enough.
        #[allow(clippy::cast_possible_truncation)] // Approximate is good enough.
        #[allow(clippy::cast_sign_loss)] // Negative results cast to zeros. (TODO: Do they?)
        let mut result = self.approx_log(base as f64) as usize;

        // Adjust result to get the exact value. At most one of these should happen, but
        // we loop regardless.
        loop {
            if let Some(value) = Self::from(base).checked_pow(result) {
                if value > self {
                    assert!(result >= 1);
                    result -= 1;
                    continue;
                }
            }
            break;
        }
        loop {
            if let Some(value) = Self::from(base).checked_pow(result + 1) {
                if value <= self {
                    assert!(result < usize::MAX);
                    result += 1;
                    continue;
                }
            }
            break;
        }

        result
    }

    #[must_use]
    pub fn log10(self) -> usize {
        self.log(10)
    }

    #[must_use]
    pub fn log2(self) -> usize {
        self.log(2)
    }

    /// Double precision logarithm.
    #[must_use]
    pub fn approx_log(self, base: f64) -> f64 {
        self.approx_log2() / base.log2()
    }

    /// Double precision binary logarithm.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruint::{Uint, uint, aliases::*};
    /// # uint!{
    /// assert_eq!(0_U64.approx_log2(), f64::NEG_INFINITY);
    /// assert_eq!(1_U64.approx_log2(), 0.0);
    /// assert_eq!(2_U64.approx_log2(), 1.0);
    /// assert_eq!(U64::MAX.approx_log2(), 64.0);
    /// # }
    /// ```
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn approx_log2(self) -> f64 {
        // The naive solution would be `f64::from(self).log2()`, but
        // `f64::from(self)` quickly overflows (`f64::MAX` is 2^1024).
        // So instead we first approximate as `bits * 2^exp` and then
        // compute using`log2(bits * 2^exp) = log2(bits) + exp`
        let (bits, exp) = self.most_significant_bits();
        // Convert to floats
        let bits = bits as f64;
        let exp = exp as f64;
        bits.log2() + exp
    }

    /// Double precision decimal logarithm.
    #[must_use]
    pub fn approx_log10(self) -> f64 {
        self.approx_log2() / core::f64::consts::LOG2_10
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{aliases::U128, const_for, nlimbs};
    use proptest::{prop_assume, proptest};

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
    fn test_approx_log2_pow2() {
        const_for!(BITS in SIZES {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(value: U)| {
                let log = value.approx_log2();
                let pow = U::approx_pow2(log).unwrap();
                let error = value.abs_diff(pow);
                let correct_bits = value.bit_len() - error.bit_len();
                // The maximum precision we could expect here is 53 bits.
                // OPT: Find out exactly where the precision is lost and what
                // the bounds are.
                assert!(correct_bits == value.bit_len() || correct_bits >= 42);
            });
        });
    }

    #[test]
    fn test_pow_log() {
        const_for!(BITS in NON_ZERO if (BITS >= 64) {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(b in 2_u64..100, e in 0..BITS)| {
                if let Some(value) = U::from(b).checked_pow(e) {
                    assert!(value > U::ZERO);
                    assert_eq!(value.log(b), e);
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
                assert!(U::from(b).pow(e) <= n);
                if let Some(value) = U::from(b).checked_pow(e + 1) {
                    assert!(value > n);
                }
            });
        });
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
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
