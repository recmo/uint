use crate::Uint;

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    #[must_use]
    pub fn checked_pow(self, exp: Self) -> Option<Self> {
        match self.overflowing_pow(exp) {
            (x, false) => Some(x),
            (_, true) => None,
        }
    }

    /// # Examples
    ///
    /// ```
    /// # use ruint::{Uint, uint};
    /// # uint!{
    /// //assert_eq!(0_U0.overflowing_pow(0_U0), (0_U0, false));
    /// //assert_eq!(0_U1.overflowing_pow(0_U1), (0_U1, false));
    /// //assert_eq!(0_U1.overflowing_pow(1_U1), (0_U1, false));
    /// //assert_eq!(1_U1.overflowing_pow(0_U1), (1_U1, false));
    /// assert_eq!(1_U1.overflowing_pow(1_U1), (1_U1, false));
    /// # }
    /// ```
    ///
    /// ```
    /// # use ruint::{Uint, uint};
    /// # uint!{
    /// assert_eq!(36_U64.overflowing_pow(12_U64), (0x41c21cb8e1000000_U64, false));
    /// assert_eq!(36_U64.overflowing_pow(13_U64), (0x3f4c09ffa4000000_U64, true));
    /// assert_eq!(36_U68.overflowing_pow(13_U68), (0x093f4c09ffa4000000_U68, false));
    /// assert_eq!(16_U65.overflowing_pow(32_U65), (0_U65, true));
    /// # }
    /// ```
    #[must_use]
    pub fn overflowing_pow(mut self, exp: Self) -> (Self, bool) {
        if BITS == 0 {
            return (self, false);
        }

        // Exponentiation by squaring
        let mut overflow = false;
        let mut base_overflow = false;
        let mut result = Self::from(1);
        for mut limb in exp.limbs {
            for _ in 0..64 {
                // Multiply by base
                if limb & 1 == 1 {
                    let (r, o) = result.overflowing_mul(self);
                    result = r;
                    overflow |= o | base_overflow;
                }

                // Square base
                let (s, o) = self.overflowing_mul(self);
                self = s;
                base_overflow |= o;
                limb >>= 1;
            }
        }
        (result, overflow)
    }

    /// # The binary [Carmichael functions][cf]
    ///
    /// [cf]: https://en.wikipedia.org/wiki/Carmichael_function
    ///
    /// $$
    /// λ(2^\mathtt{BITS}) = \begin{cases}
    ///   2^{\mathtt{BITS} - 1} & \mathtt{BITS} ≤ 3
    ///   2^{\mathtt{BITS} - 2} & \mathtt{BITS} > 3
    /// \end{cases}
    /// $$
    #[must_use]
    pub fn pow(self, exp: Self) -> Self {
        self.wrapping_pow(exp)
    }

    #[must_use]
    pub fn saturating_pow(self, exp: Self) -> Self {
        match self.overflowing_pow(exp) {
            (x, false) => x,
            (_, true) => Self::MAX,
        }
    }

    #[must_use]
    pub fn wrapping_pow(self, exp: Self) -> Self {
        self.overflowing_pow(exp).0
    }
}

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    /// Returns `true` if and only if `self == 2^k` for some `k`.
    #[must_use]
    pub fn is_power_of_two(self) -> bool {
        self.count_ones() == 1
    }

    #[must_use]
    pub fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
        todo!()
    }

    #[must_use]
    pub fn checked_next_power_of_two(self) -> Option<Self> {
        todo!()
    }

    #[must_use]
    pub fn next_multiple_of(self, rhs: Self) -> Self {
        todo!()
    }

    #[must_use]
    pub fn next_power_of_two(self) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{const_for, nlimbs};
    use proptest::proptest;
    use std::iter::repeat;

    #[test]
    fn test_pow2_shl() {
        const_for!(BITS in NON_ZERO if (BITS >= 2) {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(e in 0..BITS+2)| {
                assert_eq!(U::from(2).pow(U::from(e)), U::from(1) << e);
            });
        });
    }

    #[test]
    fn test_pow_product() {
        const_for!(BITS in NON_ZERO if (BITS >= 64) {
            const LIMBS: usize = nlimbs(BITS);
            type U = Uint<BITS, LIMBS>;
            proptest!(|(b in 2_u64..100, e in 0_usize..100)| {
                let b = U::from(b);
                let prod = repeat(b).take(e).product();
                assert_eq!(b.pow(U::from(e)), prod);
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
            bench_pow::<BITS, LIMBS>(criterion);
        });
    }

    fn bench_pow<const BITS: usize, const LIMBS: usize>(criterion: &mut Criterion) {
        let input = (Uint::<BITS, LIMBS>::arbitrary(), Uint::arbitrary());
        let mut runner = TestRunner::deterministic();
        criterion.bench_function(&format!("pow/{}", BITS), move |bencher| {
            bencher.iter_batched(
                || input.new_tree(&mut runner).unwrap().current(),
                |(b, e)| black_box(black_box(b).pow(black_box(e))),
                BatchSize::SmallInput,
            );
        });
    }
}
