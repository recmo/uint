use crate::prelude::*;

pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);
        bench_log::<BITS, LIMBS>(criterion);
    });
}

fn bench_log<const BITS: usize, const LIMBS: usize>(criterion: &mut Criterion) {
    if BITS < 7 {
        return;
    }
    bench_arbitrary_with(
        criterion,
        &format!("log/{BITS}"),
        (
            Uint::<BITS, LIMBS>::arbitrary(),
            (2_u64..100).prop_map(Uint::<BITS, LIMBS>::from),
        ),
        |(n, b)| n.checked_log(b),
    );
}
