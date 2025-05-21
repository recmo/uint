use crate::prelude::*;

pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);
        bench_is_zero::<BITS, LIMBS>(criterion);
        bench_eq::<BITS, LIMBS>(criterion);
        bench_cmp::<BITS, LIMBS>(criterion);
    });
}

fn bench_is_zero<const BITS: usize, const LIMBS: usize>(criterion: &mut Criterion) {
    let input = Uint::<BITS, LIMBS>::arbitrary();
    let mut runner = TestRunner::deterministic();
    criterion.bench_function(&format!("is_zero/{BITS}"), move |bencher| {
        bencher.iter_batched(
            || input.new_tree(&mut runner).unwrap().current(),
            |a| black_box(black_box(a).is_zero()),
            BatchSize::SmallInput,
        );
    });
}

fn bench_eq<const BITS: usize, const LIMBS: usize>(criterion: &mut Criterion) {
    let input = (Uint::<BITS, LIMBS>::arbitrary(), Uint::arbitrary());
    let mut runner = TestRunner::deterministic();
    criterion.bench_function(&format!("eq/{BITS}"), move |bencher| {
        bencher.iter_batched(
            || input.new_tree(&mut runner).unwrap().current(),
            |(a, b)| black_box(black_box(a) == black_box(b)),
            BatchSize::SmallInput,
        );
    });
}

fn bench_cmp<const BITS: usize, const LIMBS: usize>(criterion: &mut Criterion) {
    let input = (Uint::<BITS, LIMBS>::arbitrary(), Uint::arbitrary());
    let mut runner = TestRunner::deterministic();
    criterion.bench_function(&format!("cmp/{BITS}"), move |bencher| {
        bencher.iter_batched(
            || input.new_tree(&mut runner).unwrap().current(),
            |(a, b)| black_box(black_box(a).cmp(black_box(&b))),
            BatchSize::SmallInput,
        );
    });
}
