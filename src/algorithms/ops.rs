use super::ConstDoubleWord as DW;

#[inline(always)]
#[must_use]
pub const fn adc(lhs: u64, rhs: u64, carry: u64) -> (u64, u64) {
    let result = DW::ext(lhs) + DW::ext(rhs) + DW::ext(carry);
    DW(result).split()
}

#[inline(always)]
#[must_use]
pub const fn sbb(lhs: u64, rhs: u64, borrow: u64) -> (u64, u64) {
    let result = DW::ext(lhs)
        .wrapping_sub(DW::ext(rhs))
        .wrapping_sub(DW::ext(borrow));
    (DW(result).low(), DW(result).high().wrapping_neg())
}
