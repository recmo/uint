use crate::{nlimbs, Uint};
use core::arch::asm;
use itertools::izip;

#[allow(clippy::module_name_repetitions)]
pub trait OverflowingAdd: Sized {
    fn overflowing_add(self, other: Self) -> (Self, bool);
}

#[cfg(not(target_arch = "aarch64"))]
impl<const BITS: usize> OverflowingAdd for Uint<BITS>
where
    [(); nlimbs(BITS)]:,
{
    #[inline(never)]
    #[must_use]
    fn overflowing_add(self, other: Self) -> (Self, bool) {
        let mut result = Self::zero();
        let mut carry = 0;
        for (res, lhs, rhs) in izip!(
            result.limbs.iter_mut(),
            self.limbs.into_iter(),
            other.limbs.into_iter()
        ) {
            let sum = (lhs as u128) + (rhs as u128) + (carry as u128);
            *res = sum as u64;
            carry = (sum >> 64) as u64;
        }
        (result, carry != 0)
    }
}

#[cfg(target_arch = "aarch64")]
impl<const BITS: usize> OverflowingAdd for Uint<BITS>
where
    [(); nlimbs(BITS)]:,
{
    #[inline(never)]
    #[must_use]
    fn overflowing_add(self, other: Self) -> (Self, bool) {
        if BITS == 0 {
            return (self, false);
        }
        unsafe {
            let mut limbs = [0; nlimbs(BITS)];
            asm!(
                "adds {}, {}, {}",
                in(reg) self.limbs[0],
                in(reg) other.limbs[0],
                out(reg) limbs[0],
                options(pure, nomem, nostack),
            );
            for (res, lhs, rhs) in izip!(
                limbs.iter_mut(),
                self.limbs.into_iter(),
                other.limbs.into_iter()
            ) {
                asm!(
                    "adcs {}, {}, {}",
                    in(reg) lhs,
                    in(reg) rhs,
                    out(reg) *res,
                    options(pure, nomem, nostack),
                );
            }
            let mut carry: u64;
            asm!(
                "cset {}, cs",
                out(reg) carry,
                options(pure, nomem, nostack),
            );
            (Self { limbs }, carry != 0)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::repeat;

    #[test]
    fn construct_zeros() {
        let _ = Uint::<0>::zero();
        repeat!({
            let _ = Uint::<N>::zero();
        });
    }

    #[test]
    fn construct_ones() {
        repeat!(
            {
                let _ = Uint::<N>::one();
            },
            1,
            2,
            64,
            128
        );
    }
}

#[cfg(feature = "bench")]
pub mod bench {
    use super::*;
    use crate::repeat;
    use ::proptest::{
        arbitrary::Arbitrary,
        strategy::{Strategy, ValueTree},
        test_runner::TestRunner,
    };
    use criterion::{black_box, BatchSize, Criterion};

    pub fn group(criterion: &mut Criterion) {
        repeat!(
            {
                bench_add::<N>(criterion);
            },
            64,
            256,
            384,
            512,
            4096
        );
    }

    fn bench_add<const BITS: usize>(criterion: &mut Criterion)
    where
        [(); nlimbs(BITS)]:,
    {
        let input = (Uint::<BITS>::arbitrary(), Uint::arbitrary());
        let mut runner = TestRunner::deterministic();
        criterion.bench_function(&format!("uint_add_{}", BITS), move |bencher| {
            bencher.iter_batched(
                || input.new_tree(&mut runner).unwrap().current(),
                |(a, b)| black_box(black_box(a).overflowing_add(black_box(b))),
                BatchSize::SmallInput,
            );
        });
    }
}
