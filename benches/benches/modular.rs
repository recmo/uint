use crate::prelude::*;

pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);

        bench_binop::<BITS, LIMBS, _>(criterion, "reduce_mod", |a, m| a.reduce_mod(m));
        bench_ternary::<BITS, LIMBS, _>(criterion, "add_mod", |a, b, m| a.add_mod(b, m));
        bench_ternary::<BITS, LIMBS, _>(criterion, "mul_mod", |a, b, m| a.mul_mod(b, m));
        bench_ternary::<BITS, LIMBS, _>(criterion, "pow_mod", |a, b, m| a.pow_mod(b, m));
        bench_binop::<BITS, LIMBS, _>(criterion, "inv_mod", |a, m| a.inv_mod(m));
    });
}
