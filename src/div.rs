use crate::{algorithms::div::divrem, impl_bin_op, Uint};
use core::ops::{Div, DivAssign, Rem, RemAssign};

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Computes `self / rhs`, returning [`None`] if `rhs == 0`.
    #[must_use]
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs == Self::ZERO {
            return None;
        }
        todo!() // Some(self / rhs)
    }
    /// Computes `self % rhs`, returning [`None`] if `rhs == 0`.
    #[must_use]
    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        if rhs == Self::ZERO {
            return None;
        }
        todo!() // Some(self % rhs)
    }

    #[must_use]
    pub fn div_ceil(self, rhs: Self) -> Self {
        assert!(rhs != Self::ZERO);
        let (q, r) = self.div_rem(rhs);
        if r != Self::ZERO {
            q + Self::from(1)
        } else {
            q
        }
    }

    #[must_use]
    pub fn div_rem(self, rhs: Self) -> (Self, Self) {
        assert!(rhs != Self::ZERO, "Division by zero");
        let mut result = vec![0_u64; LIMBS + 1];
        let mut divisor = [0_u64; LIMBS];
        result[..LIMBS].copy_from_slice(&self.limbs);
        divisor[..LIMBS].copy_from_slice(&rhs.limbs);
        divrem(&mut result, &mut divisor);
        (
            Self::from_limbs_slice(&result[..LIMBS]),
            Self::from_limbs(divisor),
        )
    }

    #[must_use]
    pub fn div(self, rhs: Self) -> Self {
        self.div_rem(rhs).0
    }

    #[must_use]
    pub fn rem(self, rhs: Self) -> Self {
        self.div_rem(rhs).1
    }
}

impl_bin_op!(Div, div, DivAssign, div_assign, div);
impl_bin_op!(Rem, rem, RemAssign, rem_assign, rem);

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{const_for, nlimbs};
    use proptest::{prop_assume, proptest};

    #[test]
    fn test_divceil() {
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
            bench_div_rem::<BITS, LIMBS>(criterion);
        });
    }

    fn bench_div_rem_small<const BITS: usize, const LIMBS: usize>(criterion: &mut Criterion) {
        if BITS == 0 {
            return;
        }
        let input = (Uint::<BITS, LIMBS>::arbitrary(), u64::arbitrary());
        let mut runner = TestRunner::deterministic();
        criterion.bench_function(&format!("div_rem_64/{}", BITS), move |bencher| {
            bencher.iter_batched(
                || {
                    let (n, mut d) = input.new_tree(&mut runner).unwrap().current();
                    if d == 0 {
                        d = 1;
                    }
                    if BITS < 64 {
                        d &= Uint::<BITS, LIMBS>::MASK;
                    }
                    (n, Uint::from(d))
                },
                |(a, b)| black_box(black_box(a).div_rem(black_box(b))),
                BatchSize::SmallInput,
            );
        });
    }

    fn bench_div_rem<const BITS: usize, const LIMBS: usize>(criterion: &mut Criterion) {
        if BITS == 0 {
            return;
        }
        let input = (Uint::<BITS, LIMBS>::arbitrary(), Uint::arbitrary());
        let mut runner = TestRunner::deterministic();
        criterion.bench_function(&format!("div_rem/{}", BITS), move |bencher| {
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
