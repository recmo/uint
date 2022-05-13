mod proptest;

use core::arch::asm;
// use itertools::izip;

#[cfg(feature = "proptest")]
pub use self::proptest::arb_uint;

/// Number of `u64` limbs required to represent the given number of bits.
pub const fn num_limbs(bits: usize) -> usize {
    (bits + 63) / 64
}

/// Binary numbers modulo $2^n$.
#[derive(Clone, Copy, Debug)]
pub struct Uint<const BITS: usize>
where
    [(); num_limbs(BITS)]:,
{
    limbs: [u64; num_limbs(BITS)],
}

trait OverflowingAdd: Sized {
    fn overflowing_add(self, other: Self) -> (Self, bool);
}

impl<const BITS: usize> Uint<BITS>
where
    [(); num_limbs(BITS)]:,
{
    pub const BITS: usize = BITS;

    #[must_use]
    pub const fn zero() -> Self {
        Self::from_limbs([0; num_limbs(BITS)])
    }

    #[must_use]
    pub const fn one() -> Self {
        let mut result = Self::zero();
        result.limbs[0] = 1;
        result
    }

    #[must_use]
    pub const fn from_limbs(limbs: [u64; num_limbs(BITS)]) -> Self {
        Self { limbs }
    }

    #[must_use]
    pub fn from_limbs_slice(slice: &[u64]) -> Self {
        let mut limbs = [0; num_limbs(BITS)];
        limbs.copy_from_slice(slice);
        Self { limbs }
    }
}

#[cfg(not(target_arch = "aarch64"))]
impl<const BITS: usize> OverflowingAdd for Uint<BITS>
where
    [(); num_limbs(BITS)]:,
{
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
    [(); num_limbs(BITS)]:,
{
    #[inline(never)]
    #[must_use]
    default fn overflowing_add(self, other: Self) -> (Self, bool) {
        if BITS == 0 {
            return (self, false);
        }
        unsafe {
            let mut limbs = [0; num_limbs(BITS)];
            asm!(
                "adds {}, {}, {}",
                in(reg) self.limbs[0],
                in(reg) other.limbs[0],
                out(reg) limbs[0],
                options(pure, nomem, nostack),
            );
            for i in 1..num_limbs(BITS) {
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
        [(); num_limbs(BITS)]:,
    {
        let input = (arb_uint::<BITS>(), arb_uint());
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
