use crate::prelude::*;
use ruint::ToUintError;

pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);
        bench_from_f64::<BITS, LIMBS>(criterion, Uint::<BITS, LIMBS>::try_from);
        bench_from_f32::<BITS, LIMBS>(criterion, Uint::<BITS, LIMBS>::try_from);
        bench_to_f64::<BITS, LIMBS>(criterion, f64::from);
        bench_to_f32::<BITS, LIMBS>(criterion, f32::from);
    });
}

fn bench_from_f64<const BITS: usize, const LIMBS: usize>(
    criterion: &mut Criterion,
    f: impl FnMut(f64) -> Result<Uint<BITS, LIMBS>, ToUintError<Uint<BITS, LIMBS>>>,
) {
    bench_arbitrary_with(criterion, &format!("from/f64/{BITS}"), f64::arbitrary(), f);
}

fn bench_from_f32<const BITS: usize, const LIMBS: usize>(
    criterion: &mut Criterion,
    f: impl FnMut(f32) -> Result<Uint<BITS, LIMBS>, ToUintError<Uint<BITS, LIMBS>>>,
) {
    bench_arbitrary_with(criterion, &format!("from/f32/{BITS}"), f32::arbitrary(), f);
}

fn bench_to_f64<const BITS: usize, const LIMBS: usize>(
    criterion: &mut Criterion,
    f: impl FnMut(Uint<BITS, LIMBS>) -> f64,
) {
    bench_arbitrary_with(
        criterion,
        &format!("to/f64/{BITS}"),
        Uint::<BITS, LIMBS>::arbitrary(),
        f,
    );
}

fn bench_to_f32<const BITS: usize, const LIMBS: usize>(
    criterion: &mut Criterion,
    f: impl FnMut(Uint<BITS, LIMBS>) -> f32,
) {
    bench_arbitrary_with(
        criterion,
        &format!("to/f32/{BITS}"),
        Uint::<BITS, LIMBS>::arbitrary(),
        f,
    );
}
