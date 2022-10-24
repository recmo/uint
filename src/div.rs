use crate::{algorithms, impl_bin_op, Uint};
use core::ops::{Div, DivAssign, Rem, RemAssign};

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Computes `self / rhs`, returning [`None`] if `rhs == 0`.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // False positive
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs == Self::ZERO {
            return None;
        }
        Some(self.div(rhs))
    }

    /// Computes `self % rhs`, returning [`None`] if `rhs == 0`.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // False positive
    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        if rhs == Self::ZERO {
            return None;
        }
        Some(self.rem(rhs))
    }

    /// Computes `self / rhs` rounding up.
    ///
    /// # Panics
    ///
    /// Panics if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub fn div_ceil(self, rhs: Self) -> Self {
        assert!(rhs != Self::ZERO, "Division by zero");
        let (q, r) = self.div_rem(rhs);
        if r == Self::ZERO {
            q
        } else {
            q + Self::from(1)
        }
    }

    /// Computes `self / rhs` and `self % rhs`.
    ///
    /// # Panics
    ///
    /// Panics if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub fn div_rem(mut self, mut rhs: Self) -> (Self, Self) {
        assert!(rhs != Self::ZERO, "Division by zero");
        algorithms::div(&mut self.limbs, &mut rhs.limbs);
        (self, rhs)
    }

    /// Computes `self / rhs` rounding down.
    ///
    /// # Panics
    ///
    /// Panics if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub fn wrapping_div(self, rhs: Self) -> Self {
        self.div_rem(rhs).0
    }

    /// Computes `self % rhs`.
    ///
    /// # Panics
    ///
    /// Panics if `rhs == 0`.
    #[must_use]
    #[track_caller]
    pub fn wrapping_rem(self, rhs: Self) -> Self {
        self.div_rem(rhs).1
    }
}

impl_bin_op!(Div, div, DivAssign, div_assign, wrapping_div);
impl_bin_op!(Rem, rem, RemAssign, rem_assign, wrapping_rem);

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{const_for, nlimbs};
    use proptest::{prop_assume, proptest};

    #[test]
    fn test_div_ceil() {
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(n: U, mut d: U)| {
                d >>= BITS / 2; // make d small
                prop_assume!(d != U::ZERO);
                let qf = n / d;
                let qc = n.div_ceil(d);
                assert!(qf <= qc);
                assert!(qf == qc || qf == qc - U::from(1));
                if qf == qc {
                    assert!(n % d == U::ZERO);
                }
            });
        });
    }

    #[test]
    fn test_divrem() {
        const_for!(BITS in NON_ZERO {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(n: U, mut d: u64)| {
                if BITS < 64 {
                    d &= U::MASK;
                }
                if d == 0 {
                    d = 1;
                }
                let d = U::from(d);
                let (q, r) = n.div_rem(d);
                assert!(r < d);
                assert_eq!(q * d + r, n);
            });
            proptest!(|(n: U, mut d: U)| {
                d >>= BITS / 2; // make d small
                prop_assume!(d != U::ZERO);
                let (q, r) = n.div_rem(d);
                assert!(r < d);
                assert_eq!(q * d + r, n);
            });
            proptest!(|(n: U, d: U)| {
                prop_assume!(d != U::ZERO);
                let (q, r) = n.div_rem(d);
                assert!(r < d);
                assert_eq!(q * d + r, n);
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
            bench_div_rem_small::<BITS, LIMBS>(criterion);
            bench_div_rem_half::<BITS, LIMBS>(criterion);
            bench_div_rem_full::<BITS, LIMBS>(criterion);
        });
    }

    fn bench_div_rem_small<const BITS: usize, const LIMBS: usize>(criterion: &mut Criterion) {
        if BITS == 0 {
            return;
        }
        let input = (Uint::<BITS, LIMBS>::arbitrary(), u64::arbitrary());
        let mut runner = TestRunner::deterministic();
        criterion.bench_function(&format!("div_rem/{BITS}/64"), move |bencher| {
            bencher.iter_batched(
                || {
                    let (n, mut d) = input.new_tree(&mut runner).unwrap().current();
                    if BITS < 64 {
                        d &= Uint::<BITS, LIMBS>::MASK;
                    }
                    if d == 0 {
                        d = 1;
                    }
                    (n, Uint::from(d))
                },
                |(a, b)| black_box(black_box(a).div_rem(black_box(b))),
                BatchSize::SmallInput,
            );
        });
    }

    fn bench_div_rem_half<const BITS: usize, const LIMBS: usize>(criterion: &mut Criterion) {
        if BITS == 0 {
            return;
        }
        let input = (Uint::<BITS, LIMBS>::arbitrary(), Uint::arbitrary());
        let mut runner = TestRunner::deterministic();
        criterion.bench_function(
            &format!("div_rem/{BITS}/{}", BITS - BITS / 2),
            move |bencher| {
                bencher.iter_batched(
                    || {
                        let (n, mut d) = input.new_tree(&mut runner).unwrap().current();
                        d >>= BITS / 2; // make d half size
                        if d == Uint::ZERO {
                            d = Uint::from(1);
                        }
                        (n, d)
                    },
                    |(a, b)| black_box(black_box(a).div_rem(black_box(b))),
                    BatchSize::SmallInput,
                );
            },
        );
    }

    fn bench_div_rem_full<const BITS: usize, const LIMBS: usize>(criterion: &mut Criterion) {
        if BITS == 0 {
            return;
        }
        let input = (Uint::<BITS, LIMBS>::arbitrary(), Uint::arbitrary());
        let mut runner = TestRunner::deterministic();
        criterion.bench_function(&format!("div_rem/{BITS}/{BITS}"), move |bencher| {
            bencher.iter_batched(
                || {
                    let (n, mut d) = input.new_tree(&mut runner).unwrap().current();
                    if d == Uint::ZERO {
                        d = Uint::from(1);
                    }
                    (n, d)
                },
                |(a, b)| black_box(black_box(a).div_rem(black_box(b))),
                BatchSize::SmallInput,
            );
        });
    }
}
