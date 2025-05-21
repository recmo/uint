use crate::prelude::*;

pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);
        bench_binop::<BITS, LIMBS, _>(criterion, "mul", |a, b| a * b);
    });

    const_for!(BITS_LHS in [64, 256, 1024] {
        const LIMBS_LHS: usize = nlimbs(BITS_LHS);
        const_for!(BITS_RHS in [64, 256, 1024] {
            const LIMBS_RHS: usize = nlimbs(BITS_RHS);
            const BITS_RES: usize = BITS_LHS + BITS_RHS;
            const LIMBS_RES: usize = nlimbs(BITS_RES);
            bench_widening_mul::<BITS_LHS, LIMBS_LHS, BITS_RHS, LIMBS_RHS, BITS_RES, LIMBS_RES>(criterion);
        });
    });
}

fn bench_widening_mul<
    const BITS_LHS: usize,
    const LIMBS_LHS: usize,
    const BITS_RHS: usize,
    const LIMBS_RHS: usize,
    const BITS_RES: usize,
    const LIMBS_RES: usize,
>(
    criterion: &mut Criterion,
) {
    bench_arbitrary::<(Uint<BITS_LHS, LIMBS_LHS>, Uint<BITS_RHS, LIMBS_RHS>), _>(
        criterion,
        &format!("widening_mul/{BITS_LHS}/{BITS_RHS}"),
        |(a, b)| a.widening_mul::<BITS_RHS, LIMBS_RHS, BITS_RES, LIMBS_RES>(b),
    );
}
