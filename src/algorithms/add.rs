#![allow(dead_code)] // TODO

use super::ops::{adc, sbb};

/// `lhs += rhs + carry`
#[inline(always)]
pub fn adc_n(lhs: &mut [u64], rhs: &[u64], mut carry: u64) -> u64 {
    for (l, r) in lhs.iter_mut().zip(rhs.iter()) {
        let (result, new_carry) = adc(*l, *r, carry);
        *l = result;
        carry = new_carry;
    }
    carry
}

/// `lhs -= rhs + carry`
#[inline(always)]
pub fn sbb_n(lhs: &mut [u64], rhs: &[u64], mut carry: u64) -> u64 {
    for (l, r) in lhs.iter_mut().zip(rhs.iter()) {
        let (result, new_carry) = sbb(*l, *r, carry);
        *l = result;
        carry = new_carry;
    }
    carry
}
