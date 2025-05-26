use crate::prelude::*;
use ruint::algorithms::LehmerMatrix as Matrix;

pub fn group(criterion: &mut Criterion) {
    bench_from_u64(criterion);
    bench_from_u64_prefix(criterion);
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);
        bench_apply::<BITS, LIMBS>(criterion);
    });
}

fn bench_from_u64(criterion: &mut Criterion) {
    bench_arbitrary_with(
        criterion,
        "algorithms/gcd/matrix/from_u64",
        input::<u64>(),
        |(a, b)| Matrix::from_u64(a, b),
    );
}

fn bench_from_u64_prefix(criterion: &mut Criterion) {
    bench_arbitrary_with(
        criterion,
        "algorithms/gcd/matrix/from_u64_prefix",
        input::<u64>(),
        |(a, b)| Matrix::from_u64_prefix(a, b),
    );
}

fn bench_apply<const BITS: usize, const LIMBS: usize>(criterion: &mut Criterion) {
    bench_arbitrary_with(
        criterion,
        &format!("algorithms/gcd/matrix/apply/{BITS}"),
        input::<Uint<BITS, LIMBS>>().prop_map(|(a, b)| (Matrix::from(a, b), a, b)),
        |(m, mut a, mut b)| m.apply(&mut a, &mut b),
    );
}

fn input<T: Ord + Arbitrary>() -> impl Strategy<Value = (T, T)> {
    <(T, T)>::arbitrary().prop_map(|(a, b)| if a >= b { (a, b) } else { (b, a) })
}
