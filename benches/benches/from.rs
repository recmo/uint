use crate::prelude::*;
use ruint::ToUintError;

pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);
        bench_with_f64::<BITS, LIMBS>(criterion, Uint::<BITS, LIMBS>::try_from);
        bench_with_f32::<BITS, LIMBS>(criterion, Uint::<BITS, LIMBS>::try_from);
    });
}

fn bench_with_f64<const BITS: usize, const LIMBS: usize>(
    criterion: &mut Criterion,
    mut f: impl FnMut(f64) -> Result<Uint<BITS, LIMBS>, ToUintError<Uint<BITS, LIMBS>>>,
) {
    bench_arbitrary_with(
        criterion,
        &format!("from/f64/{BITS}"),
        f64::arbitrary(),
        |x| {
            black_box(f(black_box(x)));
        },
    );
}

fn bench_with_f32<const BITS: usize, const LIMBS: usize>(
    criterion: &mut Criterion,
    mut f: impl FnMut(f32) -> Result<Uint<BITS, LIMBS>, ToUintError<Uint<BITS, LIMBS>>>,
) {
    bench_arbitrary_with(
        criterion,
        &format!("from/f32/{BITS}"),
        f32::arbitrary(),
        |x| {
            black_box(f(black_box(x)));
        },
    );
}
