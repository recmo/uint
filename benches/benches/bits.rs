use crate::prelude::*;

pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);
        bench_arbitrary_with(criterion, "overflowing_shl", (Uint::<BITS, LIMBS>::arbitrary(), 0..=BITS), move |(a, b)| {
            a.overflowing_shl(b)
        });
    });
}
