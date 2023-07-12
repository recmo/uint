use crate::prelude::*;
use ruint2::algorithms::div::*;

pub fn group(criterion: &mut Criterion) {
    bench_reciprocal_ref(criterion);
    bench_reciprocal_mg10(criterion);
    bench_reciprocal_2_mg10(criterion);
}

fn bench_reciprocal_ref(criterion: &mut Criterion) {
    let mut rng = rand::thread_rng();
    criterion.bench_function("algo/div/reciprocal/ref", move |bencher| {
        bencher.iter_batched(
            || rng.gen::<u64>() | (1 << 63),
            |a| black_box(reciprocal_ref(black_box(a))),
            BatchSize::SmallInput,
        );
    });
}

fn bench_reciprocal_mg10(criterion: &mut Criterion) {
    let mut rng = rand::thread_rng();
    criterion.bench_function("algo/div/reciprocal/mg10", move |bencher| {
        bencher.iter_batched(
            || rng.gen::<u64>() | (1 << 63),
            |a| black_box(reciprocal_mg10(black_box(a))),
            BatchSize::SmallInput,
        );
    });
}

fn bench_reciprocal_2_mg10(criterion: &mut Criterion) {
    let mut rng = rand::thread_rng();
    criterion.bench_function("algo/div/reciprocal_2/mg10", move |bencher| {
        bencher.iter_batched(
            || rng.gen::<u128>() | (1 << 127),
            |a| black_box(reciprocal_2_mg10(black_box(a))),
            BatchSize::SmallInput,
        );
    });
}
