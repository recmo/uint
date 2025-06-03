use crate::prelude::*;
use ruint::aliases::U64;

pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);

        // Compare pow_mod vs pow_mod_redc performance
        let name = format!("modexp_comparison/{BITS}");
        let mut group = criterion.benchmark_group(&name);

        // Regular pow_mod
        group.bench_function("pow_mod", |bencher| {
            bencher.iter_batched(
                || {
                    let runner = &mut TestRunner::deterministic();
                    let base = Uint::<BITS, LIMBS>::arbitrary().new_tree(runner).unwrap().current();
                    let exp = Uint::<BITS, LIMBS>::arbitrary().new_tree(runner).unwrap().current();
                    let mut modulus = Uint::<BITS, LIMBS>::arbitrary().new_tree(runner).unwrap().current();
                    modulus |= Uint::from(1u64); // Make sure modulus is odd
                    (base, exp, modulus)
                },
                |(base, exp, modulus)| {
                    black_box(base.pow_mod(exp, modulus))
                },
                BatchSize::SmallInput,
            )
        });

        // Optimized pow_mod_redc
        group.bench_function("pow_mod_redc", |bencher| {
            bencher.iter_batched(
                || {
                    let runner = &mut TestRunner::deterministic();
                    let base = Uint::<BITS, LIMBS>::arbitrary().new_tree(runner).unwrap().current();
                    let exp = Uint::<BITS, LIMBS>::arbitrary().new_tree(runner).unwrap().current();
                    let mut modulus = Uint::<BITS, LIMBS>::arbitrary().new_tree(runner).unwrap().current();
                    modulus |= Uint::from(1u64); // Make sure modulus is odd
                    let inv = U64::from(modulus.as_limbs()[0]).inv_ring().unwrap();
                    let inv = (-inv).as_limbs()[0];
                    (base, exp, modulus, inv)
                },
                |(base, exp, modulus, inv)| {
                    black_box(base.pow_mod_redc(exp, modulus, inv))
                },
                BatchSize::SmallInput,
            )
        });

        group.finish();
    });
}
