use crate::prelude::*;
use ruint::algorithms::div::*;

pub fn group(criterion: &mut Criterion) {
    bench_div_2x1_ref(criterion);
    bench_div_2x1_mg10(criterion);
    bench_div_3x2_ref(criterion);
    bench_div_3x2_mg10(criterion);
}

fn input_2x1() -> impl Strategy<Value = (u128, u64)> {
    (u64::arbitrary(), u64::arbitrary(), u64::arbitrary()).prop_map(|(q, r, d)| {
        let d = d | (1 << 63);
        let r = r % d;
        let n = u128::from(q) * u128::from(d) + u128::from(r);
        (n, d)
    })
}

fn input_3x2() -> impl Strategy<Value = (u128, u64, u128)> {
    (u64::arbitrary(), u128::arbitrary(), u128::arbitrary()).prop_map(|(q, r, d)| {
        let d = d | (1 << 127);
        let r = r % d;
        let (n21, n0) = {
            let d1 = (d >> 64) as u64;
            let d0 = d as u64;
            let r1 = (r >> 64) as u64;
            let r0 = r as u64;
            // n = q * d + r
            let n10 = u128::from(q) * u128::from(d0) + u128::from(r0);
            let n0 = n10 as u64;
            let n21 = (n10 >> 64) + u128::from(q) * u128::from(d1) + u128::from(r1);
            (n21, n0)
        };
        (n21, n0, d)
    })
}

fn bench_div_2x1_ref(criterion: &mut Criterion) {
    bench_arbitrary_with(criterion, "algo/div/2x1/ref", input_2x1(), |(n, d)| {
        div_2x1_ref(n, d)
    });
}

fn bench_div_2x1_mg10(criterion: &mut Criterion) {
    bench_arbitrary_with(
        criterion,
        "algo/div/2x1/mg10",
        input_2x1().prop_map(|(n, d)| (n, d, unsafe { reciprocal(d) })),
        |(n, d, v)| div_2x1_mg10(n, d, v),
    );
}

fn bench_div_3x2_ref(criterion: &mut Criterion) {
    bench_arbitrary_with(
        criterion,
        "algo/div/3x2/ref",
        input_3x2(),
        |(n21, n0, d)| div_3x2_ref(n21, n0, d),
    );
}

fn bench_div_3x2_mg10(criterion: &mut Criterion) {
    bench_arbitrary_with(
        criterion,
        "algo/div/3x2/mg10",
        input_3x2().prop_map(|(n21, n0, d)| (n21, n0, d, unsafe { reciprocal_2(d) })),
        |(n21, n0, d, v)| unsafe { div_3x2_mg10(n21, n0, d, v) },
    );
}
