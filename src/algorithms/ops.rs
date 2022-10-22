use super::DoubleWord;

#[inline(always)]
pub fn adc(lhs: u64, rhs: u64, carry: u64) -> (u64, u64) {
    let result = u128::from(lhs) + u128::from(rhs) + u128::from(carry);
    result.split()
}

#[inline(always)]
pub fn sbb(lhs: u64, rhs: u64, borrow: u64) -> (u64, u64) {
    let result = u128::from(lhs)
        .wrapping_sub(u128::from(rhs))
        .wrapping_sub(u128::from(borrow));
    (result.low(), result.high().wrapping_neg())
}

//
