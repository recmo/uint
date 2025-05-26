mod add;
mod algorithms;
mod cmp;
mod div;
mod log;
mod modular;
mod mul;
mod pow;
mod root;

pub(crate) mod prelude;

pub fn group(c: &mut criterion::Criterion) {
    cmp::group(c);
    add::group(c);
    mul::group(c);
    div::group(c);
    pow::group(c);
    log::group(c);
    root::group(c);
    modular::group(c);
    algorithms::group(c);
}
