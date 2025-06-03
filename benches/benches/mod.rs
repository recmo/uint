mod add;
mod algorithms;
mod bits;
mod cmp;
mod div;
mod log;
mod modular;
mod mul;
mod pow;
mod pow_mod_redc;
mod root;

pub(crate) mod prelude;

pub fn group(c: &mut criterion::Criterion) {
    cmp::group(c);
    bits::group(c);
    add::group(c);
    mul::group(c);
    div::group(c);
    pow::group(c);
    log::group(c);
    root::group(c);
    modular::group(c);
    pow_mod_redc::group(c);
    algorithms::group(c);
}
