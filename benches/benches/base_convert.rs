use crate::prelude::*;
use std::collections::BTreeSet;

#[allow(clippy::single_element_loop)]
pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);
        for input_bits in BTreeSet::from([BITS]) {
            for base in [10] {
                bench_from_base::<BITS, LIMBS>(criterion, input_bits, base, false, |digits| {
                    Uint::<BITS, LIMBS>::from_base_le(base, digits.iter().copied()).unwrap()
                });
                bench_from_base::<BITS, LIMBS>(criterion, input_bits, base, true, |digits| {
                    Uint::<BITS, LIMBS>::from_base_be(base, digits.iter().copied()).unwrap()
                });

                bench_to_base::<BITS, LIMBS>(criterion, input_bits, base, false, |n, f| {
                    n.to_base_le(base).for_each(f);
                });
                bench_to_base::<BITS, LIMBS>(criterion, input_bits, base, true, |n, f| {
                    n.to_base_be(base).for_each(f);
                });
            }
        }
    });
}

fn bench_from_base<const BITS: usize, const LIMBS: usize>(
    criterion: &mut Criterion,
    input_bits: usize,
    base: u64,
    is_be: bool,
    mut f: impl FnMut(&[u64]) -> Uint<BITS, LIMBS>,
) {
    let endian = if is_be { "be" } else { "le" };
    bench_arbitrary_with(
        criterion,
        &format!("base_convert/{BITS}/{input_bits}/{base}/{endian}"),
        Uint::<BITS, LIMBS>::arbitrary().prop_map(|mut n| {
            n >>= BITS - input_bits;
            let digits: Vec<u64> = if is_be {
                n.to_base_be(base).collect()
            } else {
                n.to_base_le(base).collect()
            };
            arrayvec::ArrayVec::<u64, BITS>::from_iter(digits)
        }),
        |n| f(black_box(n.as_slice())),
    );
}

fn bench_to_base<const BITS: usize, const LIMBS: usize>(
    criterion: &mut Criterion,
    input_bits: usize,
    base: u64,
    is_be: bool,
    mut f: impl FnMut(Uint<BITS, LIMBS>, fn(u64)),
) {
    fn noop(_: u64) {}
    let noop = black_box(noop);

    let endian = if is_be { "be" } else { "le" };
    bench_arbitrary_with(
        criterion,
        &format!("base_convert/{BITS}/{input_bits}/{base}/{endian}"),
        Uint::<BITS, LIMBS>::arbitrary().prop_map(|n| n >> (BITS - input_bits)),
        |n| f(n, noop),
    );
}
