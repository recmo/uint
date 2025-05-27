use crate::prelude::*;

pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);
        bench_shifting_binop::<BITS, LIMBS, _>(criterion, "overflowing_shl", |a, b| a.overflowing_shl(b));
    });
}
