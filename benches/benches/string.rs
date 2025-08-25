use crate::prelude::*;

pub fn group(criterion: &mut Criterion) {
    const_for!(BITS in BENCH {
        const LIMBS: usize = nlimbs(BITS);

        bench_parse::<BITS, LIMBS,  2>(criterion, "parse/binary");
        bench_parse::<BITS, LIMBS,  8>(criterion, "parse/octal");
        bench_parse::<BITS, LIMBS, 10>(criterion, "parse/decimal");
        bench_parse::<BITS, LIMBS, 16>(criterion, "parse/hex");
    });
}

fn bench_parse<const BITS: usize, const LIMBS: usize, const RADIX: u64>(
    criterion: &mut Criterion,
    name: &str,
) {
    let name = &format!("{name}/{BITS}");
    let max = match RADIX {
        2 => format!("{:b}", Uint::<BITS, LIMBS>::MAX),
        8 => format!("{:o}", Uint::<BITS, LIMBS>::MAX),
        10 => format!("{}", Uint::<BITS, LIMBS>::MAX),
        16 => format!("{:x}", Uint::<BITS, LIMBS>::MAX),
        _ => unreachable!(),
    };
    for (subname, s) in [("zero", "0"), ("max", &max)] {
        let s = black_box(s);
        let name = &format!("{name}/{subname}");
        bench_arbitrary::<(), _>(criterion, name, |()| {
            Uint::<BITS, LIMBS>::from_str_radix(black_box(s), RADIX).unwrap()
        });
    }
}
