use crate::prelude::*;
use std::fmt::Write;

pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);

        bench_fmt::<BITS, LIMBS, _>(criterion, "fmt/binary", |n, buf| {
            write!(buf, "{n:b}").unwrap()
        });
        bench_fmt::<BITS, LIMBS, _>(criterion, "fmt/octal", |n, buf| {
            write!(buf, "{n:o}").unwrap()
        });
        bench_fmt::<BITS, LIMBS, _>(criterion, "fmt/decimal", |n, buf| {
            write!(buf, "{n}").unwrap()
        });
        bench_fmt::<BITS, LIMBS, _>(criterion, "fmt/hex", |n, buf| {
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
