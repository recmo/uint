use crate::prelude::*;
use std::collections::BTreeSet;

pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);
        for rhs_bits in BTreeSet::from([64, BITS - BITS / 2, BITS]) {
            bench_div_rem::<BITS, LIMBS>(criterion, rhs_bits);
        }
    });
}

fn bench_div_rem<const BITS: usize, const LIMBS: usize>(
    criterion: &mut Criterion,
    rhs_bits: usize,
) {
    if BITS == 0 {
        return;
    }
    bench_arbitrary_with(
        criterion,
        &format!("div_rem/{BITS}/{rhs_bits}"),
        (
            Uint::<BITS, LIMBS>::arbitrary(),
            Uint::<BITS, LIMBS>::arbitrary().prop_map(|mut x| {
                x >>= BITS - rhs_bits;
                if x.is_zero() {
                    x = Uint::ONE;
                }
                x
            }),
        ),
        |(a, b)| a.div_rem(b),
    );
}
