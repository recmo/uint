use crate::prelude::*;

pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);
        bench_binop::<BITS, LIMBS, _>(criterion, "pow", |a, b| a.pow(b));
        bench_binop::<BITS, LIMBS, _>(criterion, "overflowing_pow", |a, b| a.overflowing_pow(b));
    });
}
