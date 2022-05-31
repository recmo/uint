use crate::Uint;

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Computes the floor of the `degree`-th root of the number.
    ///
    /// $$
    /// \floor{\sqrt[\mathtt degree]{\mathtt{self}}}
    /// $$
    ///
    /// # Panics
    ///
    /// Panics if `degree` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruint::{Uint, uint, aliases::*};
    /// # uint!{
    /// // assert_eq!(0_U64.root(2), 0_U64);
    /// // assert_eq!(1_U64.root(63), 0_U64);
    /// # }
    /// ```
    #[must_use]
    pub fn root(self, degree: usize) -> Self {
        assert!(degree > 0);

        // Handle case where `index > Self::MAX`.
        if BITS == 0 || (BITS <= 64 && degree as u64 > Self::MAX.limbs[0]) {
            return if self == Self::ZERO {
                Self::ZERO
            } else {
                Self::from(1)
            };
        }

        // Create a first guess.
        // Root should be less than the value, so approx_pow2 should always succeed.
        #[allow(clippy::cast_precision_loss)] // Approximation is good enough.
        #[allow(clippy::cast_sign_loss)] // Result should be positive.
        let mut result = Self::approx_pow2(self.approx_log2() / degree as f64).unwrap();

        // Iterate using Newton's method
        // See <https://en.wikipedia.org/wiki/Integer_square_root#Algorithm_using_Newton's_method>
        // See <https://gmplib.org/manual/Nth-Root-Algorithm>
        let mut first = true;
        loop {
            if result == Self::ZERO {
                return result;
            }
            // OPT: This could benefit from single-limb multiplication and division.
            // OPT: The division can be turned into bit-shifts when the degree is a power
            // of two.
            let iter = (self / result.pow(degree - 1) + Self::from(degree - 1) * result)
                / Self::from(degree);
            debug_assert!(iter != Self::ZERO);
            if !first && iter >= result {
                break result;
            }
            first = false;
            result = iter;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{const_for, nlimbs};
    use proptest::proptest;

    #[test]
    fn test_root() {
        const_for!(BITS in SIZES if (BITS > 3) {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(value: U, degree in 1_usize..=5)| {
                let root = value.root(degree);
                let lower = root.pow(degree);
                assert!(value >= lower);
                let upper = root
                    .checked_add(U::from(1))
                    .and_then(|n| n.checked_pow(degree));
                if let Some(upper) = upper {
                   assert!(value < upper);
                }
            });
        });
    }

    #[test]
    fn test_root_large() {
        const_for!(BITS in SIZES if (BITS > 3) {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(value: U, degree in 1_usize..)| {
                let root = value.root(degree);
                let lower = root.pow(degree);
                assert!(value >= lower);
                let upper = root
                    .checked_add(U::from(1))
                    .and_then(|n| n.checked_pow(degree));
                if let Some(upper) = upper {
                   assert!(value < upper);
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
            bench_root::<BITS, LIMBS>(criterion, 2);
            bench_root::<BITS, LIMBS>(criterion, 3);
            bench_root::<BITS, LIMBS>(criterion, 5);
            bench_root::<BITS, LIMBS>(criterion, 1_073_741_824);
        });
    }

    fn bench_root<const BITS: usize, const LIMBS: usize>(criterion: &mut Criterion, degree: usize) {
        let input = Uint::<BITS, LIMBS>::arbitrary();
        let mut runner = TestRunner::deterministic();
        criterion.bench_function(&format!("root/{}/{}", degree, BITS), move |bencher| {
            bencher.iter_batched(
                || input.new_tree(&mut runner).unwrap().current(),
                |value| black_box(black_box(value).root(black_box(degree))),
                BatchSize::SmallInput,
            );
        });
    }
}
