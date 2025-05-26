#![allow(unused_imports)]
#![allow(clippy::incompatible_msrv)]

pub use criterion::{BatchSize, Criterion};
pub use proptest::{
    arbitrary::Arbitrary,
    strategy::{Strategy, ValueTree},
    test_runner::TestRunner,
};
pub use rand_09::{self as rand, prelude::*, rng};
pub use ruint::{const_for, nlimbs, uint, Bits, Uint, UintTryFrom, UintTryTo};
pub use std::hint::black_box;

pub fn bench_unop<const BITS: usize, const LIMBS: usize, U>(
    criterion: &mut criterion::Criterion,
    name: &str,
    f: impl FnMut(Uint<BITS, LIMBS>) -> U,
) {
    bench_arbitrary::<Uint<BITS, LIMBS>, U>(criterion, &format!("{name}/{BITS}"), f);
}

pub fn bench_binop<const BITS: usize, const LIMBS: usize, U>(
    criterion: &mut criterion::Criterion,
    name: &str,
    mut f: impl FnMut(Uint<BITS, LIMBS>, Uint<BITS, LIMBS>) -> U,
) {
    bench_arbitrary::<(Uint<BITS, LIMBS>, Uint<BITS, LIMBS>), U>(
        criterion,
        &format!("{name}/{BITS}"),
        move |(a, b)| f(a, b),
    );
}

pub fn bench_ternary<const BITS: usize, const LIMBS: usize, U>(
    criterion: &mut criterion::Criterion,
    name: &str,
    mut f: impl FnMut(Uint<BITS, LIMBS>, Uint<BITS, LIMBS>, Uint<BITS, LIMBS>) -> U,
) {
    bench_arbitrary::<(Uint<BITS, LIMBS>, Uint<BITS, LIMBS>, Uint<BITS, LIMBS>), U>(
        criterion,
        &format!("{name}/{BITS}"),
        move |(a, b, c)| f(a, b, c),
    );
}

pub fn bench_arbitrary<T: Arbitrary, U>(
    criterion: &mut criterion::Criterion,
    name: &str,
    f: impl FnMut(<T::Strategy as proptest::strategy::Strategy>::Value) -> U,
) {
    bench_arbitrary_with(criterion, name, T::arbitrary(), f)
}

pub fn bench_arbitrary_with<T: Strategy, U>(
    criterion: &mut criterion::Criterion,
    name: &str,
    input: T,
    mut f: impl FnMut(T::Value) -> U,
) {
    let mut runner = TestRunner::deterministic();
    let mut setup = mk_setup(&input, &mut runner);
    criterion.bench_function(name, move |bencher| {
        bencher.iter_batched(
            &mut setup,
            |input| black_box(f(black_box(input))),
            BatchSize::SmallInput,
        );
    });
}

fn mk_setup<'a, T: Strategy>(
    input: &'a T,
    runner: &'a mut TestRunner,
) -> impl FnMut() -> T::Value + 'a {
    move || input.new_tree(runner).unwrap().current()
}
