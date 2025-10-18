mod div;
mod gcd;
mod mul;
mod shift;

pub fn group(criterion: &mut criterion::Criterion) {
    self::mul::group(criterion);
    self::div::group(criterion);
    self::gcd::group(criterion);
    self::shift::group(criterion);
}
