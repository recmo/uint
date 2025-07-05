use crate::prelude::*;
use std::fmt::Write;

pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);

        bench_fmt::<BITS, LIMBS, _>(criterion, &format!("fmt/binary/{BITS}"), |n, buf| {
            write!(buf, "{n:b}").unwrap()
        });
        bench_fmt::<BITS, LIMBS, _>(criterion, &format!("fmt/octal/{BITS}"), |n, buf| {
            write!(buf, "{n:o}").unwrap()
        });
        bench_fmt::<BITS, LIMBS, _>(criterion, &format!("fmt/decimal/{BITS}"), |n, buf| {
            write!(buf, "{n}").unwrap()
        });
        bench_fmt::<BITS, LIMBS, _>(criterion, &format!("fmt/hex/{BITS}"), |n, buf| {
            write!(buf, "{n:x}").unwrap()
        });
    });
}

fn bench_fmt<const BITS: usize, const LIMBS: usize, T>(
    criterion: &mut Criterion,
    name: &str,
    mut f: impl FnMut(Uint<BITS, LIMBS>, &mut String) -> T,
) {
    let mut buf = String::with_capacity(BITS);
    bench_unop(criterion, name, |n| {
        buf.clear();
        f(n, black_box(&mut buf))
    });
}
