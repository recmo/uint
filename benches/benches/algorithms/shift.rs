use crate::prelude::*;
use ruint::algorithms::{shift_left_small, shift_right_small};

pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);
        let name = |n: &str| format!("{n}/{BITS}");

        bench_arbitrary_with(
            criterion,
            &name("shift_right_small"),
            (Uint::<BITS, LIMBS>::arbitrary(), 1..64usize),
            |(a, amount)| {
                let mut limbs = a.into_limbs();
                shift_right_small(limbs.as_mut(), amount)
            },
        );

        bench_arbitrary_with(
            criterion,
            &name("shift_left_small"),
            (Uint::<BITS, LIMBS>::arbitrary(), 1..64usize),
            |(a, amount)| {
                let mut limbs = a.into_limbs();
                shift_left_small(limbs.as_mut(), amount)
            },
        );
    });
}
