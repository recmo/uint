use crate::prelude::*;

pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);
        const_for!(DEGREE in [2, 3, 5, 127] {
            bench_root::<BITS, LIMBS, DEGREE>(criterion);
        });
    });
}

fn bench_root<const BITS: usize, const LIMBS: usize, const DEGREE: usize>(
    criterion: &mut Criterion,
) {
    bench_unop::<BITS, LIMBS, _>(criterion, &format!("root/{DEGREE}/{BITS}"), |a| {
        a.root(black_box(DEGREE))
    });
}
