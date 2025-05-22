use crate::prelude::*;

pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);
        bench_unop::<BITS, LIMBS, _>(criterion, "neg", |a| -a);
        bench_binop::<BITS, LIMBS, _>(criterion, "add", |a, b| a + b);
        bench_binop::<BITS, LIMBS, _>(criterion, "sub", |a, b| a - b);
    });
}
