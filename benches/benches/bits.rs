use crate::prelude::*;

pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);
        let name = |n: &str| format!("{n}/{BITS}");
        let input = || (Uint::<BITS, LIMBS>::arbitrary(), 0..=BITS);
        bench_arbitrary_with(criterion, &name("overflowing_shl"), input(), |(a, b)| {
            a.overflowing_shl(b)
        });
        bench_arbitrary_with(criterion, &name("overflowing_shr"), input(), |(a, b)| {
            a.overflowing_shr(b)
        });
        bench_arbitrary_with(criterion, &name("wrapping_shl"), input(), |(a, b)| {
            a.wrapping_shl(b)
        });
        bench_arbitrary_with(criterion, &name("wrapping_shr"), input(), |(a, b)| {
            a.wrapping_shr(b)
        });
        bench_unop::<BITS, LIMBS, _>(criterion, &name("most_significant_bits"), |a| {
            a.most_significant_bits()
        });
    });
}
