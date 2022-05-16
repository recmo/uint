use crate::{Uint};
use itertools::izip;

#[allow(clippy::module_name_repetitions)]
pub trait OverflowingAdd: Sized {
    fn overflowing_add(self, other: Self) -> (Self, bool);
}

impl<const BITS: usize, const LIMBS: usize> OverflowingAdd for Uint<BITS, LIMBS> {
    #[must_use]
    #[allow(clippy::cast_lossless)]
    #[allow(clippy::cast_possible_truncation)]
    fn overflowing_add(self, other: Self) -> (Self, bool) {
        let mut result = Self::MIN;
        let mut carry = 0;
        for (res, &lhs, &rhs) in izip!(result.as_limbs_mut(), self.as_limbs(), other.as_limbs()) {
            let sum = (lhs as u128) + (rhs as u128) + (carry as u128);
            *res = sum as u64;
            carry = (sum >> 64) as u64;
        }
        (result, carry != 0)
    }
}

// #[cfg(target_arch = "aarch64")]
// impl<const BITS: usize> OverflowingAdd for Uint<BITS>
// where
//     [(); nlimbs(BITS)]:,
// {
//     #[inline(never)]
//     #[must_use]
//     fn overflowing_add(self, other: Self) -> (Self, bool) {
//         if BITS == 0 {
//             return (self, false);
//         }
//         unsafe {
//             let mut limbs = [0; nlimbs(BITS)];
//             asm!(
//                 "adds {}, {}, {}",
//                 in(reg) self.limbs[0],
//                 in(reg) other.limbs[0],
//                 out(reg) limbs[0],
//                 options(pure, nomem, nostack),
//             );
//             for (res, lhs, rhs) in izip!(
//                 limbs.iter_mut(),
//                 self.limbs.into_iter(),
//                 other.limbs.into_iter()
//             ) {
//                 asm!(
//                     "adcs {}, {}, {}",
//                     in(reg) lhs,
//                     in(reg) rhs,
//                     out(reg) *res,
//                     options(pure, nomem, nostack),
//                 );
//             }
//             let mut carry: u64;
//             asm!(
//                 "cset {}, cs",
//                 out(reg) carry,
//                 options(pure, nomem, nostack),
//             );
//             (Self { limbs }, carry != 0)
//         }
//     }
// }

#[cfg(feature = "bench")]
pub mod bench {
    use super::*;
    use crate::const_for;
    use ::proptest::{
        arbitrary::Arbitrary,
        strategy::{Strategy, ValueTree},
        test_runner::TestRunner,
    };
    use criterion::{black_box, BatchSize, Criterion};

    pub fn group(criterion: &mut Criterion) {
        const_for!(BITS in [64, 256, 384, 512, 4096] {
            bench_add::<BITS>(criterion);
        });
    }

    fn bench_add<const BITS: usize>(criterion: &mut Criterion) {
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
