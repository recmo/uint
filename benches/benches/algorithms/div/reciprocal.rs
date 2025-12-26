use crate::prelude::*;
use ruint::algorithms::div::*;

pub fn group(criterion: &mut Criterion) {
    bench_reciprocal_ref(criterion);
    bench_reciprocal_mg10(criterion);
    bench_reciprocal_2_mg10(criterion);
}

fn bench_reciprocal_ref(criterion: &mut Criterion) {
    bench_arbitrary_with(
        criterion,
        "algo/div/reciprocal/ref",
        u64::arbitrary().prop_map(|a| a | (1 << 63)),
        reciprocal_ref,
    );
}

fn bench_reciprocal_mg10(criterion: &mut Criterion) {
    bench_arbitrary_with(
        criterion,
        "algo/div/reciprocal/mg10",
        u64::arbitrary().prop_map(|a| a | (1 << 63)),
        |arg| unsafe { reciprocal_mg10(arg) },
    );
}

fn bench_reciprocal_2_mg10(criterion: &mut Criterion) {
    bench_arbitrary_with(
        criterion,
        "algo/div/reciprocal_2/mg10",
        u128::arbitrary().prop_map(|a| a | (1u128 << 127)),
        |arg| unsafe { reciprocal_2_mg10(arg) },
    );
}
