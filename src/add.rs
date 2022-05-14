use crate::{nlimbs, Uint};
use core::arch::asm;

pub trait OverflowingAdd: Sized {
    fn overflowing_add(self, other: Self) -> (Self, bool);
}

#[cfg(not(target_arch = "aarch64"))]
impl<const BITS: usize> OverflowingAdd for Uint<BITS> {
    #[inline(never)]
    #[must_use]
    default fn overflowing_add(self, other: Self) -> (Self, bool) {
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
    default fn overflowing_add(self, other: Self) -> (Self, bool) {
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
            for i in 1..nlimbs(BITS) {
                asm!(
                    "adcs {}, {}, {}",
                    in(reg) self.limbs[i],
                    in(reg) other.limbs[i],
                    out(reg) limbs[i],
                    options(pure, nomem, nostack),
                )
            }
            let mut carry = 0_u64;
            asm!(
                "cset {}, cs",
                out(reg) carry,
                options(pure, nomem, nostack),
            );
            (Self { limbs }, carry != 0)
        }
    }
}

impl OverflowingAdd for Uint<64> {
    #[inline(never)]
    #[must_use]
    fn overflowing_add(self, other: Self) -> (Self, bool) {
        let (limb, carry) = self.limbs[0].overflowing_add(other.limbs[0]);
        (Self { limbs: [limb] }, carry)
    }
}

pub fn test() {
    let val = Uint::<256>::one();
    val.overflowing_add(Uint::<256>::one());
    dbg!(val);
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! repeat {
        ( $x:block ) => {
            repeat!($x, 1, 2, 63, 64, 65, 127,128,129,256,384,512,4096);
        };
        ( $x:block, $( $n:literal ),* ) => {
            $({
                const N: usize = $n;
                dbg!(N);
                $x
            })*
        };
    }

    #[test]
    fn construct_zeros() {
        let _ = Uint::<0>::zero();
        repeat!({
            let _ = Uint::<N>::zero();
        });
    }

    #[test]
    fn construct_ones() {
        repeat!({
            let _ = Uint::<N>::one();
        });
    }
}

#[cfg(feature = "bench")]
pub mod bench {
    use super::*;
    use ::proptest::{
        strategy::{Strategy, ValueTree},
        test_runner::TestRunner,
    };
    use criterion::{black_box, BatchSize, Criterion};

    pub fn group(criterion: &mut Criterion) {
        bench_add::<64>(criterion);
        bench_add::<256>(criterion);
        bench_add::<384>(criterion);
        bench_add::<512>(criterion);
        bench_add::<4096>(criterion);
    }

    fn bench_add<const BITS: usize>(criterion: &mut Criterion)
    where
        [(); nlimbs(BITS)]:,
    {
        let input = (Uint::<BITS>::arb(), Uint::arb());
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
