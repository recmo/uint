mod add;
mod algorithms;
mod base_convert;
mod bits;
mod cmp;
mod div;
mod fmt;
mod log;
mod modular;
mod mul;
mod pow;
mod root;

pub(crate) mod prelude;

pub fn group(c: &mut criterion::Criterion) {
    bits::group(c);

    add::group(c);
    mul::group(c);
    div::group(c);
    pow::group(c);
    log::group(c);
    root::group(c);
    modular::group(c);

    cmp::group(c);

    base_convert::group(c);
    fmt::group(c);

    algorithms::group(c);
}
