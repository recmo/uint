use crate::prelude::*;
use ruint::algorithms::addmul_n;

pub fn group(criterion: &mut Criterion) {
    bench_addmul_nnn(criterion);
}

fn bench_addmul_nnn(criterion: &mut Criterion) {
    const_for!(SIZE in [1,2,3,4,5,6] {
        bench_arbitrary::<([u64; SIZE], [u64; SIZE], [u64; SIZE]), _>(
            criterion,
            &format!("algo/addmul_n/{SIZE}"),
            |(mut lhs, a, b)| {
                addmul_n(&mut lhs, &a, &b);
                lhs
            },
        );
    });
}
